use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_partial::SerializePartial;
use uuid::Uuid;

#[derive(Serialize, Deserialize, SerializePartial)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub pw_hash: String,
    pub created_at: String,
    pub friend_list: Vec<Uuid>,
}

impl User {
    #[must_use]
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            pw_hash: password_hash,
            created_at: Utc::now().to_rfc3339(),
            friend_list: vec![],
        }
    }
}
