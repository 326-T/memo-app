use sqlx;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,
    #[error("Resource already exists")]
    Conflict,
    #[error("Internal server error")]
    InternalServerError,
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        AppError::InternalServerError
    }
}
