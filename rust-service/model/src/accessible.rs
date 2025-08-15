use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use crate::jwt::Claims;
use jsonwebtoken::errors::Error;
use uuid::Uuid;

pub fn generate_jwt(claims: Claims, secret: &str) -> Result<String, Error> {
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    if data.claims.exp < now {
        return Err(Error::from(jsonwebtoken::errors::ErrorKind::ExpiredSignature));
    }

    Ok(data.claims)
}

pub fn parse_id (id:String) -> Uuid {
    Uuid::parse_str(&id).unwrap_or_else(|er| {
        eprintln!("{}", er);
        Uuid::nil()
    })
}