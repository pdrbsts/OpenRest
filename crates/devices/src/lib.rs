use std::path::PathBuf;
use thiserror::Error;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub mod escpos;

#[derive(Error, Debug)]
pub enum PrinterError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

/// Phase 1 generic printer: appends formatted receipts to a file so the flow
/// can be exercised without a physical ESC/POS device wired up.
pub struct GenericPrinter {
    output_path: PathBuf,
}

impl GenericPrinter {
    pub fn new(output_path: PathBuf) -> Self {
        Self { output_path }
    }

    pub async fn print_receipt(&self, text: &str) -> Result<(), PrinterError> {
        if let Some(parent) = self.output_path.parent() {
            if !parent.as_os_str().is_empty() {
                tokio::fs::create_dir_all(parent).await.ok();
            }
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_path)
            .await?;

        file.write_all(b"--- BEGIN RECEIPT ---\n").await?;
        file.write_all(text.as_bytes()).await?;
        file.write_all(b"\n--- END RECEIPT ---\n\n").await?;
        file.flush().await?;
        Ok(())
    }
}
