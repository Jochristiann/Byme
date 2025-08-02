use axum::{Json, extract::State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use crate::{controller, AppState};
use model::users::{NewUser,LoginRequest};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Response {
    let (status, data) = controller::register_user(payload, state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}
#[axum::debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Response {
    let (status, data) = controller::login_user(payload, state).await;

    let mut response = data.into_response();
    *response.status_mut() = status;
    response
}

pub async fn get_current_user(State(state): State<AppState>) -> Response {
    let current = state.current_user.lock().await;
    if let Some(user) = &*current {
        Json(user).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, "Not logged in").into_response()
    }
}


pub async fn change_password (State(state): State<AppState>) -> Response {
    
    (StatusCode::UNAUTHORIZED, "Not logged in").into_response()
}

pub fn auth_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/get-user", axum::routing::get(get_current_user))
}
