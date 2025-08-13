use axum::http::StatusCode;
use axum::Json;
use model::accessible;
use model::comments::CommentRequest;
use model::state::{AppState, DbPool};
use crate::service;

pub async fn create_comment(pool: &DbPool, state:AppState, comment:CommentRequest, token: &str) -> (StatusCode, Json<String>) {
    let claims = match accessible::verify_jwt(token, &state.secret) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json("Failed to create comment".to_string())),
    };

    let response = service::create_comment(pool, comment, claims.id).await;

    if response {
        return (StatusCode::OK, Json("Successfully create a comment".to_string()))
    }

    (StatusCode::BAD_REQUEST, Json("Failed to create comment".to_string()))
}

pub async fn delete_comment(pool: &DbPool, comment_id:String) -> (StatusCode, Json<String>) {
    let actual_user_id = service::get_user_id_by_comment(pool,comment_id.clone()).await;
    if let Some(user_id) = actual_user_id {
        if user_id != comment_id.clone() {
            return (StatusCode::UNAUTHORIZED, Json("Forbid to delete comment".to_string()))
        }
        let response = service::delete_comment(pool, comment_id).await;
        if response {
            return (StatusCode::OK, Json("Successfully delete a comment".to_string()))
        }

        return (StatusCode::BAD_REQUEST, Json("Failed to delete comment".to_string()))
    }

    (StatusCode::BAD_REQUEST, Json("Failed to delete comment".to_string()))
}