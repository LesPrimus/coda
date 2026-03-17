use crate::errors::AppError;
use crate::models;
use crate::models::Artist;
use axum::extract::Path;
use axum::Json;

pub async fn get_artist(Path(name): Path<String>) -> Result<Json<Artist>, AppError> {
    // share client in app state.
    let url = format!(
        "https://musicbrainz.org/ws/2/artist/?query={}&fmt=json",
        urlencoding::encode(&name)
    );

    let client = reqwest::Client::new();
    let artist_search_response = client
        .get(url)
        .header("User-Agent", "coda/1.0")
        .send()
        .await?
        .json::<models::ArtistSearchResponse>()
        .await?;

    let artist = artist_search_response
        .artists
        .into_iter()
        .next()
        .ok_or(anyhow::anyhow!("No artist found for {}", name))?;

    Ok(Json(artist))
}