use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Json,
};
use serde::Serialize;
use crate::auth::authenticate;
use crate::error::AppError;
use crate::models::UpdateProfileRequest;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct UpdateProfileResponse {
    pub user_id: String,
    pub message: String,
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UpdateProfileResponse>, AppError> {
    // 1. Authenticate the request
    let user_info = authenticate(&state, &headers).await?;

    // 2. Check if user is updating their own profile (optional security)
    if user_info.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    // 3. Lock the profiles HashMap
    let mut profiles = state.profiles.lock()
        .map_err(|_| AppError::InternalError)?;

    // 4. Find the profile
    let profile = profiles
        .get_mut(&user_id)
        .ok_or(AppError::UserNotFound)?;

    // 5. Update only the fields that were provided
    if let Some(full_name) = req.full_name {
        profile.full_name = full_name;
    }
    if let Some(bio) = req.bio {
        profile.bio = bio;
    }

    // 6. Return success response
    Ok(Json(UpdateProfileResponse {
        user_id,
        message: "Profile updated successfully".to_string(),
    }))
}