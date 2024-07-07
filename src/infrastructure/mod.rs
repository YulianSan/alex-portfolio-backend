use std::sync::Arc;

use axum::{http::StatusCode, Router};
use tokio::net::TcpListener;

use crate::adapters::{controllers, repositories::{token_repository::TokenRepository, user_repository::UserRepository}, shared::app_state::AppState};
mod database;

pub async fn server(listener: TcpListener, db_name: &str) -> Result<(), std::io::Error> {
    let pool = database::get_connection_pool();

    let data = AppState {
        token_repository: Arc::new(TokenRepository {}),
        user_repository: Arc::new(UserRepository { db_connection: pool.clone() }),
        db_pool: pool
    }; 

    let app = Router::new().nest("/", controllers::routes()).with_state(data);

    axum::serve(listener, app).await?;
    Ok(())
}

async fn generate_jwt() -> Result<StatusCode, (StatusCode, String)> {
    Ok(StatusCode::OK)
}
