use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;

use crate::entity::user;
use crate::state::AppState;
use crate::{auth, db_util};

#[derive(Deserialize)]
pub struct SignInData {
    pub email: String,
    pub password: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/login", get(login))
}

async fn index() -> &'static str {
    "Hello, world!"
}

pub async fn login(
    State(state): State<AppState>,
    Json(user_data): Json<SignInData>,
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
