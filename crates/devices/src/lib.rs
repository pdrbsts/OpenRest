use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrinterError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct GenericPrinter {
    output_path: PathBuf,
}

impl GenericPrinter {
    pub fn new(output_path: PathBuf) -> Self {
        Self { output_path }
    }

    /// Mock printing by writing the receipt text to a file
    pub async fn print_receipt(&self, text: &str) -> Result<(), PrinterError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.output_path)
            .await?;
            
        file.write_all(b"--- BEGIN RECEIPT ---\n").await?;
        file.write_all(text.as_bytes()).await?;
        file.write_all(b"\n--- END RECEIPT ---\n\n").await?;
        
        Ok(())
    }
}
