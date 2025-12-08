use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn connect() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Failed to connect to Database");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Connected to Database !!");

    pool
}
