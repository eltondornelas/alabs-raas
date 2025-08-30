use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use reqwest::Method;
use tower::{ServiceBuilder, limit::ConcurrencyLimitLayer};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};

#[tokio::main]
async fn main() {
    let service = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .layer(ConcurrencyLimitLayer::new(100));

    let app = Router::new()
        .route("/", get(handler))
        .layer(service.into_inner());
    // .layer(CompressionLayer::new());
    // can see on browser the difference between "size" and "transferred", the Compression do it on the fly

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("war_and_peace.txt");
    Html(WAR_AND_PEACE)
}
