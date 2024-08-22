mod crud;
mod middlewares;
mod root;

use crate::config::create_pool;
use crud::{create_user, get_users};
use middlewares::*;
use root::root;

use axum::{routing::get, Router};

pub async fn create_routes() -> Router {
    let pool = create_pool().await;
    let cors = create_cors().await;

    Router::new()
        .route("/", get(root))
        .route("/api/users", get(get_users).post(create_user))
        .with_state(pool)
        .layer(cors)
}
