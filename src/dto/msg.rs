use serde::{Serialize, Deserialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Message{
    pub id: Uuid,
    pub author: Uuid,
    pub created_at: SystemTime,
    pub text: String,
}