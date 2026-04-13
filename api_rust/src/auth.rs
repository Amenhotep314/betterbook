// Useful resources about authentication:
// https://blog.logrocket.com/using-rust-axum-build-jwt-authentication-api/

use axum::{
    body::Body,
    extract::{Json, Request, State},
    http,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

use crate::db_util;
use crate::state::AppState;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp_time: usize,
    pub issue_time: usize,
    pub email: String,
}

pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let secret: String = env::var("SECRET_KEY").expect("Secret key must be set.");
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp_time: usize = (now + expire).timestamp() as usize;
    let issue_time: usize = now.timestamp() as usize;
    let claim = Claims {
        exp_time,
        issue_time,
        email,
    };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = env::var("SECRET_KEY").expect("Secret key must be set.");
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn authorization_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(AuthError {
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            });
        }
    };
    let mut header = auth_header.split_whitespace();
    let _ = header.next();
    let token: Option<&str> = header.next();
    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    let current_user =
        match db_util::retrieve_user_by_email(&state.db, &token_data.claims.email).await {
            Some(user) => user,
            None => {
                return Err(AuthError {
                    message: "You are not an authorized user".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                });
            }
        };
    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
