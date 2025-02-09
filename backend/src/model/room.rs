use rocket::State;
use sqlx::{self, PgPool};

use super::Room;

impl Room {
    pub async fn by_id(pool: &State<PgPool>, room_id: String) -> Result<Option<Room>, sqlx::Error> {
        let result = sqlx::query_as!(Room, "SELECT * FROM rooms WHERE room_id = $1", room_id)
            .fetch_one(&**pool)
            .await;

        match result {
            Ok(room) => Ok(Some(room)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn create(pool: &State<PgPool>, room_id: String) -> Result<Room, sqlx::Error> {
        let room = sqlx::query_file_as!(Room, "src/model/sql/create_room.sql", room_id)
            .fetch_one(&**pool)
            .await?;

        Ok(room)
    }
}
