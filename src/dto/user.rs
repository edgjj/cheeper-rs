use actix_web::http::header::Date;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_partial::SerializePartial;
use uuid::Uuid;

#[derive(Serialize, Deserialize, SerializePartial, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub pw_hash: String,
    pub created_at: String,
    pub friend_list: Vec<Uuid>,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        User {
            id: Uuid::new_v4(),
            username: username,
            pw_hash: password_hash,
            created_at: Utc::now().to_rfc3339(),
            friend_list: vec![],
        }
    }
}
