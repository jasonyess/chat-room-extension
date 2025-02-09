use rocket::State;
use sqlx::PgPool;
use uuid::Uuid;

use super::User;

impl User {
    pub async fn by_id(pool: &State<PgPool>, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id = $1", user_id)
            .fetch_one(&**pool)
            .await;

        match result {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub async fn create(
        pool: &State<PgPool>,
        user_id: Uuid,
        name: String,
        room_id: String,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_file_as!(
            User,
            "src/model/sql/create_user.sql",
            user_id,
            name,
            room_id
        )
        .fetch_one(&**pool)
        .await?;

        Ok(user)
    }
}
