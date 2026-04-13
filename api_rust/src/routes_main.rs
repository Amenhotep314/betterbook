use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::auth;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/feed", get(services::hello).layer(middleware::from_fn(auth::authorize))),
}


async fn feed() {}
