use dotenv::dotenv;
use model::Message;
use rocket::tokio::sync::broadcast::channel;
use rocket_cors::CorsOptions;
use routes::{
    message::{create_message, get_messages},
    room::{create_room, get_room, stream_room},
    user::create_user,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod model;
mod routes;

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let cors = CorsOptions::default()
        .to_cors()
        .expect("Failed to create CORS");

    rocket::build()
        .attach(cors)
        .manage(pool)
        .manage(channel::<Message>(100).0)
        .mount(
            "/api/v1/room",
            rocket::routes![get_room, create_room, stream_room],
        )
        .mount("/api/v1/user", rocket::routes![create_user])
        .mount(
            "/api/v1/message",
            rocket::routes![get_messages, create_message],
        )
}
