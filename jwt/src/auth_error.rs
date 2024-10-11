use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    TokenCreation,
    MissingCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credential"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credential"),
        };

        let body = Json(json!({
            "error": err_msg,
        }));

        (status, body).into_response()
    }
}
