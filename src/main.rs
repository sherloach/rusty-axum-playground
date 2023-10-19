use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static()); // if the path doesn't match, serve static files

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(".")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello, {name}"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, {name}"))
}
