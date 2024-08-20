use axum::{extract::State, Json};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, PgPool, Pool, Postgres};
use std::env::var;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: Uuid,
    name: String,
    password: String,
    age: i32,
}

pub async fn create_pool() -> Pool<Postgres> {
    dotenv().ok();
    let conn_str: String = var("CONN_STRING").expect("CONN_STRING must be set").into();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Failed to create pool");
    pool
}

pub async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("can't fetch user");

    Json(user)
}
