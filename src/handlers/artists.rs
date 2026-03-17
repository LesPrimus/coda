use axum::extract::Path;
use axum::Json;
use crate::errors::AppError;
use crate::models;

pub async fn get_artists(Path(name): Path<String>) -> Result<Json<models::ArtistSearchResponse>, AppError> {
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