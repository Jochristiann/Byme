use std::env;
use axum::http::StatusCode;
use lettre::{Message, SmtpTransport, Transport};
use argon2::{
    Argon2, PasswordHash,
};
use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::transport::smtp::authentication::Credentials;
use password_hash::{ PasswordHasher, PasswordVerifier, SaltString};
use rand_core::OsRng;
use model::users::{LoginRequest, RegisterUsers, UserResponse};
use crate::{service};
use model::state::{AppState};
use regex::Regex;
use model::accessible;
use model::accessible::parse_id;
use model::jwt::Claims;
use user_service::service as external_service;
use crate::responses::{LoginResponse, ResetQuery};

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

pub async fn get_current_user(state: AppState, token:&str) -> (StatusCode, Json<Option<UserResponse>>) {
    let claims = match accessible::verify_jwt(token, &state.secret) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(None)),
    };

    let (curr_user, _, _) = external_service::get_user_by_id(&state.db, claims.id).await;
    if let Some(user) = curr_user {
        (StatusCode::OK,Json(Some(user)))
    } else {
        (StatusCode::UNAUTHORIZED, Json(None))
    }
}

pub async fn register_user(user: RegisterUsers, state: AppState) -> (StatusCode, Json<LoginResponse>) {
    let message;
    let mut status= "Failed".to_string();
    let responses;

    let (passed, validation_result) = validate_password(&user.password.clone());
    if !passed {
        responses = LoginResponse{
            status : status.clone(),
            message: validation_result,
            user: None,
            token: None
        };

        return (StatusCode::FORBIDDEN,Json(responses))
    }

    let hashed = hash_password(user.password.as_str());
    let user = RegisterUsers {
        name: user.name.clone(),
        email: user.email.clone(),
        password: hashed,

    };

    let (response,_, _) = external_service::get_user_by_email(&state.db, user.email.clone()).await;

    if let Some(_) = response {
        message = "The email is already registered".to_string();
        responses = LoginResponse{
            status,
            message,
            user: None,
            token: None
        };

        return (StatusCode::FORBIDDEN,Json(responses))
    }

    let (x, id) = service::insert_new_user(&state.db, user.clone()).await;

    if x {
        let (response, _, _) = external_service::get_user_by_email(&state.db, user.email).await;
        if let Some(user) = response {
            let claims = Claims::with_exp(id, user.role.clone());
            let token = match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.secret.as_ref()),
            ) {
                Ok(t) => t,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json((LoginResponse{
                    status,
                    message: "Internal server error (Token is not created)".to_string(),
                    user: None,
                    token: None
                }))),
            };

            message = "Successfully registered".to_string();
            status = "Success".to_string();

            responses = LoginResponse{
                status,
                message,
                user: Some(user.clone()),
                token: Some(token)
            };

            (StatusCode::OK,Json(responses))
        }else{
            message = "User Not Found".to_string();
            responses = LoginResponse{
                status,
                message,
                user: None,
                token: None
            };
            (StatusCode::NOT_FOUND,Json(responses))
        }
    }else{
        message = "Failed to Register".to_string();
        responses = LoginResponse{
            status,
            message,
            user: None,
            token: None
        };
        (StatusCode::BAD_REQUEST,Json(responses))
    }
}

pub async fn login_user(login: LoginRequest, state: AppState) -> (StatusCode, Json<LoginResponse>) {
    let (response, id, hashed_password) = external_service::get_user_by_email(&state.db, login.email).await;
    let mut message= "Incorrect Password".to_string() ;
    let mut status= "Failed".to_string();
    let responses;

    if let Some(user) = response {
        if verify_password(&hashed_password,&login.password) {

            let claims = Claims::with_exp(id, user.role.clone());
            let token = match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.secret.as_ref()),
            ) {
                Ok(t) => t,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(LoginResponse{
                    status,
                    message: "Internal server error (Token is not created)".to_string(),
                    user: None,
                    token:None
                })),
            };

            message = "Successfully logged in".to_string();
            status = "Success".to_string();
            responses = LoginResponse{
                status,
                message,
                user: Some(user.clone()),
                token: Some(token)
            };

            return (StatusCode::OK,Json(responses));
        }
        responses = LoginResponse{
            status,
            message,
            user: None,
            token:None
        };

        (StatusCode::UNAUTHORIZED,Json(responses))
    }else{
        message = "User Not Found".to_string();
        responses = LoginResponse{
            status,
            message,
            user: None,
            token: None
        };
        (StatusCode::NOT_FOUND,Json(responses))
    }
}

pub async fn login_by_google(state: AppState) -> (StatusCode, Json<String>){
    (StatusCode::BAD_REQUEST,Json("Failed to login".to_string()))
}

pub async fn change_password(new_password:String, token:&str, state: AppState)-> (StatusCode, Json<String>){

    let (passed, validation_result) = validate_password(&new_password.clone());
    if !passed {
        return (StatusCode::FORBIDDEN,Json(validation_result.to_string()));
    }

    let claims = match accessible::verify_jwt(token, &state.secret) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json("".to_string())),
    };

    let (curr_user, id, password) = external_service::get_user_by_id(&state.db, claims.id).await;

    if let Some(user) = curr_user {
        if verify_password(&password,&new_password){
            return (StatusCode::FORBIDDEN, Json("Change the character combination".to_string()))
        }

        let hashed_password = hash_password(new_password.as_str());

        let response = service::change_password(&state.db, id.clone(), hashed_password).await;
        if response {
            return (StatusCode::OK, Json("Successfully change password".to_string()))
        }
        (StatusCode::BAD_REQUEST, Json("Failed to change password".to_string()))
    } else {
        (StatusCode::UNAUTHORIZED, Json("Not logged in".to_string()))
    }
}

pub async fn forgot_password(email: String, new_password:String, state: AppState)-> (StatusCode, Json<String>){
    let (response,id, _) = external_service::get_user_by_email(&state.db, email.clone()).await;

    if let Some(user) = response {
        let hashed_password = hash_password(new_password.as_str());
        let user_id = parse_id(id.to_string());
        let (responses, token) = service::insert_token(&state.db,user_id, hashed_password).await;

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


