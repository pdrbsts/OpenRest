//! Transporte de impressão — "como chegam os bytes ao hardware".
//!
//! Abstrai os destinos físicos num único enum [`Connection`], construído a
//! partir da configuração guardada no dispositivo (`conexao_tipo` +
//! `conexao_config` JSON). Cobre os dois mundos:
//!
//! - **Impressoras instaladas no Windows** → [`Connection::WindowsSpooler`],
//!   que envia bytes RAW ao spooler (ESC/POS passa intacto).
//! - **Acesso directo** → [`Connection::Tcp`] (IP, porta 9100 RAW/JetDirect) e
//!   [`Connection::Serial`] (porta COM).
//!
//! Mais [`Connection::File`] (mock/ecrã/testes) e [`Connection::Null`] (descarta).

use std::path::PathBuf;
use std::time::Duration;

use serde::Deserialize;
use tokio::io::AsyncWriteExt;

use crate::PrinterError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Parity {
    None,
    Odd,
    Even,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FlowControl {
    None,
    Software,
    Hardware,
}

/// Estado reportado por um dispositivo. Em transportes sem canal de retorno
/// (file/null/spooler) fica [`PrinterStatus::Unknown`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrinterStatus {
    Online,
    Offline,
    Unknown,
}

/// Destino físico de uma impressão.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Connection {
    /// Acrescenta os bytes a um ficheiro (mock / ecrã / testes).
    File { path: PathBuf },
    /// Descarta tudo (porta nula da spec — desactiva impressões).
    Null,
    /// Cliente TCP RAW (impressoras IP; porta 9100 por defeito).
    Tcp {
        host: String,
        port: u16,
        timeout_ms: u64,
    },
    /// Porta série (COM).
    Serial {
        port: String,
        baud: u32,
        data_bits: u8,
        parity: Parity,
        stop_bits: u8,
        flow: FlowControl,
    },
    /// Fila de impressão do Windows (envio RAW). Só Windows.
    WindowsSpooler {
        printer_name: String,
        data_type: String,
    },
}

// --- Parsing a partir da configuração do dispositivo ---------------------

fn default_tcp_port() -> u16 {
    9100
}
fn default_timeout_ms() -> u64 {
    3000
}
fn default_baud() -> u32 {
    9600
}
fn default_data_bits() -> u8 {
    8
}
fn default_stop_bits() -> u8 {
    1
}
fn default_parity() -> Parity {
    Parity::None
}
fn default_flow() -> FlowControl {
    FlowControl::None
}
fn default_data_type() -> String {
    "RAW".to_string()
}

fn parse_cfg<T: serde::de::DeserializeOwned>(
    config: &serde_json::Value,
) -> Result<T, PrinterError> {
    serde_json::from_value(config.clone()).map_err(|e| PrinterError::Config(e.to_string()))
}

#[derive(Deserialize)]
struct FileCfg {
    path: String,
}

#[derive(Deserialize)]
struct TcpCfg {
    host: String,
    #[serde(default = "default_tcp_port")]
    port: u16,
    #[serde(default = "default_timeout_ms")]
    timeout_ms: u64,
}

#[derive(Deserialize)]
struct SerialCfg {
    port: String,
    #[serde(default = "default_baud")]
    baud: u32,
    #[serde(default = "default_data_bits")]
    data_bits: u8,
    #[serde(default = "default_parity")]
    parity: Parity,
    #[serde(default = "default_stop_bits")]
    stop_bits: u8,
    #[serde(default = "default_flow")]
    flow: FlowControl,
}

#[derive(Deserialize)]
struct SpoolerCfg {
    printer_name: String,
    #[serde(default = "default_data_type")]
    data_type: String,
}

impl Connection {
    /// Constrói a partir do `conexao_tipo` e do `conexao_config` (JSON) do
    /// dispositivo. `config` pode ser `Null` para tipos sem parâmetros.
    pub fn from_config(
        conexao_tipo: &str,
        config: &serde_json::Value,
    ) -> Result<Connection, PrinterError> {
        Ok(match conexao_tipo {
            "file" => {
                let c: FileCfg = parse_cfg(config)?;
                Connection::File { path: PathBuf::from(c.path) }
            }
            "null" => Connection::Null,
            "tcp" => {
                let c: TcpCfg = parse_cfg(config)?;
                Connection::Tcp {
                    host: c.host,
                    port: c.port,
                    timeout_ms: c.timeout_ms,
                }
            }
            "serial" => {
                let c: SerialCfg = parse_cfg(config)?;
                Connection::Serial {
                    port: c.port,
                    baud: c.baud,
                    data_bits: c.data_bits,
                    parity: c.parity,
                    stop_bits: c.stop_bits,
                    flow: c.flow,
                }
            }
            "windows_spooler" => {
                let c: SpoolerCfg = parse_cfg(config)?;
                Connection::WindowsSpooler {
                    printer_name: c.printer_name,
                    data_type: c.data_type,
                }
            }
            other => {
                return Err(PrinterError::Config(format!(
                    "conexao_tipo desconhecido: {other}"
                )))
            }
        })
    }

