pub mod server {
    use axum::{Router, routing::get};
    use dotenv::dotenv;
    use std::env;
    use std::net::SocketAddr;

    pub async fn start() {
        dotenv().ok();

        let port: u16 = env::var("PORT")
            .unwrap_or("8080".into())
            .parse()
            .expect("Failed to fetch port");

        println!("🚀 Server starting...");

        let app = Router::new().route("/test", get(handler));

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .expect("Failed to bind");

        println!("🌐 Listening on http://{}", addr);

        axum::serve(listener, app).await.expect("Server failed");
    }

    async fn handler() -> &'static str {
        "Hello, World!"
    }
}
