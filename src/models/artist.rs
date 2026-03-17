use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub score: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistSearchResponse {
    pub artists: Vec<Artist>,
}