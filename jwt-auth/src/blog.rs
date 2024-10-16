use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Blog {
    id: String,
    title: String,
    content: String,
}

impl Blog {
    pub fn new(title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
        }
    }
}
