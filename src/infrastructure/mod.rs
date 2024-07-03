use std::sync::Arc;

use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

use crate::adapters::{controllers, repositories::token_repository::TokenRepository, shared::app_state::AppState};
mod database;

pub async fn server(listener: TcpListener, db_name: &str) -> Result<(), std::io::Error> {
    let data = AppState {
        token_repository: Arc::new(TokenRepository {}),
        db_pool: database::get_connection_pool()
    }; 

    let app = Router::new().nest("/", controllers::routes()).with_state(data);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn generate_jwt() -> Result<StatusCode, (StatusCode, String)> {
    Ok(StatusCode::OK)
}
