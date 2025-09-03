use config::Config;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct MyConfig {
    test_toml: String,
    test: String,
}

fn main() {
    // Ignore the result of loading .env --- it's ok if it doesn't exist
    let _ = dotenvy::dotenv();

    let settings_reader = Config::builder()
        .add_source(config::File::with_name("settings").required(false)) // may not exist and it's ok; if exists and .env exists it will run both
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let settings = settings_reader
        // .try_deserialize::<HashMap<String, String>>()
        .try_deserialize::<MyConfig>()
        .unwrap();

    println!("{settings:?}");
}

// cargo add config
// cargo add serde -F derive
// TEST=test cargo run
// APP_TEST=test APP_SECOND=second cargo run
