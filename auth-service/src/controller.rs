use axum::extract::State;
use bcrypt::{hash, verify, DEFAULT_COST};
use model::users::{NewUser,LoginRequest};
use crate::service;
use model::state::AppState;

pub async fn register_user(user: NewUser, state: AppState) -> axum::Json<String> {
    let hashed = hash(user.password, DEFAULT_COST).unwrap();
    let user = NewUser {
        password: hashed,
        ..user
    };
    service::insert_new_user(&state.db, user).await;
    axum::Json("User registered successfully".to_string())
}

pub async fn login_user(login: LoginRequest) -> axum::Json<String> {
    let found_user = service::login_user(&login.email).await;
    if let Some(user) = found_user {
        if verify(&login.password, &user.password).unwrap() {
            return axum::Json("Login successful".to_string());
        }
    }
    axum::Json("Invalid credentials".to_string())
}

pub async fn login_by_google() -> axum::Json<String>{
    axum::Json("Failed to login".to_string())
}
