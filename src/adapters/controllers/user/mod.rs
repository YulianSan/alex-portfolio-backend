use axum::{routing::post, Router};

use crate::adapters::shared::app_state::AppState;

mod user_create_controller;

pub fn routes() -> Router<AppState> {
    axum::Router::new().route("/", post(user_create_controller::create))
}
