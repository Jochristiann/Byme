use axum::http::StatusCode;
use model::state::{AppState};
use axum::Json;
use model::users::UserResponse;
use crate::response::AllUserResponse;
use crate::service;

pub async fn by_id(state:AppState, id:String)-> (StatusCode, Json<Option<UserResponse>>){
    let (response,_)= service::get_user_by_id(&state.db, id.clone()).await;

    if let Some(user) = response{
        (StatusCode::OK,Json(Some(user)))
    }else {
        (StatusCode::NOT_FOUND,Json(None))
    }
}

pub async fn by_email(state:AppState,email:String)-> (StatusCode, Json<Option<UserResponse>>){
    let (response,_)= service::get_user_by_email(&state.db, email.clone()).await;

    if let Some(user) = response{
        (StatusCode::OK,Json(Some(user)))
    }else {
        (StatusCode::NOT_FOUND,Json(None))
    }
}

pub async fn get_all_user_by_role (state:AppState, role:String) -> (StatusCode, Json<AllUserResponse>){

    let response= service::get_all_user_by_role(&state.db, role.clone()).await;
    let responses;

    if let Some(user) = response{
        responses= AllUserResponse{
            status: "Success".to_string(),
            message: "Success to get all user by role. Role: ".to_string() + &role,
            user: Some(user),
        };
        (StatusCode::OK,Json(responses))
    }else {
         responses= AllUserResponse{
            status: "Failed".to_string(),
            message: "Failed to get all user by role".to_string(),
            user: None,
         };
        (StatusCode::NOT_FOUND,Json(responses))
    }
}