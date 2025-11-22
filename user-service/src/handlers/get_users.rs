use std::sync::Arc;
use axum::{
    extract::State,
    http::HeaderMap,
    response::Json,
};
use crate::auth::authenticate;
use crate::error::AppError;
use crate::models::UserProfile;
use crate::state::AppState;

pub async fn get_users(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<UserProfile>>, AppError> {
    // 1. Authenticate the request
    let _user_info = authenticate(&state, &headers).await?;

    // 2. Lock the profiles HashMap
    let profiles = state.profiles.lock()
        .map_err(|_| AppError::InternalError)?;

    // 3. Collect all profiles into a Vec
    let all_profiles: Vec<UserProfile> = profiles
        .values()
        .cloned()
        .collect();

    // 4. Return all profiles
    Ok(Json(all_profiles))
}