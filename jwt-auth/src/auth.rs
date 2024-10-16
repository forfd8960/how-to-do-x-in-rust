use std::fmt;

use crate::{errors::AppError, user::User, AppState};

pub trait TokenVeirfy {
    type Error: fmt::Debug;
    fn vetify(&self, token: &str) -> Result<User, Self::Error>;
}

impl TokenVeirfy for AppState {
    type Error = AppError;
    fn vetify(&self, token: &str) -> Result<User, Self::Error> {
        Ok(self.dk.verify(token)?)
    }
}
