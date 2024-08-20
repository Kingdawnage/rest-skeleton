mod middlewares;
mod routes;

use routes::create_routes;

pub async fn run() {
    let port = 8080;

    // build application with a route
    let routes = create_routes().await;

    let app = routes;

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
