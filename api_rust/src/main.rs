mod auth;
mod db_util;
mod entity;
mod routes_auth;
mod state;

use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use std::time::Duration;

use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    // Load environment variables
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.");
    let app_host = env::var("APP_HOST").expect("APP_HOST must be set in .env.");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set in .env.");

    // Launch database
    let mut opt = ConnectOptions::new(&database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("Failed to connect to db.");
    db.get_schema_registry("crate::entity::*").sync(&db).await?;

    //Create app state object
    let state = AppState { db };

    // Launch app
    let addr = format!("{}:{}", &app_host, &app_port);
    let app = Router::new().merge(api_routes()).with_state(state);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// Get mapping from URLs to functions
fn api_routes() -> Router<AppState> {
    Router::new().merge(routes_auth::routes())
}
