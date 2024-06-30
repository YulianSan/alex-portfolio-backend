use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone)]
pub struct RefreshToken {
    pub id: i32,
    pub token: String,
    pub user_id: String,
    pub expire_at: String
}
