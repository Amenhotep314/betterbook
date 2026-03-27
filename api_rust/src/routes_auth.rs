use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/get_token", get(authenticate))
}

async fn index() -> &'static str {
    "Hello, world!"
} 

async fn authenticate() {}
