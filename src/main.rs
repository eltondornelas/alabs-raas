use axum::{Json, Router, response::IntoResponse, routing::get};
use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    // axum do anything if not talking to the network
    // axum uses tokio to handle connectivity
    // tcp is the internet protocol that http uses
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("Listening on 127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Result<impl IntoResponse, (StatusCode, String)> {
    let start = std::time::SystemTime::now();
    let seconds_wrapped = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Bad clock".to_string()))?
        .as_secs()
        % 3;

    let divided = 100u64
        .checked_div(seconds_wrapped)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "div by 0".to_string()))?;

    Ok(Json(divided))
}
