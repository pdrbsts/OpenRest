//! Fila de impressão assíncrona, por dispositivo.
//!
//! Imprimir não deve bloquear o POS: [`PrintSpooler::enqueue`] devolve de
//! imediato e um worker dedicado por dispositivo entrega os jobs em série (uma
//! impressora nunca é escrita concorrentemente), com retry/backoff. O estado
//! por dispositivo (saúde, fila, último erro, jobs concluídos) fica disponível
//! para telemetria (spec §7.6).

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{mpsc, Mutex, RwLock};

use crate::transport::Connection;
use crate::PrinterError;

/// Política de re-tentativa com backoff linear (`base_delay * tentativa`).
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub base_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(500),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DeviceHealth {
    #[default]
    Unknown,
    Ok,
    Failed,
}

#[derive(Clone, Debug, Default)]
pub struct DeviceStatus {
    pub health: DeviceHealth,
    pub queued: usize,
    pub last_error: Option<String>,
    pub jobs_done: u64,
}

struct Job {
    connection: Connection,
    bytes: Vec<u8>,
    copies: u8,
}

type StatusMap = Arc<RwLock<HashMap<String, DeviceStatus>>>;

/// Distribuidor de jobs de impressão. Clonável (partilha o estado interno).
#[derive(Clone)]
pub struct PrintSpooler {
    senders: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<Job>>>>,
    statuses: StatusMap,
    retry: RetryPolicy,
}

impl PrintSpooler {
    pub fn new(retry: RetryPolicy) -> Self {
        Self {
            senders: Arc::new(Mutex::new(HashMap::new())),
            statuses: Arc::new(RwLock::new(HashMap::new())),
            retry,
        }
    }

    /// Coloca um job na fila do dispositivo (não bloqueia na impressão).
    pub async fn enqueue(&self, device_id: &str, connection: Connection, bytes: Vec<u8>, copies: u8) {
        let mut senders = self.senders.lock().await;
        let tx = senders.entry(device_id.to_string()).or_insert_with(|| {
            let (tx, rx) = mpsc::unbounded_channel::<Job>();
            spawn_worker(device_id.to_string(), rx, self.statuses.clone(), self.retry.clone());
            tx
        });
        {
            let mut st = self.statuses.write().await;
            st.entry(device_id.to_string()).or_default().queued += 1;
        }
        // O envio só falha se o worker tiver morrido; nesse caso reflicte no estado.
        if tx.send(Job { connection, bytes, copies }).is_err() {
            let mut st = self.statuses.write().await;
            let e = st.entry(device_id.to_string()).or_default();
            e.queued = e.queued.saturating_sub(1);
            e.health = DeviceHealth::Failed;
            e.last_error = Some("worker indisponível".into());
        }
    }

    pub async fn status(&self, device_id: &str) -> Option<DeviceStatus> {
        self.statuses.read().await.get(device_id).cloned()
    }

    pub async fn snapshot(&self) -> HashMap<String, DeviceStatus> {
        self.statuses.read().await.clone()
    }
}

fn spawn_worker(
    device_id: String,
    mut rx: mpsc::UnboundedReceiver<Job>,
    statuses: StatusMap,
    retry: RetryPolicy,
) {
    tokio::spawn(async move {
        while let Some(job) = rx.recv().await {
            let mut last_err: Option<PrinterError> = None;
            for _ in 0..job.copies.max(1) {
                let conn = job.connection.clone();
                let bytes = job.bytes.clone();
                let (_, res) = deliver(|| conn.send(&bytes), &retry).await;
                if let Err(e) = res {
                    last_err = Some(e);
                    break;
                }
            }
            let mut st = statuses.write().await;
            let entry = st.entry(device_id.clone()).or_default();
            entry.queued = entry.queued.saturating_sub(1);
            match last_err {
                None => {
                    entry.health = DeviceHealth::Ok;
                    entry.last_error = None;
                    entry.jobs_done += 1;
                }
                Some(e) => {
                    entry.health = DeviceHealth::Failed;
                    entry.last_error = Some(e.to_string());
                }
            }
        }
    });
}

/// Tenta entregar com retry/backoff. Devolve `(tentativas, resultado)`.
/// Extraída para ser testável sem I/O real.
pub async fn deliver<F, Fut>(mut attempt_fn: F, policy: &RetryPolicy) -> (u32, Result<(), PrinterError>)
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), PrinterError>>,
{
    let mut last = Err(PrinterError::Connection("sem tentativas".into()));
    for attempt in 1..=policy.max_attempts.max(1) {
        match attempt_fn().await {
            Ok(()) => return (attempt, Ok(())),
            Err(e) => {
                last = Err(e);
                if attempt < policy.max_attempts {
                    tokio::time::sleep(policy.base_delay * attempt).await;
                }
            }
        }
    }
    (policy.max_attempts.max(1), last)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn fast_policy(max: u32) -> RetryPolicy {
        RetryPolicy {
            max_attempts: max,
            base_delay: Duration::from_millis(1),
        }
    }

    #[tokio::test]
    async fn deliver_succeeds_after_retries() {
        let calls = AtomicU32::new(0);
        let (attempts, res) = deliver(
            || {
                let n = calls.fetch_add(1, Ordering::SeqCst) + 1;
                async move {
                    if n < 3 {
                        Err(PrinterError::Timeout)
                    } else {
                        Ok(())
                    }
                }
            },
            &fast_policy(5),
        )
        .await;
        assert_eq!(attempts, 3);
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn deliver_gives_up_after_max() {
        let calls = AtomicU32::new(0);
        let (attempts, res) = deliver(
            || {
                calls.fetch_add(1, Ordering::SeqCst);
                async { Err(PrinterError::Timeout) }
            },
            &fast_policy(3),
        )
        .await;
        assert_eq!(attempts, 3);
        assert!(res.is_err());
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn enqueue_to_file_updates_status() {
        let path = std::env::temp_dir().join(format!(
            "openrest_spool_{}.bin",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let spooler = PrintSpooler::new(fast_policy(3));
        spooler
            .enqueue("dev1", Connection::File { path: path.clone() }, b"hello".to_vec(), 2)
            .await;

        // Espera a conclusão (timeout defensivo).
        let mut done = false;
        for _ in 0..200 {
            if let Some(s) = spooler.status("dev1").await {
                if s.jobs_done == 1 && s.queued == 0 {
                    assert_eq!(s.health, DeviceHealth::Ok);
                    done = true;
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
        assert!(done, "job não concluiu a tempo");
        // 2 cópias → 2x "hello".
        let content = std::fs::read(&path).unwrap();
        assert_eq!(content, b"hellohello");
        let _ = std::fs::remove_file(&path);
    }
}
