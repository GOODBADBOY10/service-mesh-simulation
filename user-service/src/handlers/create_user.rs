use std::sync::Arc;
use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
};
use chrono::Utc;
use serde::Serialize;
use crate::auth::authenticate;
use crate::error::AppError;
use crate::models::{CreateProfileRequest, UserProfile};
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct CreateProfileResponse {
    pub user_id: String,
    pub message: String,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<CreateProfileRequest>,
) -> Result<Json<CreateProfileResponse>, AppError> {
    // 1. Authenticate the request
    let _user_info = authenticate(&state, &headers).await?;

    // 2. Lock the profiles HashMap
    let mut profiles = state.profiles.lock()
        .map_err(|_| AppError::InternalError)?;

    // 3. Check if user already exists
    if profiles.contains_key(&req.user_id) {
        return Err(AppError::UserAlreadyExists);
    }

    // 4. Create the profile
    let profile = UserProfile {
        user_id: req.user_id.clone(),
        username: req.username,
        email: req.email,
        full_name: req.full_name,
        bio: req.bio,
        created_at: Utc::now().to_rfc3339(),
    };

    // 5. Store in HashMap
    profiles.insert(req.user_id.clone(), profile);

    // 6. Return success response
    Ok(Json(CreateProfileResponse {
        user_id: req.user_id,
        message: "Profile created successfully".to_string(),
    }))
}