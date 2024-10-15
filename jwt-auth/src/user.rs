use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    user_id: String,
    user_name: String,
    email: String,
}

impl User {
    pub fn new(user_name: String, email: String) -> Self {
        Self {
            user_id: Uuid::new_v4().to_string(),
            user_name,
            email,
        }
    }
}
