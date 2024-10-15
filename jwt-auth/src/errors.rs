use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Wrong credential")]
    WrongCredentials,

    #[error("Missing credentials")]
    MissingCredentials,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("http header parse error: {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HttpHeaderError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidToken => StatusCode::BAD_REQUEST,
            Self::WrongCredentials => StatusCode::FORBIDDEN,
            Self::MissingCredentials => StatusCode::BAD_REQUEST,
        };

        (
            status,
            Json(ErrorOutput {
                error: self.to_string(),
            }),
        )
            .into_response()
    }
}
