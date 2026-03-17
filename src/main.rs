mod errors;
mod models;
mod handlers;

use axum::{routing::get, Router};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 4000;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/artists/{name}", get(handlers::artists::get_artist));

    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", HOST, PORT)
    ).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
