use uuid::Uuid;
use model::state::DbPool;
use model::users::{NewUser, RegisterUsers};
use crate::{repository};

pub async fn insert_new_user(pool: &DbPool, new_user:NewUser) -> bool{
    let user = RegisterUsers{
        id : Uuid::new_v4(),
        name : new_user.name,
        email : new_user.email,
        password : new_user.password,
    };
    
    repository::insert_user(pool,&user).await
}



pub async fn change_password(pool: &DbPool, id:String, new_password:String) -> bool{
    repository::change_password(pool, id, new_password).await
}