use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;
    let claims = Claims { exp, iat };
    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&Header::default(), &claims, &key).map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let secret = dotenv!("JWT_SECRET");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            eprintln!("Error validanting token {:?}", error);
            AppError::new(StatusCode::UNAUTHORIZED, "Your session has expired, please login again")
        },
        _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong, please try again")
    })?;
    Ok(true)
}
