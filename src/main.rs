mod errors;
mod models;

use axum::{extract::Path, routing::get, Json, Router};
use errors::AppError;

const HOST: &str = "0.0.0.0";
const PORT: u16 = 4000;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/artists/{name}", get(get_artists));

    let listener = tokio::net::TcpListener::bind(
        format!("{}:{}", HOST, PORT)
    ).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn get_artists(Path(name): Path<String>) -> Result<Json<models::ArtistSearchResponse>, AppError> {
    // share client in app state.
    let url = format!(
        "https://musicbrainz.org/ws/2/artist/?query={}&fmt=json",
        urlencoding::encode(&name)
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "coda/1.0")
        .send()
        .await?
        .json::<models::ArtistSearchResponse>()
        .await?;

    Ok(Json(response))
}
