use super::constants;
use std::env;

use super::routes::{book, home};
use axum::{routing::get, Router};

#[tokio::main]
pub async fn main() {
    let http_address = env::var(constants::HTTP_ADDRESS).unwrap();
    let http_port = env::var(constants::HTTP_PORT).unwrap();
    let address_port = http_address + ":" + &http_port;

    let app = Router::new()
        .route("/", get(home))
        .route("/book", get(book));
    let listener = tokio::net::TcpListener::bind(&address_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
