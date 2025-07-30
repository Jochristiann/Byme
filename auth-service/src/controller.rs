use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use model::users::{NewUser,LoginRequest};
use crate::{service};
use model::state::AppState;
use crate::response::{LoginResponse, UserResponse};

pub async fn register_user(user: NewUser, state: AppState) -> axum::Json<String> {
    let hashed = hash(user.password, DEFAULT_COST).unwrap();
    let user = NewUser {
        password: hashed,
        ..user
    };
    service::insert_new_user(&state.db, user).await;
    axum::Json("User registered successfully".to_string())
}

pub async fn login_user(login: LoginRequest, state: AppState) -> (StatusCode, axum::Json<LoginResponse>) {
    let (response, password) = service::login_user(&state.db, login.email).await;
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
                user: Some(user),
            };

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

pub async fn login_by_google() -> axum::Json<String>{
    axum::Json("Failed to login".to_string())
}
