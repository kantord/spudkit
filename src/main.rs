use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await.unwrap();
    println!("Listening on http://localhost:3030");
    axum::serve(listener, app).await.unwrap();
}
