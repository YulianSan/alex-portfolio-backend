use std::sync::Arc;

use crate::domain::repositories::refresh_token_repository::RefreshTokenRepository;

#[derive(Clone)]
pub struct AppState {
    pub refresh_token_repository: Arc<dyn RefreshTokenRepository>
}
