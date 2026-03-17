pub struct AppState {
    pub client: reqwest::Client,
    pub base_url: String,
    pub user_agent: String,
}
