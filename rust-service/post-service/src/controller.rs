use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::Json;
use model::accessible;
use model::posts::{PostRequest, PostResponse};
use model::state::AppState;
use crate::service;

pub async fn create_post(state:AppState, multipart: &mut Multipart, token:&str)  -> (StatusCode, Json<String>){
    let claims = match accessible::verify_jwt(token, &state.secret) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json("Please login first before create a post".to_string())),
    };

    let mut post = PostRequest {
        description: String::new(),
        files: Vec::new(),
    };

    while let Some(field) = match multipart.next_field().await {
        Ok(Some(f)) => Some(f),
        Ok(None) => None,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json("Invalid form data".to_string()),
            );
        }
    } {
        match field.name().unwrap_or("") {
            "description" => {
                if let Ok(desc) = field.text().await {
                    post.description = desc;
                }
            }
            "files" => {
                let filename = field.file_name().unwrap_or("upload.bin").to_string();
                let data = match field.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(_) => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json("Failed to read file".to_string()),
                        );
                    }
                };
                post.files.push((filename, data));
            }
            _ => {}
        }
    }

    let response = service::create_post(&state.db, post, claims.id).await;
    if response {
        return (StatusCode::CREATED, Json("Post uploaded".to_string()))
    }
    (StatusCode::BAD_REQUEST, Json("Failed to create post".to_string()))
}

pub async fn get_post_by_id(state: AppState, post_id: String) -> (StatusCode, Json<Option<PostResponse>>) {
    let response = service::get_post_by_id(&state.db, post_id).await;
    if let Some((post,_)) = response {
        return (StatusCode::OK, Json(Some(post)))
    }
    (StatusCode::OK, Json(None))
}

pub async fn get_all_posts_by_user_id(state: AppState, userid: String) -> (StatusCode, Json<Option<Vec<PostResponse>>>) {
    let response = service::get_all_post_by_user_id(&state.db, userid).await;
    (StatusCode::OK, Json(response))
}

pub async fn delete_post(state:AppState, post_id:String, token:&str) -> (StatusCode, Json<String>) {
    let claims = match accessible::verify_jwt(token, &state.secret) {
        Ok(c) => c,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json("Please login first before create a post".to_string())),
    };

    let post = service::get_post_by_id(&state.db, post_id.clone()).await;
    if let Some((_, user_id)) = post {
        if user_id != claims.id {
            return (StatusCode::UNAUTHORIZED, Json("Unable to delete post".to_string()));
        }
    }

    let response = service::delete_post(&state.db, post_id).await;

    if response {
        return (StatusCode::OK, Json("Post deleted".to_string()))
    }
    (StatusCode::BAD_REQUEST, Json("Failed to delete post".to_string()))
}