#![allow(unused)]

use std::os::unix::net::SocketAddr;

use axum::Router;
use axum::response::{Html, IntoResponse};
use axum::routing::get;

#[tokio::main]
async fn main() {

    let app = Router::new().route("/hello", 
    get(handler_hello));

    println!("-- LISTENING ON http://127.0.0.1:8080 --");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

 

async fn handler_hello() -> impl IntoResponse {
    println!(" --> {:<12} - handler_hello", "Handler");
    Html("Hello <strong>Hello World!!</strong>")
}