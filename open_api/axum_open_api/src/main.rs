use axum::{Json, Router, routing::get};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            handler,
        ),
        components(
            schemas(HelloWorld)
        ),
        modifiers(),
        tags(
            (name = "Test System", description = "A really simple API")
        )
    )]
    struct ApiDoc;

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, ToSchema)]
struct HelloWorld {
    message: String,
    second: String,
    third: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Say Hello to the World", body = [HelloWorld])
    )
)]
async fn handler() -> Json<HelloWorld> {
    Json(HelloWorld {
        message: "Hello, World!".to_string(),
        second: "Hello, World Again!".to_string(),
        third: "Hello, World Again and Again!".to_string(),
    })
}

// cargo addd utoipa
// cargo addd utoipa-redoc -F axum
// cargo addd utoipa-swagger-ui -F axum
