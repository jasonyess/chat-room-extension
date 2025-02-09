use std::sync::Arc;

use rocket::{
    futures::{SinkExt, StreamExt},
    http::Status,
    response::stream::{Event, EventStream},
    serde::json::Json,
    tokio::{
        select,
        sync::{
            broadcast::{error::RecvError, Sender},
            Mutex,
        },
    },
    Response, Shutdown, State,
};
use serde::Deserialize;
use serde_json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{Message, Room, User};

#[rocket::get("/<room_id>")]
pub async fn get_room(pool: &State<PgPool>, room_id: String) -> Result<Json<Room>, Status> {
    let room = Room::by_id(pool, room_id).await;

    match room {
        Ok(Some(room)) => Ok(Json(room)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(Debug, Deserialize)]
struct PostRoom {
    room_id: String,
}

#[rocket::post("/", data = "<data>")]
pub async fn create_room(pool: &State<PgPool>, data: Json<PostRoom>) -> Result<Json<Room>, Status> {
    let existing_room = Room::by_id(pool, data.room_id.clone()).await;

    match existing_room {
        Ok(Some(_)) => Err(Status::Conflict),
        Ok(None) => {
            let room = Room::create(pool, data.room_id.clone()).await;

            match room {
                Ok(room) => Ok(Json(room)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[rocket::get("/<room_id>/events")]
pub async fn stream_room(
    queue: &State<Sender<Message>>,
    room_id: String,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => {
                        if (room_id == msg.sender.room_id) {
                            msg
                        } else { continue }
                    },
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}
