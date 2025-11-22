use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::UserProfile;

pub struct AppState {
    // Storage for user profiles
    pub profiles: Mutex<HashMap<String, UserProfile>>,
    
    // HTTP client for calling auth-service (reusable, connection pooling)
    pub http_client: reqwest::Client,
    
    // Base URL of auth-service
    pub auth_service_url: String,
}

impl AppState {
    pub fn new(auth_service_url: String) -> Self {
        Self {
            profiles: Mutex::new(HashMap::new()),
            http_client: reqwest::Client::new(),
            auth_service_url,
        }
    }
}