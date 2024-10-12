use auth_error::AuthError;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

pub mod app;
pub mod auth_error;

use once_cell::sync::Lazy;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 60);
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    username: String,
    exp: usize,
}

impl Claims {
    pub fn new(username: String, exp: usize) -> Self {
        Self {
            username: username,
            exp: exp,
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    //  We also use de-structuring here to extract the bearer struct from the TypedHeader<Authorization<Bearer>> type as it’s much easier to access.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
