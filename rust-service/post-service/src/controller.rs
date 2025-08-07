use axum::http::StatusCode;
use axum::Json;
use model::posts::{PostRequest, PostResponse};
use model::state::AppState;
use crate::service;

pub async fn create_post(state:AppState, post:PostRequest)  -> (StatusCode, Json<String>){
    let response = service::create_post(&state.db, post).await;
    if response {
        return (StatusCode::CREATED, Json("Post uploaded".to_string()))
    }
    (StatusCode::BAD_REQUEST, Json("Failed to create post".to_string()))
}

pub async fn get_post_by_id(state: AppState, post_id: String) -> (StatusCode, Json<Option<PostResponse>>) {
    let response = service::get_post_by_id(&state.db, post_id).await;
    (StatusCode::OK, Json(response))
}

pub async fn get_all_posts_by_user_id(state: AppState, userid: String) -> (StatusCode, Json<Option<Vec<PostResponse>>>) {
    let response = service::get_all_post_by_user_id(&state.db, userid).await;
    (StatusCode::OK, Json(response))
}

pub async fn get_all_posts_by_category(state: AppState, category: String) -> (StatusCode, Json<Option<Vec<PostResponse>>>) {
    let response = service::get_all_post_by_category(&state.db, category).await;
    (StatusCode::OK, Json(response))
}

pub async fn delete_post(state:AppState, post_id:String) -> (StatusCode, Json<String>) { 
    let response = service::delete_post(&state.db, post_id).await;

    if response {
        return (StatusCode::OK, Json("Post deleted".to_string()))
    }
    (StatusCode::BAD_REQUEST, Json("Failed to delete post".to_string()))
}