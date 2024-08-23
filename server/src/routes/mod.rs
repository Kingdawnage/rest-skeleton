mod crud;
mod root;

use crate::config::create_pool;
use crate::middlewares::*;
use crud::*;
use root::root;

use axum::{routing::get, Router};

pub async fn create_routes() -> Router {
    let pool = create_pool().await;
    let cors = create_cors().await;

    Router::new()
        .route("/", get(root))
        .route("/api/users", get(get_users).post(create_user))
        .route(
            "/api/users/:id",
            get(get_user).delete(delete_user).patch(update_user),
        )
        .with_state(pool)
        .layer(cors)
}
