use thiserror::Error;

#[derive(Error, Debug)]

pub enum AppError {
    #[error("400 <|> Bad Request. Reason: '{0}'")]
    BadRequest(String),

    #[error("404 <|> Resource not found with {0}")]
    NotFound(String),

    #[error("500 <|> Unexpected error: {0}")]
    Unknown(#[from] std::fmt::Error),

    #[error("500 <|> Unexpected sql error: {0}")]
    SQL(#[from] rusqlite::Error),

    #[error("500 <|> Unable to get connection pool: {0}")]
    R2d2SqliteError(#[from] r2d2::Error),

    #[error("500 <|> Unable to complete upload to S3: {0}")]
    S3UploadError(String),

    #[error("400 <|> Unable to read file from disk: {0}")]
    FileReadError(String),


    #[error("500 <|> Unable to encode struct to JSON: {0}")]
    JsonEncodeError(#[from] serde_json::Error),

    #[error("500 <|> Unable to call Tauri API: {0}")]
    TauriError(#[from] tauri::Error),
}