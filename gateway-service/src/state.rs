pub struct AppState {
    pub http_client: reqwest::Client,
    pub auth_service_url: String,
    pub user_service_url: String,
}

impl AppState {
    pub fn new(auth_service_url: String, user_service_url: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            auth_service_url,
            user_service_url,
        }
    }
}