use std::collections::HashMap;

use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Router, response::Html, routing::get};
use std::sync::Arc; // common wrapper for State because most of the time the State will be cloned into a handler function

struct MyConfig {
    config_string: String,
}

#[tokio::main]
async fn main() {
    let shared_config = Arc::new(MyConfig {
        config_string: "My config string".to_string(),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract))
        .with_state(shared_config); // with_state is only one per router, so in large programs usually goes for .layers and .with_state for smaller programs

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
    Html(format!("<h1>{}</h1>", config.config_string))
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
