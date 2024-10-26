use std::path::PathBuf;

use crate::errors::AppError;



pub async fn parse_file_size_bytes(path: PathBuf) -> Result<u64, AppError> {
    // Read file metadata
    let file_metadata = tokio::fs::metadata(path)
        .await
        .map_err(|e| AppError::FileReadError(format!("{e}")))?;
    let file_size = file_metadata.len();

    Ok(file_size)
}