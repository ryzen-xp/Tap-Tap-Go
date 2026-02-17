use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn connect_db() -> PgPool {
    let url = env::var("DATABASE_URL").expect("failed to read DATABASE_URL");

    println!("Connecting to PostgreSQL...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("failed to connect to database");

    println!("Connected to Database!");

    pool
}
