use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::HeaderMap,
    response::Json,
};
use serde::Serialize;
use crate::auth::authenticate;
use crate::error::AppError;
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct DeleteProfileResponse {
    pub user_id: String,
    pub message: String,
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(user_id): Path<String>,
) -> Result<Json<DeleteProfileResponse>, AppError> {
    // 1. Authenticate the request
    let user_info = authenticate(&state, &headers).await?;

    // 2. Check if user is deleting their own profile
    if user_info.user_id != user_id {
        return Err(AppError::Forbidden);
    }

    // 3. Lock the profiles HashMap
    let mut profiles = state.profiles.lock()
        .map_err(|_| AppError::InternalError)?;

    // 4. Remove the profile
    profiles
        .remove(&user_id)
        .ok_or(AppError::UserNotFound)?;

    // 5. Return success response
    Ok(Json(DeleteProfileResponse {
        user_id,
        message: "Profile deleted successfully".to_string(),
    }))
}