use server::{config::config, serve, telemetry};
use sqlx::SqlitePool;
use std::net::{SocketAddr, TcpListener};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config().expect("Failed to read configuration.");
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    let listener = TcpListener::bind(addr).expect("Failed to bind random port.");
    let db = SqlitePool::connect(config.url.as_str())
        .await
        .expect("Failed to connect to database.");

    let _ = telemetry::init(
        String::from("server"),
        String::from("info"),
        std::io::stdout,
    );
    serve(listener, db)
        .await
        .expect("Failed to start server")
        .await
}
