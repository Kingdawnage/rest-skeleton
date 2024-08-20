mod middlewares;
mod routes;

use routes::create_routes;

pub async fn run() {
    let port = 8080;

    let routes = create_routes().await;

    let app = routes;

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