    /// Envia os bytes ao dispositivo. Cada chamada é uma operação completa
    /// (abre, escreve, fecha) — o estado fica do lado do hardware.
    pub async fn send(&self, bytes: &[u8]) -> Result<(), PrinterError> {
        match self {
            Connection::Null => Ok(()),
            Connection::File { path } => send_file(path, bytes).await,
            Connection::Tcp {
                host,
                port,
                timeout_ms,
            } => send_tcp(host, *port, *timeout_ms, bytes).await,
            Connection::Serial {
                port,
                baud,
                data_bits,
                parity,
                stop_bits,
                flow,
            } => send_serial(port, *baud, *data_bits, *parity, *stop_bits, *flow, bytes).await,
            Connection::WindowsSpooler {
                printer_name,
                data_type,
            } => send_windows_spooler(printer_name, data_type, bytes).await,
        }
    }

    /// Verificação de disponibilidade. TCP confirma a ligação; serial confirma
    /// que a porta abre; restantes ficam `Unknown`.
    pub async fn status(&self) -> PrinterStatus {
        match self {
            Connection::Null | Connection::File { .. } => PrinterStatus::Online,
            Connection::Tcp {
                host,
                port,
                timeout_ms,
            } => {
                let addr = format!("{host}:{port}");
                match tokio::time::timeout(
                    Duration::from_millis(*timeout_ms),
                    tokio::net::TcpStream::connect(&addr),
                )
                .await
                {
                    Ok(Ok(_)) => PrinterStatus::Online,
                    _ => PrinterStatus::Offline,
                }
            }
            _ => PrinterStatus::Unknown,
        }
    }
}

async fn send_file(path: &PathBuf, bytes: &[u8]) -> Result<(), PrinterError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            tokio::fs::create_dir_all(parent).await.ok();
        }
    }
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await?;
    file.write_all(bytes).await?;
    file.flush().await?;
    Ok(())
}

