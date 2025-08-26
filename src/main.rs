use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;

use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Router, response::Html, routing::get};
use std::sync::Arc; // common wrapper for State because most of the time the State will be cloned into a handler function

struct MyConfig {
    counter: AtomicUsize,
    // note: if using Mutex to lock some data, make sure to use wisely and not Mutex the root data or global data or you would be locking the whole handler having to wait
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        counter: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .with_state(shared_config);

    // axum do anything if not talking to the network
    // axum uses tokio to handle connectivity
    // tcp is the internet protocol that http uses
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn handler(
    State(config): State<Arc<MyConfig>>,
    // destructuring
) -> Html<String> {
    config
        .counter
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    
    Html(format!(
        "<h1>Your are visitor number {}</h1>",
        config.counter.load(std::sync::atomic::Ordering::Relaxed)
    ))
}

async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    Html(format!("Hello, {id}!"))
}

async fn query_extract(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("Hello, {params:#?}!"))
}

async fn header_extract(
    headers: HeaderMap,
    // is not really a extractor
) -> Html<String> {
    Html(format!("{headers:#?}"))
}
