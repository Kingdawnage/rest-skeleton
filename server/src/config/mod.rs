use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env::var;

pub async fn create_pool() -> Pool<Postgres> {
    dotenv().ok();
    let conn_str: String = var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
        .into();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await
        .expect("Failed to create pool");
    pool
}
