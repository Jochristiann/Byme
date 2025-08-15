use axum::{ Router};
use std::net::SocketAddr;
use std::path::Path;
use dotenvy::from_path;
use tower_http::cors::{Any, CorsLayer};
use model::state::{establish_connection_pool};

mod controller;
mod routes;
mod repository;
mod responses;
mod service;

#[tokio::main]
async fn main() {
    let dotenv_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../model/.env");
    from_path(dotenv_path).expect("Failed to load .env");
    let state = establish_connection_pool();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .nest("/auth", routes::auth_routes())
        .with_state(state)
        .layer(cors);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


