use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    user_id: String,
    user_name: String,
    email: String,
}

impl User {
    pub fn new(user_name: String, email: String) -> Self {
        Self {
            user_id: "".to_string(),
            user_name: user_name,
            email: email,
        }
    }
}
