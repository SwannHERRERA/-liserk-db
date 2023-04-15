use axum::{Router, Server};

#[tokio::main]
async fn main() {
    // Build our application by creating our router.
    let app = Router::new();

    // Run our application as a hyper server on http://localhost:3000.
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
