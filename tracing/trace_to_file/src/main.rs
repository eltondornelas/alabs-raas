use tracing::info;

fn main() {
    // Setup tracing
    // let file_appender = tracing_appender::rolling::hourly("test.log", "prefix.log"); // once per hour
    let file_appender = tracing_appender::rolling::hourly("test_json.log", "prefix.log"); // once per hour
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_writer(non_blocking)
        // Build the subscriber
        .init();

    info!("Starting server");
}

// cargo add tracing_appender
// cat test.log/prefix...

// cargo add tracing_subscriber -F json
