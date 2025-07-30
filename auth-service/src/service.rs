use uuid::Uuid;
use model::state::DbPool;
use model::users::{NewUser, Users};
use crate::repository;

pub async fn insert_new_user(pool: &DbPool, new_user:NewUser) -> bool{
    let user = Users{
        id : Uuid::new_v4(),
        name : new_user.name,
        email : new_user.email,
        password : new_user.password,
        dob: None,
        image: None,
        role : "User".to_string(),
        origin_id: None,
        created_at: Default::default(),
        updated_at: Default::default(),
    };
    
    repository::insert_user(pool,&user).await
}