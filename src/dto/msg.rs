use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub author_id: Uuid,
    pub created_at: String,
    pub text: String,
}

impl Message {
    #[must_use]
    pub fn new(author_id: String, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            author_id: Uuid::parse_str(author_id.as_str()).unwrap(),
            text,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}
