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

impl ArtistSearchResponse {
    pub fn iter(&self) -> std::slice::Iter<'_, Artist> {
        self.artists.iter()
    }
}

impl<'a> IntoIterator for &'a ArtistSearchResponse {
    type Item = &'a Artist;
    type IntoIter = std::slice::Iter<'a, Artist>;

    fn into_iter(self) -> Self::IntoIter {
        self.artists.iter()
    }
}