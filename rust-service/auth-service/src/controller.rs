use axum::http::StatusCode;
use lettre::{Message, SmtpTransport, Transport};
use argon2::{
    Argon2, PasswordHash,
};
use axum::Json;
use lettre::transport::smtp::authentication::Credentials;
use password_hash::{ PasswordHasher, PasswordVerifier, SaltString};
use rand_core::OsRng;
use model::users::{LoginRequest, RegisterUsers};
use crate::{service};
use model::state::{AppState, UserState};
use regex::Regex;
use user_service::service as external_service;
use crate::response::{LoginResponse};

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

pub async fn forgot_password(email: String, new_password:String, state: AppState)-> (StatusCode, axum::Json<String>){
    let (response, password) = external_service::get_user_by_email(&state.db, email).await;
    let email = Message::builder()
        .from("Your Name <your_email@gmail.com>".parse().unwrap())
        .to("Recipient <recipient@example.com>".parse().unwrap())
        .subject("Hello from Rust!")
        .body(String::from("This is a test email sent using Rust and Lettre!"))
        .unwrap();

    let creds = Credentials::new("your_email@gmail.com".to_string(), "your_app_password".to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("✅ Email sent successfully!"),
        Err(e) => eprintln!("❌ Failed to send email: {e}"),
    }

    (StatusCode::BAD_REQUEST,Json("Failed to change password".to_string()))
}


