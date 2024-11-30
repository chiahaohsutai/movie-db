mod common;
use common::spawn;
use rand::{seq::SliceRandom, thread_rng};
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use server::schema::Movie;

#[actix_web::test]
async fn test_read_movie() {
    let server = spawn().await;

    let movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_one(&server.db)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/movies/get/{}", server.addr, movie.id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);

    let response: Movie = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(response, movie);
}

#[actix_web::test]
async fn test_insert_movie() {
    let server = spawn().await;
    let db = server.db.clone();

    let count = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM movies")
        .fetch_one(&db)
        .await
        .unwrap();

    let movie = json!({
        "id": count.0 + 1,
        "budget": 55000000,
        "revenue": 677945399,
        "vote_count": 14075,
        "popularity": 21.946943,
        "vote_average": 7.7,
        "genres": "Adventure, Fantasy, Action",
        "title": "Harry Potter and the Order of the Phoenix",
        "keywords": "saving the world, witch, magic, sorcery",
        "overview": "Harry's fifth year of study at Hogwarts.",
        "release_date": "2007-06-28"
    });

    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/movies/ins", server.addr))
        .header(CONTENT_TYPE, "application/json")
        .body(movie.to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);

    let inserted_movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = ?")
        .bind(movie["id"].as_i64().unwrap())
        .fetch_one(&db)
        .await
        .unwrap();

    assert_eq!(
        inserted_movie,
        serde_json::from_value::<Movie>(movie).unwrap()
    );
}

#[actix_web::test]
async fn test_delete_movie() {
    let server = spawn().await;

    let movies = sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_all(&server.db)
        .await
        .unwrap();

    let movie = movies
        .choose(&mut thread_rng())
        .expect("No movies in the database.");

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/movies/del/{}", server.addr, movie.id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);

    let deleted_movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = ?")
        .bind(movie.id)
        .fetch_optional(&server.db)
        .await
        .unwrap();
    assert_eq!(deleted_movie, None);
}

#[actix_web::test]
async fn test_update_movie() {
    let server = spawn().await;

    let movies = sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_all(&server.db)
        .await
        .unwrap();

    let movie = movies
        .choose(&mut thread_rng())
        .expect("No movies in the database.");
    
    let new_title = "Updated Title";
    let new_budget = 12000;

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/movies/upd/{}", server.addr, movie.id))
        .header(CONTENT_TYPE, "application/json")
        .body(json!({"title": new_title, "budget": new_budget}).to_string())
        .send()
        .await
        .expect("Failed to execute request.");

    let mut updated_movie = sqlx::query_as::<_, Movie>("SELECT * FROM movies WHERE id = ?")
        .bind(movie.id)
        .fetch_one(&server.db)
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(updated_movie.title, new_title);
    assert_eq!(updated_movie.budget, new_budget);

    updated_movie.title = movie.title.to_string();
    updated_movie.budget = movie.budget;
    assert_eq!(updated_movie, *movie);
}

#[actix_web::test]
async fn test_search_movie() {
    let server = spawn().await;

    let movies = sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_all(&server.db)
        .await
        .unwrap();

    let movie = movies
        .choose(&mut thread_rng())
        .expect("No movies in the database.");

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/movies/search/{}/{}", server.addr, movie.title, 1))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);

    let response: Vec<Movie> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert!(response.contains(movie));
}

#[actix_web::test]
async fn test_read_from_to() {
    let server = spawn().await;

    let movies = sqlx::query_as::<_, Movie>("SELECT * FROM movies")
        .fetch_all(&server.db)
        .await
        .unwrap();

    let from = 1;
    let to = movies.len();

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/movies/get/range/{}/{}", server.addr, from, to))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);

    let response: Vec<Movie> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(response, movies[from..to]);
}
