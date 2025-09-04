use axum::{Router, response::Html, routing::get};
use clap::{Arg, Command, value_parser};

#[tokio::main]
async fn main() {
    let matches = Command::new("simple_http_server") // name of the program
        .version("0.1.0")
        .author("Herbert")
        .subcommand(
            Command::new("serve")
                .about("Starts the server") // what do command does
                .arg(
                    Arg::new("address")
                        .short('a')
                        .long("address")
                        .value_name("ADDRESS")
                        .help("Sets the IP address to bind to"),
                )
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("Sets the port to bind to")
                        .value_parser(value_parser!(u16)),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("serve") {
        let address: String = matches
            .get_one("address")
            .cloned()
            .unwrap_or("127.0.0.1".to_string()); // if not specify the address it will bind to localhost as default

        let port: u16 = *matches.get_one("port").unwrap_or(&3001);
        let bind_address = format!("{}:{}", address, port);

        serve(&bind_address).await;
    } else {
        println!("Run with --help for details");
    }
}

async fn serve(bind_address: &str) {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

    println!("listening on {}", bind_address);
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

// clap -> command line argument processors
// cargo run -- --help => cargo accepts cli parameters so to use the clap ones need the double dash empty to cargo undestand
// cargo run -- serve
// cargo run -- serve --help
// cargo run -- serve -a 0.0.0.0 -p 3002
