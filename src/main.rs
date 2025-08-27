use std::sync::atomic::AtomicUsize;

use axum::Json;
use axum::extract::State;
use axum::{Router, response::Html, routing::get};
use std::sync::Arc; // common wrapper for State because most of the time the State will be cloned into a handler function

struct Counter {
    count: AtomicUsize,
    // note: if using Mutex to lock some data, make sure to use wisely and not Mutex the root data or global data or you would be locking the whole handler having to wait
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Counter {
        count: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/inc", get(increment))
        .with_state(counter);
    // state is local to the service Router; if have data needed to be shared between modules it has to go in a layer (Extension)

    // axum do anything if not talking to the network
    // axum uses tokio to handle connectivity
    // tcp is the internet protocol that http uses
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    println!("Sending GET request");
    let current_count = reqwest::get("http://localhost:3001/inc")
        .await
        .unwrap()
        .json::<i32>()
        .await
        .unwrap();

    Html(format!("<h1>Remote Counter: {current_count}</h1>"))
}

async fn increment(State(counter): State<Arc<Counter>>) -> Json<usize> {
    println!("/inc service called");
    let current_value = counter
        .count
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Json(current_value)
}
