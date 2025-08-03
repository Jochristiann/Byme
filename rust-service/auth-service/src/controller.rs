use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::r2d2::State;
use model::users::{NewUser, LoginRequest};
use crate::{service};
use model::state::AppState;
use regex::Regex;
use user_service::service as external_service;
use crate::response::{LoginResponse};

pub async fn register_user(user: NewUser, state: AppState) -> (StatusCode, axum::Json<LoginResponse>) {
    let message;
    let mut status= "Failed".to_string();
    let responses;

    let (passed, validation_result) = validate_password(&user.password.clone());
    if !passed {
        responses = LoginResponse{
            status : status.clone(),
            message: validation_result,
            user: None,
        };

        return (StatusCode::FORBIDDEN,axum::Json(responses))
    }

    let hashed = hash(user.password, DEFAULT_COST).unwrap();
    let user = NewUser {
        password: hashed,
        ..user
    };

    let (response, _) = external_service::get_user_by_email(&state.db, user.email.clone()).await;
    if let Some(user) = response {
        message = "The email is already registered".to_string();
        responses = LoginResponse{
            status,
            message,
            user: Some(user.clone()),
        };

        let mut current = state.current_user.lock().await;
        *current = Some(user);

        return (StatusCode::FORBIDDEN,axum::Json(responses))
    }

    let x = service::insert_new_user(&state.db, user.clone()).await;

    if x {
        let (response, _) = external_service::get_user_by_email(&state.db, user.email).await;
        if let Some(user) = response {
            message = "Successfully registered".to_string();
            status = "Success".to_string();
            responses = LoginResponse{
                status,
                message,
                user: Some(user.clone()),
            };

            let mut current = state.current_user.lock().await;
            *current = Some(user);

            (StatusCode::OK,axum::Json(responses))
        }else{
            message = "User Not Found".to_string();
            responses = LoginResponse{
                status,
                message,
                user: None,
            };
            (StatusCode::NOT_FOUND,axum::Json(responses))
        }
    }else{
        message = "Failed to Register".to_string();
        responses = LoginResponse{
            status,
            message,
            user: None,
        };
        (StatusCode::BAD_REQUEST,axum::Json(responses))
    }
}

pub async fn login_user(login: LoginRequest, state: AppState) -> (StatusCode, axum::Json<LoginResponse>) {
    let (response, password) = external_service::get_user_by_email(&state.db, login.email).await;
    let mut message= "Incorrect Password".to_string() ;
    let mut status= "Failed".to_string();
    let responses;

    if let Some(user) = response {
        if verify(&login.password, &password).unwrap_or(false) {
            message = "Successfully logged in".to_string();
            status = "Success".to_string();
            responses = LoginResponse{
                status,
                message,
                user: Some(user.clone()),
            };

            let mut current = state.current_user.lock().await;
            *current = Some(user);

            return (StatusCode::OK,axum::Json(responses))
        }
        responses = LoginResponse{
            status,
            message,
            user: None,
        };

        (StatusCode::UNAUTHORIZED,axum::Json(responses))
    }else{
        message = "User Not Found".to_string();
        responses = LoginResponse{
            status,
            message,
            user: None,
        };
        (StatusCode::NOT_FOUND,axum::Json(responses))
    }
}

pub async fn login_by_google() -> (StatusCode, axum::Json<String>){
    (StatusCode::BAD_REQUEST,axum::Json("Failed to login".to_string()))
}

pub async fn change_password(id: String, new_password:String, state: AppState)-> (StatusCode, axum::Json<String>){
    let message;
    let mut status= "Failed".to_string();
    let responses;
    let (response, password) = external_service::get_user_by_id(&state.db, id).await;


    (StatusCode::BAD_REQUEST,axum::Json("Failed to change password".to_string()))
}

pub async fn forgot_password(email: String, new_password:String, state: AppState)-> (StatusCode, axum::Json<String>){
    let (response, password) = external_service::get_user_by_email(&state.db, email).await;


    (StatusCode::BAD_REQUEST,axum::Json("Failed to change password".to_string()))
}

fn validate_password(password:&String) -> (bool, String){
    if password.clone().len() < 6 {return (false, "Password too short".to_string())}
    let has_uppercase = Regex::new(r"[A-Z]").unwrap();
    let has_number = Regex::new(r"\d").unwrap();
    let has_special = Regex::new(r"[!@#$%^&*(),.?|:{}<>]").unwrap();

    if !has_uppercase.is_match(password){return (false, "Password must contain uppercase letter".to_string());}
    else if!has_number.is_match(password){return (true, "Password must contain at least one number".to_string());}
    else if!has_special.is_match(password) { return (false, "Password must has at least one special character.".to_string()); }

    (true, "Password accepted".to_string())
}
