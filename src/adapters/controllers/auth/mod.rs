use axum::{routing::post, Router};

use crate::adapters::shared::app_state::AppState;

mod auth_signin_controller;

pub fn routes() -> Router<AppState> {
    axum::Router::new().route("/login", post(auth_signin_controller::signin))
}
