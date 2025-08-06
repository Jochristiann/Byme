use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{ Router};
use std::net::SocketAddr;
use std::path::Path;
use dotenvy::from_path;
use model::state::{establish_connection_pool, AppState, AuthState, UserState};

mod controller;
mod routes;
mod repository;
mod response;
mod service;

#[tokio::main]
async fn main() {
    let dotenv_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../model/.env");
    from_path(dotenv_path).expect("Failed to load .env");
    let db = establish_connection_pool();
    let app_state = AppState {
        db
    };

    let user_state = UserState{
        current_user: Arc::new(Mutex::new(None))
    };

    let auth_state = AuthState{
        app_state,
        user_state
    };

    let app = Router::new()
        .nest("/auth", routes::auth_routes())
        .with_state(auth_state);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


