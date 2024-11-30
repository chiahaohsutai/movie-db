use actix_web::{dev::Server, web, App, HttpServer};
use schema::Movie;
use simsearch::{SearchOptions, SimSearch};
use sqlx::SqlitePool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub mod config;
pub mod routes;
pub mod schema;
pub mod telemetry;

struct AppState {
    db: SqlitePool,
    se: SimSearch<i64>,
}

async fn build_search_engine(db: SqlitePool) -> SimSearch<i64> {
    let options = SearchOptions::new().levenshtein(true);
    let mut engine: SimSearch<i64> = SimSearch::new_with(options);

    let _ = sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_all(&db)
        .await
        .expect("Failed to fetch movies.")
        .iter()
        .for_each(|movie| engine.insert(movie.id, movie.title.as_str()));

    engine
}

pub async fn serve(listener: TcpListener, db: SqlitePool) -> Result<Server, std::io::Error> {
    let engine = build_search_engine(db.clone()).await;

    let app = move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                db: db.clone(),
                se: engine.clone(),
            }))
            .service(routes::health_check)
            .service(routes::fetch)
            .service(routes::add)
            .service(routes::delete_movie)
            .service(routes::modify)
            .service(routes::search)
            .service(routes::fetch_from_to)
    };
    Ok(HttpServer::new(app)
        .listen(listener)
        .expect("Failed to bind server.")
        .run())
}
