use async_trait::async_trait;

use crate::domain::entities::refresh_token::RefreshToken;

#[async_trait]
pub trait RefreshTokenRepository: Sync + Send {
    async fn create(&self) -> Result<RefreshToken, Box<dyn std::error::Error>>;
    async fn get(&self) -> Result<Vec<RefreshToken>, Box<dyn std::error::Error>>;
}
