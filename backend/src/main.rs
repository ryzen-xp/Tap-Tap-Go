mod server;

#[tokio::main]
async fn main() {
    server::server::start().await;
}
