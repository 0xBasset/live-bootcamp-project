use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie, CookieJar};

use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    State(state): State<AppState>,
    mut jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let cookie = match jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    match validate_token(&token, state.banned_token_store.clone()).await {
        Ok(_) => {
            jar = jar.remove(cookie::Cookie::from(JWT_COOKIE_NAME));

            let mut store = state.banned_token_store.write().await;
            if store.store(token).await.is_err() {
                return (jar, Err(AuthAPIError::UnexpectedError));
            }

            return (jar, Ok(StatusCode::OK));
        }
        Err(_) => return (jar, Err(AuthAPIError::InvalidToken)),
    }
}
