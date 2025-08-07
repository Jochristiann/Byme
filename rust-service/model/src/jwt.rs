use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

const EXPIRATION: u64 = 1800;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

impl Claims {
    pub fn with_exp(sub: String, role: String) -> Self {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + EXPIRATION;
        Self { sub, role, exp: exp as usize }
    }
}
