use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct UserResponse{
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub dob: Option<chrono::NaiveDate>,
    pub image: Option<String>,
    pub role: String,
    pub origin_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
    pub status: String,
    pub message: String,
    pub user: Option<UserResponse>
}