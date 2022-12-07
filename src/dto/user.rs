use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub pw_hash: String,
    pub created_at: SystemTime,
    pub friend_list: Vec<Uuid>,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        User {
            id: Uuid::new_v4(),
            username: username,
            pw_hash: password_hash,
            created_at: SystemTime::now(),
            friend_list: vec![],
        }
    }
}
