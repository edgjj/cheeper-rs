use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub author_id: Uuid,
    pub created_at: SystemTime,
    pub text: String,
}
