use std::{env::VarError, io::Error as StdError, num::TryFromIntError};

use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Sql(#[from] sqlx::Error),

    #[error("DB connection error")]
    Cn(#[from] VarError),

    #[error("IO error")]
    Io(#[from] StdError),

    #[error("bcrypt error")]
    Bypt(#[from] BcryptError),

    #[error("JWT error")]
    JWT(#[from] JwtError),

    #[error("Invalid user")]
    InvalidUser,

    #[error("JSON error")]
    Json(#[from] serde_json::Error),

    #[error("Data type error")]
    Datatype,

    #[error("Conversion error")]
    Conversion(#[from] TryFromIntError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::Sql(_) | Self::Cn(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong with the database or its connection",
            ),
            Self::Io(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "I/O error occurred while connecting to the server",
            ),
            Self::Bypt(_) | Self::JWT(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication failed while processing the user",
            ),
            Self::InvalidUser => (
                StatusCode::FORBIDDEN,
                "You are not authorized to access this resource",
            ),
            Self::Json(_) => (
                StatusCode::BAD_REQUEST,
                "Invalid JSON format in the request",
            ),
            Self::Datatype => (
                StatusCode::BAD_REQUEST,
                "Expected an executable (.exe) file",
            ),
            Self::Conversion(_) => (StatusCode::BAD_REQUEST, "Failed to convert number type"),
        };

        (status, message).into_response()
    }
}
