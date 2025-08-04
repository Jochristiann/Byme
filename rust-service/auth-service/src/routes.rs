use axum::{Json, extract::State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use model::state::{AuthState};
use crate::{controller};
use model::users::{LoginRequest, RegisterUsers};

pub async fn register(
    State(state): State<AuthState>,
    Json(payload): Json<RegisterUsers>,
) -> Response {
    let (status, data) = controller::register_user(payload, state.app_state,state.user_state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}
#[axum::debug_handler]
pub async fn login(
    State(state): State<AuthState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let (status, data) = controller::login_user(payload, state.app_state, state.user_state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn get_current_user(State(state): State<AuthState>) -> Response {
    let current = state.user_state.current_user.lock().await;
    if let Some(user) = &*current {
        Json(user).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Not logged in").into_response()
    }
}

pub async fn logout(State(state): State<AuthState>) -> Response {
    let mut current = state.user_state.current_user.lock().await;
    if current.is_some() {
        *current = None;
        (StatusCode::OK, "Successfully logged out").into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Not logged in").into_response()
    }
}


pub async fn change_password (State(state): State<AuthState>, Json(payload): Json<String>) -> Response {
    let (status, data) = controller::change_password(payload, state.app_state, state.user_state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
    // (StatusCode::UNAUTHORIZED, "Not logged in").into_response()
}

pub fn auth_routes() -> axum::Router<AuthState> {
    axum::Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/logout", axum::routing::post(logout))
        .route("/get-user", axum::routing::get(get_current_user))
        .route("/change-password", axum::routing::post(change_password))
}
