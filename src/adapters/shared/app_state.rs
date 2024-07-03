use std::sync::Arc;
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

use crate::domain::repositories::token_repository::TokenRepository;

#[derive(Clone)]
pub struct AppState {
    pub token_repository: Arc<dyn TokenRepository>,
    pub db_pool: Pool<ConnectionManager<PgConnection>>
}
