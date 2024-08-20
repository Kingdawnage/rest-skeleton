use server::run;

#[tokio::main]
async fn main() {
    // let port = 3000;

    // // build application with a route
    // let app: Router = Router::new()
    //     .route("/", get(root))
    //     .route("/users", post(create_user))
    //     .route("/test", post(test_post));

    // // run it with hyper on localhost:3000
    // let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
    //     .await
    //     .unwrap();
    // println!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(listener, app).await.unwrap();
    run().await;
}
