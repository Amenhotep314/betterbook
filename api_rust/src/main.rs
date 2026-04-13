mod auth;
mod db_util;
mod entity;
mod routes_auth;

use axum::{Router, routing::get};
use sea_orm::{ConnectionTrait, Database, DbBackend, Schema, DatabaseConnection};
use std::env;

use crate::entity::user;


#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    // Load environment variables
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.");
    let app_host = env::var("APP_HOST").expect("APP_HOST must be set in .env.");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set in .env.");

    // Launch database
    let db = Database::connect(&database_url).await.expect("Failed to connect to db.");
    db.get_schema_registry("crate::entity::*").sync(&db).await?;

    //Create app state object
    let state = AppState { db };

    // Launch app
    let addr = format!("{}:{}", &app_host, &app_port);
    let app = Router::new().merge(api_routes());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// Get mapping from URLs to functions
fn api_routes() -> Router {
    Router::new().merge(routes_auth::routes())
}
