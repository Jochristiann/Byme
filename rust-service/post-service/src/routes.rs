use axum::extract::State;
use axum::Json;
use axum::response::{IntoResponse, Response};
use model::posts::PostRequest;
use model::state::AppState;
use crate::controller;

pub async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<PostRequest>,
)-> Response {
    let (status, data) = controller::create_post(state, payload).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn get_all_posts_by_user_id(
    State(state): State<AppState>,
    Json(payload): Json<String>,
) -> Response {
    let (status, data) = controller::get_all_posts_by_user_id(state, payload).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn get_all_posts_by_category(
    State(state): State<AppState>,
    Json(payload): Json<String>,
) -> Response {
    let (status, data) = controller::get_all_posts_by_category(state, payload).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn get_post_by_id(
    State(state): State<AppState>,
    Json(payload): Json<String>,
) -> Response {
    let (status, data) = controller::get_post_by_id(state, payload).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub fn post_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/id", axum::routing::get(get_post_by_id))
        .route("/all-by-user-id", axum::routing::get(get_all_posts_by_user_id))
        .route("/all-by-category", axum::routing::get(get_all_posts_by_category))
        .route("/create", axum::routing::post(create_post))
}