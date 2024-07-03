use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::domain::repositories::token_repository;
use crate::domain::entities::token::Token;
use diesel::prelude::*;
use crate::schema::tokens;

#[derive(Clone)]
pub struct TokenRepository {}

#[async_trait]
impl token_repository::TokenRepository for TokenRepository {
    async fn get(&self, token: String) -> Result<Vec<Token>, Box<dyn std::error::Error>> {
        Ok(vec![
            Token {
                id: 1,
                token: String::from("ofjeoajweofajw"),
                user_id: Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
                expire_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap()
            }
        ])
    }

    async fn create(&self) -> Result<Token, Box<dyn std::error::Error>> {
        Ok(Token {
            id: 1,
            token: String::from("ofjeoajweofajw"),
            user_id: Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            expire_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap()
        })
    }
} 
