use std::collections::HashMap;

use axum::extract::Path;
use axum::extract::Query;
use axum::http::HeaderMap;
use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/book/{id}", get(path_extract))
        .route("/book", get(query_extract))
        .route("/header", get(header_extract));

    // axum do anything if not talking to the network
    // axum uses tokio to handle connectivity
    // tcp is the internet protocol that http uses
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
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
