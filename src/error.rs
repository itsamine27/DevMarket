use std::{env::VarError, io::Error as StdError};

use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
pub type Result<S> = std::result::Result<S, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("DB connection error: ")]
    CnError(#[from] VarError),
    #[error("io error")]
    IoError(#[from] StdError)
}
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::SqlxError(_) | Self::CnError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "sthg wrong in the data base or data base connection",
            ),
            Self::IoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "sthg when wrong when trying to connect to the server"
            )
            
        }.into_response()
    }
}
