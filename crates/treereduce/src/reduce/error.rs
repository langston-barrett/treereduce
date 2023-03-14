use std::io;
use std::sync::PoisonError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReductionError {
    #[error("I/O error")]
    Disconnect(#[from] io::Error),
    #[error("JSON serialization error")]
    Json(#[from] serde_json::Error),
    #[error("Lock poisoned")]
    LockError(String),
}

impl<T> From<PoisonError<T>> for ReductionError {
    fn from(e: PoisonError<T>) -> ReductionError {
        ReductionError::LockError(format!("{}", e))
    }
}

#[derive(Debug, Error)]
pub enum MultiPassReductionError {
    #[error("I/O error")]
    Disconnect(#[from] io::Error),
    #[error("reduction error")]
    Reduction(#[from] ReductionError),
    #[error("utf-8 decoding error")]
    Utf8(#[from] std::str::Utf8Error),
}
