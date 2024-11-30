use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use std::result::Result;
use std::vec;

use crate::schema::{Movie, PartialMovie};
use crate::AppState;

#[tracing::instrument(name = "System health check.")]
#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Adding a new movie.",
    skip(data, details),
    fields(movie_id=%details.id, movie_name=%details.title)
)]
#[post("/movies/ins")]
async fn add(data: web::Data<AppState>, details: web::Json<Movie>) -> impl Responder {
    let db = &data.db;
    let movie = serde_json::to_value(details.into_inner());

    if let Ok(Value::Object(movie)) = movie {
        let mut fields = vec![];
        let mut values = vec![];

        for (k, v) in movie {
            fields.push(k);
            values.push(v.to_string());
        }

        match insert(db, fields, values).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(json!({"message": err.to_string()}).to_string()),
        }
    } else {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(json!({"message": "Invalid payload values."}).to_string())
    }
}

#[tracing::instrument(name = "Adding an entry in the database.", skip(conn, fields, values))]
async fn insert(
    conn: &SqlitePool,
    fields: Vec<String>,
    values: Vec<String>,
) -> Result<(), sqlx::Error> {
    let query = format!(
        "INSERT INTO movies ({}) VALUES ({})",
        fields.join(", "),
        values.join(", ")
    );
    let _ = sqlx::query(&query).execute(conn).await?;
    Ok(())
}

#[tracing::instrument(name = "Removing a movie.", skip(data, path), fields(movie_id=%path))]
#[post("/movies/del/{movie_id}")]
async fn delete_movie(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let db = &data.db;
    let id = path.into_inner();

    match delete(db, id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(json!({"message": err.to_string()}).to_string()),
    }
}

#[tracing::instrument(name = "Deleting an entry from the database.", skip(conn, id))]
async fn delete(conn: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    let _ = sqlx::query("DELETE FROM movies WHERE id = ?")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}

#[tracing::instrument(name = "Fetching movies in a range.", skip(data, path))]
#[get("/movies/get/range/{from}/{to}")]
async fn fetch_from_to(data: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let db = &data.db;
    let (from, to) = path.into_inner();

    if from > to {
        return HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(json!({"message": "Invalid range."}).to_string());
    }

    match read_from_to(db, from, to).await {
        Ok(movies) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&movies).unwrap()),
        Err(err) => {
            tracing::error!("Failed to fetch movies: {:?}", err);
            HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(json!({"message": err.to_string()}).to_string())
        }
    }
}

#[tracing::instrument(name = "Reading entries in range (from -> to)", skip(conn, from, to))]
async fn read_from_to(conn: &SqlitePool, from: i32, to: i32) -> Result<Vec<Movie>, sqlx::Error> {
    let movies = sqlx::query_as::<_, Movie>("SELECT * FROM movies LIMIT ?, ?")
        .bind(from)
        .bind(to)
        .fetch_all(conn)
        .await?;
    print!("{:?}", movies);
    Ok(movies)
}

#[tracing::instrument(name = "Fetching a movie.", skip(data, path), fields(movie_id=%path))]
#[get("/movies/get/{movie_id}")]
async fn fetch(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let db = &data.db;
    let movie_id = path.into_inner();

    match read(db, movie_id).await {
        Ok(movie) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&movie).unwrap()),
        Err(err) => {
            tracing::error!("Failed to fetch movie: {:?}", err);
            HttpResponse::NotFound()
                .content_type(ContentType::json())
                .body(json!({"message": err.to_string()}).to_string())
        }
    }
}

#[tracing::instrument(name = "Reading the database.", skip(conn, id))]
async fn read(conn: &SqlitePool, id: i32) -> Result<Movie, sqlx::Error> {
    let movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = ?")
        .bind(id)
        .fetch_one(conn)
        .await?;
    Ok(movie)
}

#[tracing::instrument(name = "Modifying a movie.", skip(data, path, details), fields(movie_id=%path))]
#[post("/movies/upd/{movie_id}")]
async fn modify(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    details: web::Json<PartialMovie>,
) -> impl Responder {
    let id = path.into_inner();
    let db = &data.db;
    let movie = serde_json::to_value(details.into_inner());

    if let Ok(Value::Object(movie)) = movie {
        let mut assignments = vec![];

        for (k, v) in movie {
            if !v.is_null() {
                assignments.push(format!("{} = {}", k, v.to_string()));
            }
        }

        match update(db, assignments, id).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(json!({"message": err.to_string()}).to_string()),
        }
    } else {
        HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(json!({"message": "Invalid payload values."}).to_string())
    }
}

#[tracing::instrument(name = "Updating the database.", skip(conn, id))]
async fn update(conn: &SqlitePool, assignments: Vec<String>, id: i32) -> Result<(), sqlx::Error> {
    let query = format!("UPDATE movies SET {} WHERE id = ?", assignments.join(", "));
    let _ = sqlx::query(query.as_str()).bind(id).execute(conn).await?;
    Ok(())
}

#[tracing::instrument(name = "Searching for a movie.", skip(data, query), fields(search_query=%query.0))]
#[get("/movies/search/{query}/{n}")]
async fn search(data: web::Data<AppState>, query: web::Path<(String, i32)>) -> impl Responder {
    let db = &data.db;
    let se = &data.se;
    let (query, n) = query.into_inner();

    let search_results: Vec<i64> = se.search(&query).into_iter().take(n as usize).collect();
    let futures = search_results.iter().map(|id| async move {
        match sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await
        {
            Ok(movie) => Some(movie),
            Err(_) => None,
        }
    });

    let results: Vec<Movie> = futures::future::join_all(futures)
        .await
        .into_iter()
        .filter_map(|x| x.into())
        .collect();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&results).unwrap())
}
