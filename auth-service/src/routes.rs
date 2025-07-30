use axum::{Json, extract::State};
use axum::response::{IntoResponse, Response};
use crate::{controller, AppState};
use model::users::{NewUser,LoginRequest};
use crate::response::LoginResponse;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Json<String> {
    controller::register_user(payload,state).await
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
pub fn auth_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
}
