//  Modules
mod db;
mod state;
mod models;
mod services;
mod routes;

//  Imports
use axum::{Router, routing::get};
use dotenv::dotenv;
use std::{env, net::SocketAddr};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let port: u16 = port.parse().expect("port must be number");

    let pool: sqlx::Pool<sqlx::Postgres> = db::connect_db().await;

    let app_state = AppState {
        db  : pool
    };

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Tap-Tap-Go running at http://localhost:{}", port);

    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .expect("failed to start sercer!!"),
        app,
    )
    .await
    .expect("Server Failed");
}

async fn root() -> &'static str {
    "Tap-Tap-Go Server is Running 🦀"
}
