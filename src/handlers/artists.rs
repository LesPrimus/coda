use crate::errors::AppError;
use crate::models::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde_json::Value;
use std::sync::Arc;


pub async fn get_album_tracks(
    State(state): State<Arc<AppState>>,
    Path((artist, album)): Path<(String, String)>,
) -> Result<Json<Value>, AppError> {
    // Step 1: search for the release
    let search_url = format!(
        "{}/release/?query=release:{} AND artist:{}&fmt=json&limit=1",
        state.base_url,
        urlencoding::encode(&album),
        urlencoding::encode(&artist),
    );

    let search: Value = state
        .client
        .get(&search_url)
        .header("User-Agent", &state.user_agent)
        .send()
        .await?
        .json()
        .await?;

    let release_id = search["releases"][0]["id"]
        .as_str()
        .ok_or(anyhow::anyhow!("No release found"))?;

    // Step 2: fetch recordings
    let release_url = format!(
        "{}/release/{}?inc=recordings&fmt=json",
        state.base_url, release_id
    );

    let release: Value = state
        .client
        .get(&release_url)
        .header("User-Agent", &state.user_agent)
        .send()
        .await?
        .json()
        .await?;

    let tracks: Vec<&Value> = release["media"]
        .as_array()
        .ok_or(anyhow::anyhow!("No media found"))?
        .iter()
        .flat_map(|medium| medium["tracks"].as_array().into_iter().flatten())
        .collect();

    Ok(Json(serde_json::json!(tracks)))
}
