use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub async fn create_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers(Any)
}
