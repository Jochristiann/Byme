use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub secret: String,
}

pub fn establish_connection_pool() -> AppState {
    let user = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set");
    let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let host = env::var("DATABASE_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string());
    let dbname = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let jwt_secret = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, dbname
    );

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pools = Pool::builder()
        .build(manager)
        .expect("Failed to create DB pool");

    AppState{
        db: pools,
        secret: jwt_secret,
    }
}