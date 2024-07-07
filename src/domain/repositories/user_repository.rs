use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn create(&self) -> Result<User, Box<dyn std::error::Error>>;
    // async fn get(&self, uuid: Uuid) -> Result<Vec<User>, Box<dyn std::error::Error>>;
}
