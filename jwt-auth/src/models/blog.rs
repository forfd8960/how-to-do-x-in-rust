use serde::Serialize;
use uuid::Uuid;

use super::user::User;

#[derive(Debug, Serialize)]
pub struct Blog {
    id: String,
    author: User,
    title: String,
    content: String,
}

impl Blog {
    pub fn new(author: User, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            author,
            title,
            content,
        }
    }
}
