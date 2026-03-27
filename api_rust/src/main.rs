mod models;
mod routes_auth;

use axum::{routing::get, Router};
use migration::{Migrator, MigratorTrait};


let connection = sea_orm::Database::connect(&database_url).await?;
Migrator::up(&connection, None).await?;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(api_routes());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn api_routes() -> Router {
    Router::new()
        .merge(routes_auth::routes())
}
