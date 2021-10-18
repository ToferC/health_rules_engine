use std::str::FromStr;

use actix_web::{HttpRequest, Result};
use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use argonautica::{Error, Hasher, Verifier};

lazy_static! {
    static ref JWT_SECRET_KEY: String = 
        std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");
}

lazy_static! {
    static ref PASSWORD_SECRET_KEY: String = 
        std::env::var("PASSWORD_SECRET_KEY").expect("Can't read PASSWORD_SECRET_KEY");
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

#[derive(Eq, PartialEq, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    Admin,
    User,
}

pub fn create_token(username: String, role: Role) -> String {
    let exp_time = Local::now() + Duration::minutes(120);

    let claims = Claims {
        sub: username,
        exp: exp_time.timestamp(),
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
    )
    .expect("Can't create token")
}

pub fn get_role(http_request: HttpRequest) -> Option<Role> {
    http_request
        .headers()
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let jwt_start_index = "Bearer ".len();
                let jwt = s[jwt_start_index..s.len()].to_string();
                let token_data = decode_token(&jwt);
                Role::from_str(&token_data.claims.role).expect("Can't parse role")
            })
        })
}

fn decode_token(token: &str) -> TokenData<Claims> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )
    .expect("Can't decode token")
}

pub fn hash_password(password: &str) -> Result<String, Error> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .hash()
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(PASSWORD_SECRET_KEY.as_str())
        .verify()
}




