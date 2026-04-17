use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use sea_orm::Set;
use serde::Deserialize;

use crate::entity::user;
use crate::state::AppState;
use crate::{auth, db_util};

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignUpData {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login))
        .route("/signup", get(signup))
}

async fn index() -> &'static str {
    "Hello, world!"
}

async fn login(
    State(state): State<AppState>,
    Json(user_data): Json<LoginData>,
) -> Result<Json<String>, StatusCode> {
    let curr_user: user::Model =
        match db_util::retrieve_user_by_email(&state.db, &user_data.email).await {
            Some(curr_user) => curr_user,
            None => return Err(StatusCode::UNAUTHORIZED),
        };

    if !auth::verify_password(&user_data.password, &curr_user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token =
        auth::encode_jwt(curr_user.email_addr).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(token))
}

async fn signup(
    State(state): State<AppState>,
    Json(user_data): Json<SignUpData>,
) -> impl IntoResponse {
    if db_util::user_already_exists(&state.db, &user_data.username, &user_data.email)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::CONFLICT);
    }

    let password_hash =
        auth::hash_password(&user_data.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user: user::ActiveModel = user::ActiveModel {
        email_addr: Set(user_data.email.to_owned()),
        username: Set(user_data.username.to_owned()),
        password_hash: Set(password_hash.to_owned()),
        ..Default::default()
    };

    db_util::add_new_user(&state.db, new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
