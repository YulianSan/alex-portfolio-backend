use async_trait::async_trait;
use crate::domain::repositories::refresh_token_repository;
use crate::domain::entities::refresh_token::RefreshToken;

#[derive(Clone)]
pub struct RefreshTokenRepository {}

#[async_trait]
impl refresh_token_repository::RefreshTokenRepository for RefreshTokenRepository {
    async fn get(&self) -> Result<Vec<RefreshToken>, Box<dyn std::error::Error>> {
        Ok(vec![
            RefreshToken {
                id: 1,
                token: String::from("ofjeoajweofajw"),
                user_id: String::from("fjsofjiasfjdsa"),
                expire_at: String::from("2005-03-13 00:00:00")
            }
        ])
    }

    async fn create(&self) -> Result<RefreshToken, Box<dyn std::error::Error>> {
        Ok(RefreshToken {
            id: 1,
            token: String::from("ofjeoajweofajw"),
            user_id: String::from("fjsofjiasfjdsa"),
            expire_at: String::from("2005-03-13 00:00:00")
        })
    }
} 
