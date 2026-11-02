use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

/// Global application-wide errors for cross-cutting concerns (500s, infrastructure, templates).
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),

    #[error("Template rendering error: {0}")]
    Template(#[from] askama::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Sqlx(ref err) => {
                tracing::error!("Global Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "خطأ عام بقاعدة البيانات".to_string(),
                )
            }
            AppError::Migrate(ref err) => {
                tracing::error!("Migration error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "خطأ في تحديث هيكل قاعدة البيانات (Migrations)".to_string(),
                )
            }
            AppError::Template(ref err) => {
                tracing::error!("Template rendering error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "خطأ في معالجة واجهة العرض".to_string(),
                )
            }
            AppError::Io(ref err) => {
                tracing::error!("I/O system error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "خطأ في نظام الملفات الداخلي".to_string(),
                )
            }
            AppError::Internal(ref msg) => {
                tracing::error!("Internal application error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "حدث خطأ غير متوقع في النظام".to_string(),
                )
            }
        };

        (status, error_message).into_response()
    }
}