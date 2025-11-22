use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Json,
};
use crate::auth::authenticate;
use crate::error::AppError;
use crate::models::UserProfile;
use crate::state::AppState;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
) -> Result<Json<UserProfile>, AppError> {
    // 1. Authenticate the request (validate token)
    let _user_info = authenticate(&state, &headers).await?;

    // 2. Lock the profiles HashMap
    let profiles = state.profiles.lock()
        .map_err(|_| AppError::InternalError)?;

    // 3. Find the user profile
    let profile = profiles
        .get(&user_id)
        .ok_or(AppError::UserNotFound)?;

    // 4. Return the profile (clone because we can't return reference)
    Ok(Json(profile.clone()))
}