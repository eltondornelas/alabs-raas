use axum::{Router, response::Html, routing::get};
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    // Setup default tracing
    tracing_subscriber::fmt::init();
    info!("Starting server");

    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http());
    // careful with this trace it will log tons of iformation
    // need de RUST_LOG=debug variable

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    info!("Serving Hello World");
    Html("<h1>Hello, World!</h1>")
}

// cargo add tower_http -F trace
