use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserCreate {
    name: String,
    email: String,
    password: String
}

pub async fn create(Json(user): Json<UserCreate>) -> Result<StatusCode, StatusCode>{
    Ok(StatusCode::OK)
}
