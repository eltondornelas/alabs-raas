use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let other = Router::new().route("/other", get(handler2));

    let app = Router::new().route("/", get(handler)).merge(other);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler2() -> Html<&'static str> {
    Html("<h1>Hello, World 2!</h1>")
}
