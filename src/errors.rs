use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Error centralizado de la API. Implementa IntoResponse para Axum.
#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    Unauthorized(String),
    Forbidden(String),
    InsufficientStock(String),
}

/// Estructura de respuesta de error en formato JSON
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error, message) = match self {
            AppError::DatabaseError(e) => {
                tracing::error!("Error de base de datos: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR".to_string(),
                    "Ocurrió un error en la base de datos".to_string(),
                )
            }
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND".to_string(),
                msg,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST".to_string(),
                msg,
            ),
            AppError::InternalError(msg) => {
                tracing::error!("Error interno: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_SERVER_ERROR".to_string(),
                    msg,
                )
            }
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED".to_string(),
                msg,
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN".to_string(),
                msg,
            ),
            AppError::InsufficientStock(msg) => (
                StatusCode::CONFLICT,
                "INSUFFICIENT_STOCK".to_string(),
                msg,
            ),
        };

        (status, Json(ErrorResponse { error, message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Registro no encontrado".to_string()),
            other => AppError::DatabaseError(other),
        }
    }
}

