use axum::extract::{State,Multipart};
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use model::state::AppState;
use crate::controller;

pub async fn create_post(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut multipart: Multipart,
)-> Response {
    let token = bearer.token();
        let (status, data) = controller::create_post(state, &mut multipart, token).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn delete_post(
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<String>,
)-> Response {
    let token = bearer.token();
    let (status, data) = controller::delete_post(state, payload, token).await;

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
        .route("/create", axum::routing::post(create_post))
        .route("/delete", axum::routing::delete(delete_post))
}