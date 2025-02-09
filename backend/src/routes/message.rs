use rocket::{http::Status, serde::json::Json, tokio::sync::broadcast::Sender, State};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{Message, Room};

#[rocket::get("/room/<room_id>")]
pub async fn get_messages(
    pool: &State<PgPool>,
    room_id: String,
) -> Result<Json<Vec<Message>>, Status> {
    let room = Room::by_id(pool, room_id).await;

    match room {
        Ok(Some(room)) => {
            let messages = Message::by_room_id(pool, room.room_id).await;

            match messages {
                Ok(messages) => Ok(Json(messages)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(Deserialize)]
struct PostMessage {
    user_id: Uuid,
    content: String,
}

#[rocket::post("/", data = "<data>")]
pub async fn create_message(
    pool: &State<PgPool>,
    queue: &State<Sender<Message>>,
    data: Json<PostMessage>,
) -> Result<Json<Message>, Status> {
    let message = Message::create(pool, queue, data.user_id, data.content.to_owned())
        .await
        .unwrap();

    // error handling where

    Ok(Json(message))
}
