use axum::{Router, body::Body, http::Request, response::Html, routing::get};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    // Setup tracing
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Include per-span timings
        .with_span_events(FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    // Set the subscriber as the default; quick and easy way to override all of the existing setup
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Starting server");

    // Remove the `.layer` call to only show your own spans
    let app =
        Router::new()
            .route("/", get(handler))
            .layer(
                TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                    let request_id = uuid::Uuid::new_v4();
                    tracing::span!(
                        tracing::Level::INFO,
                        "request",
                        method = tracing::field::display(request.method()),
                        uri = tracing::field::display(request.uri()),
                        version = tracing::field::debug(request.version()),
                        request_id = tracing::field::display(request_id)
                    )
                }),
            );
    // .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// create a span when the handler runs and ends the span when the handler finish
// #[instrument]
async fn handler() -> Html<&'static str> {
    info!("Serving Hello World");
    Html("<h1>Hello, World!</h1>")
}

// cargo add uuid -F v4 (v4 was the first version to incorporate automatic random generation)
