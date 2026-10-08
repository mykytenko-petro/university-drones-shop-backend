pub mod core;
pub mod features;

use core::config::{HOST, PORT};

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let url = format!("{}:{}", HOST, PORT);

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();

    println!("running derver on: http://{}", url);

    axum::serve(listener, router).await.unwrap();
}