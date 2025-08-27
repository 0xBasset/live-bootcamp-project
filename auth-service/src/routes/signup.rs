use core::str;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User},
    services::hashmap_user_store::UserStoreError,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email.clone();
    let password = request.password.clone();

    if email.is_empty() || !email.contains("@") || password.len() < 8 {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(request.email, request.password, request.require_2fa);
    let mut user_store = state.user_store.write().await;

    let result = user_store.add_user(user);
    match result {
        Ok(()) => {}
        Err(UserStoreError::UserAlreadyExists) => return Err(AuthAPIError::UserAlreadyExists),
        _ => return Err(AuthAPIError::UnexpectedError),
    }

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub require_2fa: bool,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}
