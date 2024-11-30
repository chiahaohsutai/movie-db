use once_cell::sync::Lazy;
use server::serve;
use sqlx::SqlitePool;
use std::fs;

static TRACING: Lazy<()> = Lazy::new(|| {
    let name = String::from("test");
    let filter = String::from("info");

    if std::env::var("TEST_LOG").is_ok() {
        server::telemetry::init(name, filter, std::io::stdout);
    } else {
        server::telemetry::init(name, filter, std::io::sink);
    };
});

#[allow(dead_code)]
pub struct TestServer {
    pub addr: String,
    pub db: sqlx::SqlitePool,
}
impl TestServer {
    pub fn new(addr: String, db: sqlx::SqlitePool) -> Self {
        Self { addr, db }
    }
}

async fn migrate(conn: SqlitePool) {
    let path = "tests/common/migrate.sql";
    let sql = fs::read_to_string(path).expect("Missing migration file.");

    let _ = sqlx::query(&sql)
        .execute(&conn)
        .await
        .expect("Failed to migrate.");
}

pub async fn spawn() -> TestServer {
    Lazy::force(&TRACING);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = std::net::TcpListener::bind(addr).expect("Failed to bind random port.");

    let conn = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let _ = migrate(conn.clone()).await;

    let addr = format!("http://{}", listener.local_addr().unwrap().to_string());
    let _ = tokio::spawn(serve(listener, conn.clone()).await.expect("Failed to start server."));

    TestServer::new(addr, conn.clone())
}
