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
}