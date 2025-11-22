use serde::{Deserialize, Serialize};

// What we store in our database (HashMap)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub bio: String,
    pub created_at: String,
}

// Request body for creating/updating a profile
#[derive(Debug, Deserialize)]
pub struct CreateProfileRequest {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub bio: String,
}

// Request body for updating a profile
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub full_name: Option<String>,
    pub bio: Option<String>,
}

// What we send to auth-service to validate token
#[derive(Debug, Serialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

// What auth-service returns after validation
#[derive(Debug, Deserialize)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub user_id: String,
    // pub username: String,
}