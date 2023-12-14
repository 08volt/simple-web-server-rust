#![allow(unused)]

mod error;
mod web;

pub use self::error::{Error, Result};

use std::os::unix::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(routes_static());

    // region --- Start Server ---
    println!("-- LISTENING ON http://127.0.0.1:8080 --");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region -- Routes Hello
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
    println!(" --> {:<12} - handler_hello - {params:?}", "Handler");
    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>Hello {name}!</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!(" --> {:<12} - handler_hello - {name}", "Handler");
    Html(format!("Hello <strong>Hello {name}!</strong>"))
}
