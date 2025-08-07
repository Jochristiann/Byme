use axum::extract::State;
use axum::Json;
use axum::response::{IntoResponse, Response};
use model::state::AppState;
use crate::controller;

pub async fn by_id(
    State(state): State<AppState>,
    Json(payload): Json<String>,
) -> Response {

    let (status, data) = controller::by_id(state, payload).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}
 
pub async fn by_email(
    State(state): State<AppState>,
    Json(payload): Json<String>,
) -> Response {

    let (status, data) = controller::by_email(state, payload).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn get_all_user_by_role(
    State(state): State<AppState>,
    Json(payload): Json<String>,
) -> Response {
    let (status, data) = controller::get_all_user_by_role(state, payload).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub fn user_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/get-user-by-id", axum::routing::get(by_id))
        .route("/get-user-by-email", axum::routing::get(by_email))
        .route("/all-by-role", axum::routing::get(get_all_user_by_role))
}