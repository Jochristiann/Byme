use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::PgPool;
use crate::model::users::Users;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub current_user: Arc<Mutex<Option<Users>>>,
}
