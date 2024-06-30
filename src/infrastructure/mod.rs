use std::sync::Arc;

use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;
use crate::adapters::{controllers, repositories::refresh_token_repository::RefreshTokenRepository, shared::app_state::AppState};

pub async fn server(listener: TcpListener, db_name: &str) -> Result<(), std::io::Error> {
    let data = AppState {
        refresh_token_repository: Arc::new(RefreshTokenRepository {})
    }; 

    let app = Router::new().nest("/", controllers::routes()).with_state(data);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn generate_jwt() -> Result<StatusCode, (StatusCode, String)> {
    Ok(StatusCode::OK)
}