async fn send_tcp(host: &str, port: u16, timeout_ms: u64, bytes: &[u8]) -> Result<(), PrinterError> {
    let addr = format!("{host}:{port}");
    let connect = tokio::net::TcpStream::connect(&addr);
    let mut stream = tokio::time::timeout(Duration::from_millis(timeout_ms), connect)
        .await
        .map_err(|_| PrinterError::Timeout)?
        .map_err(|e| PrinterError::Connection(format!("{addr}: {e}")))?;
    stream.write_all(bytes).await?;
    stream.flush().await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn send_serial(
    port: &str,
    baud: u32,
    data_bits: u8,
    parity: Parity,
    stop_bits: u8,
    flow: FlowControl,
    bytes: &[u8],
) -> Result<(), PrinterError> {
    let port = port.to_string();
    let data = bytes.to_vec();
    // serialport é bloqueante: corre fora do executor async.
    tokio::task::spawn_blocking(move || {
        use std::io::Write;
        let mut sp = serialport::new(&port, baud)
            .data_bits(match data_bits {
                5 => serialport::DataBits::Five,
                6 => serialport::DataBits::Six,
                7 => serialport::DataBits::Seven,
                _ => serialport::DataBits::Eight,
            })
            .parity(match parity {
                Parity::None => serialport::Parity::None,
                Parity::Odd => serialport::Parity::Odd,
                Parity::Even => serialport::Parity::Even,
            })
            .stop_bits(match stop_bits {
                2 => serialport::StopBits::Two,
                _ => serialport::StopBits::One,
            })
            .flow_control(match flow {
                FlowControl::None => serialport::FlowControl::None,
                FlowControl::Software => serialport::FlowControl::Software,
                FlowControl::Hardware => serialport::FlowControl::Hardware,
            })
            .timeout(Duration::from_millis(3000))
            .open()
            .map_err(|e| PrinterError::Connection(format!("{port}: {e}")))?;
        sp.write_all(&data)?;
        sp.flush()?;
        Ok::<(), PrinterError>(())
    })
    .await
    .map_err(|e| PrinterError::Connection(e.to_string()))?
}

#[cfg(windows)]
async fn send_windows_spooler(
    printer_name: &str,
    data_type: &str,
    bytes: &[u8],
) -> Result<(), PrinterError> {
    let printer_name = printer_name.to_string();
    let data_type = data_type.to_string();
    let data = bytes.to_vec();
    tokio::task::spawn_blocking(move || winspool::print_raw(&printer_name, &data_type, &data))
        .await
        .map_err(|e| PrinterError::Connection(e.to_string()))?
}

#[cfg(not(windows))]
async fn send_windows_spooler(
    _printer_name: &str,
    _data_type: &str,
    _bytes: &[u8],
) -> Result<(), PrinterError> {
    Err(PrinterError::Unsupported(
        "windows_spooler só está disponível no Windows".into(),
    ))
}

/// FFI mínimo para o spooler do Windows (winspool). Envia um job RAW.
#[cfg(windows)]
mod winspool {
    use std::os::raw::c_void;

    use crate::PrinterError;

    type Handle = *mut c_void;
    type Bool = i32;

    #[repr(C)]
    struct DocInfo1W {
        p_doc_name: *const u16,
        p_output_file: *const u16,
        p_datatype: *const u16,
    }

    #[link(name = "winspool")]
    extern "system" {
        fn OpenPrinterW(p_printer_name: *const u16, ph_printer: *mut Handle, p_default: *mut c_void) -> Bool;
        fn ClosePrinter(h_printer: Handle) -> Bool;
        fn StartDocPrinterW(h_printer: Handle, level: u32, p_doc_info: *const DocInfo1W) -> u32;
        fn EndDocPrinter(h_printer: Handle) -> Bool;
        fn StartPagePrinter(h_printer: Handle) -> Bool;
        fn EndPagePrinter(h_printer: Handle) -> Bool;
        fn WritePrinter(h_printer: Handle, p_buf: *const c_void, cb_buf: u32, pc_written: *mut u32) -> Bool;
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn print_raw(printer_name: &str, data_type: &str, bytes: &[u8]) -> Result<(), PrinterError> {
        let name = wide(printer_name);
        let mut doc_name = wide("OpenRest");
        let mut datatype = wide(data_type);
        unsafe {
            let mut hprinter: Handle = std::ptr::null_mut();
            if OpenPrinterW(name.as_ptr(), &mut hprinter, std::ptr::null_mut()) == 0 {
                return Err(PrinterError::Connection(format!(
                    "OpenPrinter falhou para '{printer_name}'"
                )));
            }
            // Garante o fecho do handle mesmo em caso de erro.
            let result = (|| {
                let doc = DocInfo1W {
                    p_doc_name: doc_name.as_mut_ptr(),
                    p_output_file: std::ptr::null(),
                    p_datatype: datatype.as_mut_ptr(),
                };
                if StartDocPrinterW(hprinter, 1, &doc) == 0 {
                    return Err(PrinterError::Connection("StartDocPrinter falhou".into()));
                }
                if StartPagePrinter(hprinter) == 0 {
                    return Err(PrinterError::Connection("StartPagePrinter falhou".into()));
                }
                let mut written: u32 = 0;
                if WritePrinter(
                    hprinter,
                    bytes.as_ptr() as *const c_void,
                    bytes.len() as u32,
                    &mut written,
                ) == 0
                    || written as usize != bytes.len()
                {
                    return Err(PrinterError::Connection("WritePrinter incompleto".into()));
                }
                let _ = EndPagePrinter(hprinter);
                let _ = EndDocPrinter(hprinter);
                Ok(())
            })();
            ClosePrinter(hprinter);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tcp_defaults() {
        let c = Connection::from_config("tcp", &serde_json::json!({ "host": "192.168.1.50" }))
            .unwrap();
        assert_eq!(
            c,
            Connection::Tcp {
                host: "192.168.1.50".into(),
                port: 9100,
                timeout_ms: 3000,
            }
        );
    }

    #[test]
    fn parse_serial_full() {
        let c = Connection::from_config(
            "serial",
            &serde_json::json!({
                "port": "COM3", "baud": 19200, "parity": "even", "flow": "hardware"
            }),
        )
        .unwrap();
        assert_eq!(
            c,
            Connection::Serial {
                port: "COM3".into(),
                baud: 19200,
                data_bits: 8,
                parity: Parity::Even,
                stop_bits: 1,
                flow: FlowControl::Hardware,
            }
        );
    }

    #[test]
    fn parse_windows_spooler() {
        let c = Connection::from_config(
            "windows_spooler",
            &serde_json::json!({ "printer_name": "EPSON TM-T20II" }),
        )
        .unwrap();
        assert_eq!(
            c,
            Connection::WindowsSpooler {
                printer_name: "EPSON TM-T20II".into(),
                data_type: "RAW".into(),
            }
        );
    }

    #[test]
    fn parse_null_and_unknown() {
        assert_eq!(
            Connection::from_config("null", &serde_json::Value::Null).unwrap(),
            Connection::Null
        );
        assert!(Connection::from_config("carrier_pigeon", &serde_json::Value::Null).is_err());
    }

    #[tokio::test]
    async fn null_discards() {
        assert!(Connection::Null.send(b"qualquer coisa").await.is_ok());
    }

    #[tokio::test]
    async fn file_appends_bytes() {
        let path = std::env::temp_dir().join(format!("openrest_tx_{}.bin", uuid_like()));
        let conn = Connection::File { path: path.clone() };
        conn.send(b"AAA").await.unwrap();
        conn.send(b"BBB").await.unwrap();
        let content = std::fs::read(&path).unwrap();
        assert_eq!(content, b"AAABBB");
        let _ = std::fs::remove_file(&path);
    }

    fn uuid_like() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    }
}
