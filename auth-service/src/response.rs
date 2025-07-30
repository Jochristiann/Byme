use serde::{Deserialize, Serialize};
use model::users::UserResponse;

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
    pub status: String,
    pub message: String,
    pub user: Option<UserResponse>
}