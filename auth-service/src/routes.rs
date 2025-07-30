use axum::{Json, extract::State};
use crate::{controller, AppState};
use model::users::{NewUser,LoginRequest};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Json<String> {
    controller::register_user(payload,state).await
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<String> {
    controller::login_user(payload, state).await
}

pub fn auth_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
}
