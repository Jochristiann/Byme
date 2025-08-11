use model::accessible::parse_id;
use model::master::Origins;
use model::state::DbPool;
use model::users::{UserResponse, Users};
use crate::repository;

pub async fn get_user_by_id(pool: &DbPool, id:String) -> (Option<UserResponse>, String, String) {
    let parsed_id = parse_id(id);
    let response = repository::get_user_by_id(pool,parsed_id).await;
    if let Some((user,origin)) = response{
        let converted_user = convert_user_to_user_res(user.clone(),origin.clone());
        return (Option::from(converted_user), user.id.to_string(), user.password)
    }

    (None,"".to_string(),"".to_string())
}

pub async fn get_user_by_email(pool: &DbPool, email_input:String) -> (Option<UserResponse>, String, String){
    let response = repository::get_user_by_email(pool,email_input).await;
    if let Some((user,origin)) = response{
        let converted_user = convert_user_to_user_res(user.clone(), origin.clone());
        return (Option::from(converted_user),user.id.to_string(), user.password)
    }

    (None,"".to_string(),"".to_string())
}

pub async fn get_all_user_by_role(pool: &DbPool, role:String) -> Option<Vec<UserResponse>>{
    let response = repository::get_user_by_role(pool,role).await;
    if let Some(users) = response{
        let mut converted_users: Vec<UserResponse> = Vec::new();

        for (user,origin) in users {
            let converted_user = convert_user_to_user_res(user.clone(), Some(origin.clone()));
            converted_users.push(converted_user);
        }

        return Some(converted_users)
    }
    None
}

pub fn convert_user_to_user_res(user:Users, origins: Option<Origins>) -> UserResponse
{
    UserResponse{
        name: user.name,
        gender: user.gender,
        dob: user.dob,
        email:  user.email,
        bio: user.bio,
        image: user.image,
        role: user.role,
        isverified: user.isverified,
        origin: origins,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}