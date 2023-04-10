use crate::models::{LogonSession, User};
use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, TokenData, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Debug, Serialize, Deserialize)]
struct TokenContent {
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub id: i32,
    pub session: Uuid,
    pub role: String,
}

pub enum Security {
    AppToken(TokenContent),
    ApiKey(String),
}

#[derive(Debug)]
pub enum SecurityError {
    Missing,
    Invalid,
}

fn get_secret() -> String {
    dotenv().ok();

    env::var("APP_SECRET").expect("APP_SECRET environment variable must be set")
}

fn get_token_data(token: String) -> Result<TokenData<TokenContent>, SecurityError> {
    let key = &DecodingKey::from_secret(get_secret().as_bytes());

    match decode::<TokenContent>(&token, key, &Validation::default()) {
        Ok(data) => Ok(data),
        Err(_) => Err(SecurityError::Invalid),
    }
}

fn verify_token_data(token: &TokenData<TokenContent>) -> bool {
    match LogonSession::load_by_id(token.claims.session) {
        Some(session) => {
            if session.verify() {
                session.refresh();

                true
            } else {
                false
            }
        }
        None => false,
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Security {
    type Error = SecurityError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match Security::read_any(request) {
            Ok(security) => Outcome::Success(security),
            Err(error) => Outcome::Failure((Status::Unauthorized, error)),
        }
    }
}

impl Security {
    pub fn make_token(user: User, session: LogonSession) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000;
        let key = &EncodingKey::from_secret(get_secret().as_bytes());
        let payload = TokenContent {
            iat: now,
            nbf: now,
            exp: now + ONE_WEEK,
            id: user.id,
            role: user.role,
            session: session.id,
        };

        encode(&jsonwebtoken::Header::default(), &payload, key)
            .expect("Error while creating user token")
    }

    fn read_any(request: &Request<'_>) -> Result<Security, SecurityError> {
        if let Some(security) = Security::read_app_token(request) {
            return Ok(security);
        }

        if let Some(security) = Security::read_api_key(request) {
            return Ok(security);
        }

        Err(SecurityError::Missing)
    }

    fn read_app_token(request: &Request<'_>) -> Option<Security> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let auth_string = auth_header.to_string();

            if auth_string.starts_with("Bearer ") {
                let token_string = auth_string[6..auth_string.len()].trim();

                if let Ok(token_data) = get_token_data(token_string.to_string()) {
                    if verify_token_data(&token_data) {
                        return Some(Security::AppToken(token_data.claims));
                    }
                }
            }
        }

        None
    }

    fn read_api_key(request: &Request<'_>) -> Option<Security> {
        if let Some(key_header) = request.headers().get_one("X-Api-Key") {
            todo!();
        }

        None
    }
}
