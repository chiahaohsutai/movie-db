use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq)]
pub struct Movie {
    pub id: i64,
    pub budget: i64,
    pub revenue: i64,
    pub vote_count: i64,
    pub popularity: f32,
    pub vote_average: f32,
    pub genres: String,
    pub title: String,
    pub keywords: String,
    pub overview: String,
    pub release_date: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug, PartialEq)]
pub struct PartialMovie {
    pub budget: Option<i64>,
    pub revenue: Option<i64>,
    pub vote_count: Option<i64>,
    pub popularity: Option<f32>,
    pub vote_average: Option<f32>,
    pub genres: Option<String>,
    pub title: Option<String>,
    pub keywords: Option<String>,
    pub overview: Option<String>,
    pub release_date: Option<String>,
}