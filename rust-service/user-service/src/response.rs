use serde::{Deserialize, Serialize};
use model::users::UserResponse;

#[derive(Serialize,Deserialize)]
pub struct AllUserResponse{
    pub status: String,
    pub message: String,
    pub user: Option<Vec<UserResponse>>
}