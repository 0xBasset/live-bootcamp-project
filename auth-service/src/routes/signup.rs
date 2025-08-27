use core::str;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    let user = User::new(request.email, request.password, request.require_2fa);

    let mut user_store = state.user_store.write().await;

    let result = user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User Create successfully".to_string(),
    });

    (StatusCode::CREATED, response)
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
