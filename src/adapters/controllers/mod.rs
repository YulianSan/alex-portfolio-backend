use axum::Router;

use super::shared::app_state::AppState;

mod auth;
mod user;

pub fn routes() -> Router<AppState> {
    axum::Router::new().nest("/auth", auth::routes())
        .nest("/user", user::routes())
}
