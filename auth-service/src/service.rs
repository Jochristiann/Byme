use uuid::Uuid;
use model::state::DbPool;
use model::users::{NewUser, RegisterUsers, UserResponse, Users};
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

pub async fn login_user(pool: &DbPool, email_input:String) -> (Option<UserResponse>, String){

    let response = repository::get_user_by_email(pool,email_input).await;
    if let Some(user) = response{
        let converted_user = UserResponse{
            id: user.id,
            name: user.name,
            gender: user.gender,
            email:  user.email,
            dob: user.dob,
            image: user.image,
            role: user.role,
            origin_id: user.originid,
            created_at: user.created_at,
            updated_at: user.updated_at
        };
        return (Option::from(converted_user), user.password)
    }

    (None,"".to_string())
}