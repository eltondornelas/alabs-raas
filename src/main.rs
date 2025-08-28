use axum::{response::{Html, IntoResponse}, routing::get, Router};
use reqwest::StatusCode;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .fallback_service(ServeDir::new("web"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<impl IntoResponse, StatusCode> {
    Ok(Html("<h1>Hello, World!</h1>"))
}
