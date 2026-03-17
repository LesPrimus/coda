use crate::errors::AppError;
use crate::models::{self, AppState, Artist};
use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

pub async fn get_artist(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<Artist>, AppError> {
    let url = format!(
        "{}/artist/?query={}&fmt=json",
        state.base_url,
        urlencoding::encode(&name)
    );

    let artist_search_response = state
        .client
        .get(url)
        .header("User-Agent", state.user_agent.clone())
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use rstest::*;

    #[fixture]
    fn artist_json() -> String {
        serde_json::json!({
            "artists": [
                { "id": "1", "name": "Radiohead", "score": 100 }
            ]
        })
            .to_string()
    }

    #[fixture]
    fn empty_artists_json() -> String {
        serde_json::json!({ "artists": [] }).to_string()
    }

    fn make_state(base_url: String) -> State<Arc<AppState>> {
        State(Arc::new(AppState {
            client: reqwest::Client::new(),
            base_url,
            user_agent: "coda/1.0".to_string(),
        }))
    }

    #[rstest]
    #[tokio::test]
    async fn test_returns_first_artist(artist_json: String) {
        let mut server = Server::new_async().await;
        server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&artist_json)
            .create_async()
            .await;

        let result = get_artist(make_state(server.url()), Path("radiohead".to_string())).await;

        let Json(artist) = result.unwrap();
        assert_eq!(artist.name, "Radiohead");
        assert_eq!(artist.id, "1");
    }

    #[rstest]
    #[tokio::test]
    async fn test_errors_when_no_artists(empty_artists_json: String) {
        let mut server = Server::new_async().await;
        server
            .mock("GET", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&empty_artists_json)
            .create_async()
            .await;

        let result = get_artist(
            make_state(server.url()),
            Path("unknown_xyz".to_string()),
        )
            .await;

        assert!(result.is_err());
    }
}
