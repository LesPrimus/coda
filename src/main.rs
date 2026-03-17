mod errors;
mod models;
mod handlers;

use axum::{routing::get, Router};
use models::AppState;
use std::sync::Arc;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 4000;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        client: reqwest::Client::new(),
        base_url: "https://musicbrainz.org/ws/2".to_string(),
        user_agent: "coda/1.0".to_string(),
    });

    let app = Router::new()
        .route("/artists/{name}", get(handlers::artists::get_artist))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", HOST, PORT)
    ).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
