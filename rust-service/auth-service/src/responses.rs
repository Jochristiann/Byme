use serde::{Deserialize, Serialize};
use model::users::UserResponse;

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
    pub status: String,
    pub message: String,
    pub user: Option<UserResponse>,
    pub token: Option<String>
}

#[derive(Serialize,Deserialize)]
pub struct ForgotPasswordRequest{
    pub email: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct ResetQuery {
    pub token: String,
}