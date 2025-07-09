use uuid::Uuid;
use crate::{model::users::{Users, NewUser}, AppState};
use crate::auth_services::repository;
use crate::model::users::RegisterUsers;

pub async fn insert_new_user(new_user:NewUser) -> bool{
    let user = RegisterUsers{
        id : Uuid::new_v4(),
        name : new_user.name,
        email : new_user.email,
        password : new_user.password,
        role : "User".to_string(),
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    
    repository::insert_user(&user);
    true
}