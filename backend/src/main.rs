mod database;
mod server;
mod errors;
mod models;

#[tokio::main]
async fn main() {
    database::client::connect().await;

    server::server::start().await;
}
