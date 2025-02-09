use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod message;
pub mod room;
pub mod user;

#[derive(Deserialize, Serialize, Debug)]
pub struct Room {
    pub room_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub room_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub message_id: Uuid,

    pub sender: User,

    pub content: String,
    pub time: String,
}
