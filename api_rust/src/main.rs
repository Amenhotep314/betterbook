mod routes_auth;
mod entity;

use axum::{routing::get, Router};
use std::env;
use sea_orm::{Database, Schema, DbBackend, ConnectionTrait};

use crate::entity::user;

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    // Load environment variables
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.");
    let app_host = env::var("APP_HOST").expect("APP_HOST must be set in .env.");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set in .env.");

    // Launch database
    let db = Database::connect(&database_url).await?;
    db.get_schema_registry("crate::entity::*").sync(&db).await?;

    // Launch app
    let addr = format!("{}:{}", &app_host, &app_port);
    let app = Router::new().merge(api_routes());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn api_routes() -> Router {
    Router::new()
        .merge(routes_auth::routes())
}
