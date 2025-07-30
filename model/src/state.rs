use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::users::Users;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub current_user: Arc<Mutex<Option<Users>>>,
}


pub fn establish_connection_pool() -> DbPool {
    let user = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set");
    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
    let dbname = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, dbname
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool")
}