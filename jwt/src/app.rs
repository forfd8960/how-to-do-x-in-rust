use axum::Json;
use chrono::Utc;
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};

use crate::{auth_error::AuthError, Claims, KEYS};

const USERNAME: &'static str = "AlexZ";
const PASSWORD: &'static str = "MySec1012@$";

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, AuthError> {
    if payload.username == "" {
        return Err(AuthError::MissingCredentials);
    }

    // verify user in DB
    if payload.username != USERNAME || payload.password != PASSWORD {
        return Err(AuthError::WrongCredentials);
    }

    let exp = (Utc::now().naive_utc() + chrono::naive::Days::new(2))
        .and_utc()
        .timestamp() as usize;
    let claims = Claims::new(payload.username, exp);

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthResponse { token }))
}
