use std::env;
use axum::http::StatusCode;
use lettre::{Message, SmtpTransport, Transport};
use argon2::{
    Argon2, PasswordHash,
};
use axum::extract::State;
use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{Duration, Utc};
use lettre::transport::smtp::authentication::Credentials;
use password_hash::{ PasswordHasher, PasswordVerifier, SaltString};
use rand_core::OsRng;
use model::users::{LoginRequest, RegisterUsers, UserResponse};
use crate::{service};
use model::state::{AppState, AuthState, UserState};
use regex::Regex;
use user_service::service as external_service;
use crate::response::{LoginResponse, ResetQuery};

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed = argon2
        .hash_password(password.as_bytes(), &salt).unwrap().to_string();

    hashed
}

fn verify_password(hashed_password: &str, plain_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(&*hashed_password).unwrap();
    let result = Argon2::default().verify_password(plain_password.as_bytes(), &parsed_hash);
    result.is_ok()
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

fn send_email(recipient_email:String, subject:String, body_message:String) -> () {
    let name = env::var("NAME").expect("NAME must be set");
    let cred_email = env::var("EMAIL").expect("EMAIL must be set");
    let app_password = env::var("EMAIL_APP_PASSWORD").expect("APP_PASSWORD must be set");
    let froms = format!("{} <{}>", name, cred_email.clone());
    let tos = format!("Recipient <{}>",  recipient_email.clone());

    let email = Message::builder()
        .from(froms.parse().unwrap())
        .to(tos.parse().unwrap())
        .subject(subject)
        .body(String::from(body_message))
        .unwrap();

    let creds = Credentials::new(cred_email, app_password.to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("✅ Email sent successfully!"),
        Err(e) => eprintln!("❌ Failed to send email: {e}"),
    }
}

pub async fn get_current_user(state: AuthState) -> (StatusCode, Json<Option<UserResponse>>) {
    let current = state.user_state.current_user.lock().await;
    if let Some(user) = &*current {
        let converted_user = user.clone();
        (StatusCode::OK,Json(Some(converted_user)))
    } else {
        (StatusCode::UNAUTHORIZED, Json(None))
    }
}

pub async fn logout(state: AuthState) -> (StatusCode, Json<String>) {
    let mut current = state.user_state.current_user.lock().await;
    if current.is_some() {
        *current = None;
        (StatusCode::OK, Json("Successfully logged out".to_string()))
    } else {
        (StatusCode::UNAUTHORIZED, Json("Not logged in".to_string()))
    }
}

pub async fn register_user(user: RegisterUsers, state: AppState, user_state: UserState) -> (StatusCode, Json<LoginResponse>) {
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

        return (StatusCode::FORBIDDEN,Json(responses))
    }

    let hashed = hash_password(user.password.as_str());
    let user = RegisterUsers {
        name: user.name.clone(),
        email: user.email.clone(),
        password: hashed,

    };

    let (response, _) = external_service::get_user_by_email(&state.db, user.email.clone()).await;

    if let Some(user) = response {
        message = "The email is already registered".to_string();
        responses = LoginResponse{
            status,
            message,
            user: Some(user.clone()),
        };

        let mut current = user_state.current_user.lock().await;
        *current = Some(user);

        return (StatusCode::FORBIDDEN,Json(responses))
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

            let mut current = user_state.current_user.lock().await;
            *current = Some(user);

            (StatusCode::OK,Json(responses))
        }else{
            message = "User Not Found".to_string();
            responses = LoginResponse{
                status,
                message,
                user: None,
            };
            (StatusCode::NOT_FOUND,Json(responses))
        }
    }else{
        message = "Failed to Register".to_string();
        responses = LoginResponse{
            status,
            message,
            user: None,
        };
        (StatusCode::BAD_REQUEST,Json(responses))
    }
}

pub async fn login_user(login: LoginRequest, state: AppState, user_state: UserState) -> (StatusCode, Json<LoginResponse>) {
    let (response, hashed_password) = external_service::get_user_by_email(&state.db, login.email).await;
    let mut message= "Incorrect Password".to_string() ;
    let mut status= "Failed".to_string();
    let responses;

    if let Some(user) = response {
        if verify_password(&hashed_password,&login.password) {
            message = "Successfully logged in".to_string();
            status = "Success".to_string();
            responses = LoginResponse{
                status,
                message,
                user: Some(user.clone()),
            };

            let mut current = user_state.current_user.lock().await;
            *current = Some(user);

            return (StatusCode::OK,Json(responses))
        }
        responses = LoginResponse{
            status,
            message,
            user: None,
        };

        (StatusCode::UNAUTHORIZED,Json(responses))
    }else{
        message = "User Not Found".to_string();
        responses = LoginResponse{
            status,
            message,
            user: None,
        };
        (StatusCode::NOT_FOUND,Json(responses))
    }
}

pub async fn login_by_google(state: AppState, user_state: UserState) -> (StatusCode, Json<String>){
    (StatusCode::BAD_REQUEST,Json("Failed to login".to_string()))
}

pub async fn change_password(new_password:String, state: AppState, user_state: UserState)-> (StatusCode, Json<String>){

    let (passed, validation_result) = validate_password(&new_password.clone());
    if !passed {
        return (StatusCode::FORBIDDEN,Json(validation_result.to_string()));
    }

    let current_user = user_state.current_user.lock().await;

    if let Some(user) = &*current_user {

        let (_, password) = external_service::get_user_by_id(&state.db, user.id.to_string().clone()).await;
        if verify_password(&password,&new_password){
            return (StatusCode::FORBIDDEN, Json("Change the character combination".to_string()))
        }

        let hashed_password = hash_password(new_password.as_str());

        let response = service::change_password(&state.db, user.id.to_string().clone(), hashed_password).await;
        if response {
            return (StatusCode::OK, Json("Successfully change password".to_string()))
        }
        (StatusCode::BAD_REQUEST, Json("Failed to change password".to_string()))
    } else {
        (StatusCode::UNAUTHORIZED, Json("Not logged in".to_string()))
    }
}

pub async fn forgot_password(email: String, new_password:String, state: AppState)-> (StatusCode, Json<String>){
    let (response, _) = external_service::get_user_by_email(&state.db, email.clone()).await;

    if let Some(user) = response {
        let hashed_password = hash_password(new_password.as_str());
        let (responses, token) = service::insert_token(&state.db,user.id, hashed_password).await;

        if !responses{
            return (StatusCode::BAD_REQUEST,Json("Failed to send email".to_string()))
        }

        let subject = "Forgot Password".to_string();
        let link = format!("http://127.0.0.1:3000/auth/reset-password?token={}", token);
        let body_message =
            format!("Hi {}, welcome back!\nPlease click the following link to reset your password\n{}\n\n\nBest Regards,\nByme",user.name ,link);
        send_email(email,subject,body_message);
    }

    (StatusCode::FORBIDDEN,Json("Email is not found".to_string()))
}

pub async fn reset_password(token:ResetQuery, state:AppState) -> (StatusCode, Json<String>){
    let response = service::get_token(&state.db,token.token).await;
    if let Some(tokens) = response {
        let now = Utc::now().naive_utc();
        if !(now - tokens.created_at < Duration::minutes(5)) {
            return (StatusCode::FORBIDDEN,Json("Token expired".to_string()))
        }


        service::change_password(&state.db,tokens.userid.to_string(),tokens.newpassword.to_string()).await;
        service::delete_token(&state.db,tokens.id).await;
        return (StatusCode::OK,Json("Successfully reset password".to_string()))
    }
    (StatusCode::BAD_REQUEST,Json("Failed to get token".to_string()))
}


