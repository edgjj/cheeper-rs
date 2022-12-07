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
    pub fn new(author_id: Uuid, text: String) -> Self {
        Message {
            id: Uuid::new_v4(),
            author_id: author_id,
            text: text,
            created_at: Utc::now().to_rfc3339(),
        }
    }
}
