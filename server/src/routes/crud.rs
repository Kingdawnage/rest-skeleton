use crate::config::create_pool;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, query, query_as, PgPool};
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

pub async fn get_user(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await;

    match query {
        Ok(user) => {
            let response = json!({
                "staus": "success",
                "data": json!({
                    "user": user
                })
            });
            return Ok(Json(response));
        }
        Err(_) => {
            let response = json!({
                "status": "fail",
                "message": format!("User with id {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(response)));
        }
    }
}

pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query = query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await;

    match query {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let response = json!({
                    "status": "success",
                    "message": "User deleted successfully"
                });
                return Ok(Json(response));
            } else {
                let response = json!({
                    "status": "fail",
                    "message": format!("User with id {} not found", id)
                });
                return Err((StatusCode::NOT_FOUND, Json(response)));
            }
        }
        Err(_) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to delete user")
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)));
        }
    }
}

pub async fn update_user(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let query = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await;

    if query.is_err() {
        let response = json!({
            "status": "fail",
            "message": format!("User with id {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(response)));
    }

    // let now = chrono::Utc::now();
    // let user = query.unwrap();

    let query = query_as!(
        User,
        "UPDATE users SET name = $1, password = $2, age = $3 WHERE id = $4 RETURNING *",
        payload.name.to_owned(),
        payload.password.to_owned(),
        payload.age.to_owned(),
        id
    )
    .fetch_one(&pool)
    .await;

    match query {
        Ok(user) => {
            let response = json!({
                "status": "success",
                "data": json!({
                    "user": user
                })
            });
            return Ok(Json(response));
        }
        Err(e) => {
            let response = json!({
                "status": "error",
                "message": e.to_string()
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response)));
        }
    }
}
