use rocket::{tokio::sync::broadcast::Sender, State};
use sqlx::{types::chrono::NaiveDateTime, PgPool};
use uuid::Uuid;

use super::{Message, User};

struct SQLMessage {
    message_id: Uuid,
    user_id: Uuid,
    content: String,
    time: NaiveDateTime,
}

impl Message {
    // clone clone clone clone get themo ut
    pub async fn by_room_id(
        pool: &State<PgPool>,
        room_id: String,
    ) -> Result<Vec<Message>, sqlx::Error> {
        // most of this could probably be done with a sql join thingy
        let users: Vec<User> =
            sqlx::query_as!(User, "SELECT * FROM users WHERE room_id = $1", room_id)
                .fetch_all(&**pool)
                .await
                .unwrap();

        let messages: &mut Vec<Message> = &mut vec![];

        for user in users.iter() {
            let user_messages: Vec<Message> = sqlx::query_as!(
                SQLMessage,
                "SELECT * FROM messages WHERE user_id = $1",
                user.user_id
            )
            .fetch_all(&**pool)
            .await
            .unwrap()
            .into_iter()
            .map(|sql_message: SQLMessage| Message {
                message_id: sql_message.message_id,
                sender: user.clone(),
                content: sql_message.content,
                time: sql_message.time.to_string(),
            })
            .collect();

            messages.extend(user_messages);
        }

        messages.sort_by(|a, b| b.time.cmp(&a.time));

        Ok(messages.clone())
    }

    pub async fn create(
        pool: &State<PgPool>,
        queue: &State<Sender<Message>>,
        user_id: Uuid,
        content: String,
    ) -> Result<Message, sqlx::Error> {
        let user = User::by_id(pool, user_id).await.unwrap();

        let sql_message = sqlx::query_file_as!(
            SQLMessage,
            "src/model/sql/create_message.sql",
            Uuid::new_v4(),
            user_id,
            content,
        )
        .fetch_one(&**pool)
        .await
        .unwrap();

        let message = Message {
            message_id: sql_message.message_id,
            sender: user.unwrap(),
            content: sql_message.content,
            time: sql_message.time.to_string(),
        };

        queue.send(message.clone());

        Ok(message)
    }
}
