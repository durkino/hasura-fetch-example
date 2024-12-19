use std::net::SocketAddr;

use axum::{routing::get, Json};
use utoipa::OpenApi;
use tower_http::compression::CompressionLayer;

#[derive(OpenApi)]
#[openapi(paths(openapi,helloworld))]
struct ApiDoc;

/// Return JSON version of an OpenAPI schema
#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = String)
    )
)]
async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

// Return hello world!
#[utoipa::path(
    get,
    path = "/helloworld",
    responses(
        (status=200, description = "Hello world!", body = String)
    )
)]
async fn helloworld() -> String {
    return String::from("Hello world! ABCDEFGHIJKLMNOPQRSTUVWXYZ.");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut first_arg = "";
    if args.len() > 1 {
        first_arg = &args[1];
    }

    let socket_address: SocketAddr = "127.0.0.1:55555".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let mut app = axum::Router::new()
        .route("/api-docs/openapi.json", get(openapi))
        .route("/helloworld", get(helloworld));

    // testing to see if server breaks when gzip middleware is present
    // trigger when any command line argument is provided
    // usage: cargo run gzip
    if first_arg == "gzip" {
        println!("Using compressed responses!");
        app = app.layer(CompressionLayer::new()); 
    }
    else {
        println!("Not using compressed responses!");
    }

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}