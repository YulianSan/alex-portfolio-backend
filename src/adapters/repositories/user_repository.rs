use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::{entities::user::User, repositories::user_repository};

pub struct UserRepository {}

#[async_trait]
impl user_repository::UserRepository for UserRepository {
    async fn create(&self) -> Result<crate::domain::entities::user::User, Box<dyn std::error::Error>> {
        Ok(User {
            id: Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            name: String::from("Yulian"),
            email: String::from("yulian@gmail.com"),
            password: String::from("132"),
            updated_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
            created_at: NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap(),
        })
    }
}
