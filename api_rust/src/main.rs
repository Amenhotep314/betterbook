mod models;
mod routes_auth;

use axum::{routing::get, Router};
use std::env;


#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.");
    let app_host = env::var("APP_HOST").expect("APP_HOST must be set in .env.");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set in .env.");

    // Launch app
    let addr = format!("{}:{}", &app_host, &app_port);
    let app = Router::new().merge(api_routes());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // Launch database
    let db = sea_orm::Database::connect(&database_url).await.unwrap();
    db.get_schema_registry("api_rust::entity::*").sync(db).await.unwrap();
}

fn api_routes() -> Router {
    Router::new()
        .merge(routes_auth::routes())
}
