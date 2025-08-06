use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use crate::jwt::Claims;
use jsonwebtoken::errors::Error;
use uuid::Uuid;

pub fn generate_jwt(claims: Claims, secret: &str) -> Result<String, Error> {
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}

pub fn parse_id (id:String) -> Uuid {
    Uuid::parse_str(&id).unwrap_or_else(|er| {
        eprintln!("{}", er);
        Uuid::nil()
    })
}