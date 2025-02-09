use rocket::{http::Status, serde::json::Json, State};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::User;

#[derive(Debug, Deserialize)]
struct PostUser {
    room_id: String,
    name: String,
}

#[rocket::post("/", data = "<data>")]
pub async fn create_user(pool: &State<PgPool>, data: Json<PostUser>) -> Result<Json<User>, Status> {
    let user = User::create(
        pool,
        Uuid::new_v4(),
        data.name.clone(),
        data.room_id.clone(),
    )
    .await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
