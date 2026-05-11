use sea_orm::{DatabaseConnection, DbErr};
use bcrypt::BcryptError;
use axum::{
    response::{Response, IntoResponse},
    http::StatusCode,
};
use serde_json::Json;
use tracing;


#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[derive(Debug)]
pub enum AppError {
    DbErr,
    BcryptError,
    AuthError { message: String, status_code: StatusCode },
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {

        tracing::error!(?self);

        match self {
            AppError::AuthError => {
                let body = Json(json!({
                    "error": self.message,
                }));
                (self.status_code, body).into_response()
            }
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
            }
        }
    }
}
