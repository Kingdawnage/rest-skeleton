use crate::config::create_pool;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, query_as, PgPool};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    name: String,
    password: String,
    age: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    id: Uuid,
    name: String,
    password: String,
    age: i32,
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let pool = create_pool().await;

    let query_result = query_as!(
        User,
        "INSERT INTO users (id, name, password, age) VALUES ($1, $2, $3, $4) RETURNING *",
        Uuid::new_v4(),
        payload.name,
        payload.password,
        payload.age
    )
    .fetch_one(&pool)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = json!({
                "status": "success",
                "data": json!({
                    "user": user
                })
            });
            return Ok((StatusCode::CREATED, Json(user_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraints")
            {
                let error_response = json!({
                    "status": "fail",
                    "message": "User already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": e.to_string(),
                })),
            ));
        }
    }
}

pub async fn get_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .expect("can't fetch user");

    Json(user)
}
