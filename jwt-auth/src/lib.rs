use std::{ops::Deref, sync::Arc};

use config::AppConfig;
use errors::AppError;
use jwt::{DecodingKey, EncodingKey};

pub mod config;
pub mod errors;
pub mod jwt;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    state: Arc<AppStateInner>,
}

#[derive(Clone)]
pub struct AppStateInner {
    pk: EncodingKey,
    dk: DecodingKey,
}

impl AppState {
    pub fn new(config: &AppConfig) -> Result<Self, AppError> {
        let pk = EncodingKey::load(&config.private_pem)?;
        let dk = DecodingKey::load(&config.public_pem)?;

        Ok(Self {
            state: Arc::new(AppStateInner { pk, dk }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
