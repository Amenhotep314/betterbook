use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use crate::{auth, services};

pub fn routes() -> Router {
    Router::new()
        .route("/feed", get(services::hello).layer(middleware::from_fn(auth::authorize))),
}


async fn feed() {}
