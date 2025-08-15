use axum::{Json, extract::{State}};
use axum::routing;
use axum::response::{IntoResponse, Response};
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use model::state::AppState;
use crate::{controller};
use model::users::{LoginRequest, RegisterUsers};
use crate::responses::{ResetQuery};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUsers>,
) -> Response {
    let (status, data) = controller::register_user(payload, state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let (status, data) = controller::login_user(payload, state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn get_current_user(State(state): State<AppState>,TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>) -> Response {
    let token = bearer.token();
    let (status, data) = controller::get_current_user(state,token).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn change_password (
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<String>) -> Response {
    let token = bearer.token();
    let (status, data) = controller::change_password(payload, token, state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn reset_password (State(state): State<AppState>, Json(payload): Json<ResetQuery>) -> Response {
    let (status, data) = controller::reset_password(payload, state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn forgot_password (State(state): State<AppState>, Json(payload): Json<String>) -> Response {
    let (status, data) = controller::send_to_forget_password(payload, state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub fn auth_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", routing::post(register))
        .route("/login", routing::post(login))
        .route("/get-user", routing::get(get_current_user))
        .route("/change-password", routing::post(change_password))
        .route("/send-email", routing::post(forgot_password))
        .route("/reset-password", routing::post(reset_password))
}
