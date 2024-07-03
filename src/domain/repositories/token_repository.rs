use async_trait::async_trait;

use crate::domain::entities::token::Token;

#[async_trait]
pub trait TokenRepository: Sync + Send {
    async fn create(&self) -> Result<Token, Box<dyn std::error::Error>>;
    async fn get(&self, token: String) -> Result<Vec<Token>, Box<dyn std::error::Error>>;
}
