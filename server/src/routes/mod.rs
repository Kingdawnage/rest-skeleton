mod create_user;
mod db_config;
mod root;

use create_user::create_user;
use db_config::create_pool;
use db_config::get_users;
use root::root;

use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub async fn create_routes() -> Router {
    let pool = create_pool().await;
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(root))
        .to_owned()
        .route("/users", post(create_user))
        .to_owned()
        .route("/db", get(get_users))
        .with_state(pool)
        .layer(cors)
}
