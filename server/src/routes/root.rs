use axum::Json;

pub async fn root() -> Json<&'static str> {
    Json("Hello, World!")
}
