use axum::{response::Html, routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    // Setup default tracing
    tracing_subscriber::fmt::init();
    info!("Starting server"); // emit a event into the tracing system using level "information"
    
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    tracing::warn!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();    
}

async fn handler() -> Html<&'static str> {
    tracing::error!("Serving Hello World");
    let res = Html("<h1>Hello, World!</h1>");
    tracing::debug!("Exiting Handler"); 
    // debug doesn't display by default
    // by default tracing look for a environment variable RUST_LOG
    // RUST_LOG=debug cargo run
    return res
}

// cargo add tracing -> emit events
// cargo add tracing_subscriber -> receive and do something