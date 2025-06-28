// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod model;
#[path = "auth-services/mod.rs"]
mod auth_services;
mod state;

use crate::state::AppState;
use sqlx::PgPool;
use tauri::Manager;

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {

    let db = PgPool::connect("postgres://user:pass@localhost/dbname")
        .await
        .expect("Could not connect to DB");

    // let app_state = AppState {
    //     db,
    //     current_user: Arc::new(Mutex::new(None)),
    // };

    // // if using Axum
    // let app = Router::new()
    //     .route("/", get(root_handler))
    //     .with_state(app_state);
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
