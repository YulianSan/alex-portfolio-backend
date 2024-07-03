use axum::{extract::State, http::StatusCode, Json};

use crate::{
    adapters::shared::app_state::AppState,
    domain::{
        entities::token::Token,
        repositories::token_repository::TokenRepository,
    },
};

struct Payload {
    email: String,
    password: String,
}

pub async fn signin(
    State(data): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Token>>), (StatusCode, String)> {
    match data.token_repository.get(String::from("test")).await {
        Ok(tokens) => Ok((StatusCode::OK, Json(tokens))),
        Err(_) => Err((StatusCode::BAD_REQUEST, String::from("erro"))),
    }
}
