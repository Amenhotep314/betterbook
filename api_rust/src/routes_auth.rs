use axum::{
    Router,
    extract::{Json, Request},
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{auth, db_util};


#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}


pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/sign_in", get(sign_in))
}

async fn index() -> &'static str {
    "Hello, world!"
}

pub async fn sign_in(Json(user_data): Json<SignInData>) -> Result<Json<String>, StatusCode> {
    let user = match db_util::retrieve_user_by_email(&user_data.email) {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !auth::verify_password(&user_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth::encode_jwt(user.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(token))
}
