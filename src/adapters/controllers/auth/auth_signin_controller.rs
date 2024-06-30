use axum::{extract::State, http::StatusCode, Json};

use crate::{adapters::shared::app_state::AppState, domain::{entities::refresh_token::RefreshToken, repositories::refresh_token_repository::RefreshTokenRepository}};

struct Payload {
    email: String,
    password: String,
}

pub async fn signin(State(data): State<AppState>) -> Result<(StatusCode, Json<Vec<RefreshToken>>), (StatusCode, String)> {
    match data.refresh_token_repository.get().await {
        Ok(refresh_tokens) => Ok(
            (StatusCode::OK, Json(refresh_tokens)),
        ),
        Err(_) => Err((StatusCode::BAD_REQUEST, String::from("erro")))
    }
}
