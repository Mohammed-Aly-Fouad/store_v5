use axum::response::{IntoResponse, Redirect, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrandError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Brand not found")]
    NotFound,

    #[error("Duplicate brand name")]
    Duplicate { name: String }, // Used for manual business logic checks

    #[error("Foreign key constraint violation")]
    HasChildren, // Used for manual pre-deletion checks
}

impl IntoResponse for BrandError {
    fn into_response(self) -> Response {
        match self {
            BrandError::NotFound => {
                Redirect::to("/web/brands?error=not_found").into_response()
            }
            BrandError::HasChildren => {
                Redirect::to("/web/brands?error=has_children").into_response()
            }
            BrandError::Duplicate { .. } => {
                Redirect::to("/web/brands?error=duplicate").into_response()
            }
            BrandError::Sqlx(ref err) => {
                // Check for specific Postgres error codes coming straight from the driver
                if let sqlx::Error::Database(db_err) = err {
                    if db_err.code().as_deref() == Some("23503") {
                        return Redirect::to("/web/brands?error=has_children").into_response();
                    }
                    if db_err.code().as_deref() == Some("23505") {
                        return Redirect::to("/web/brands?error=duplicate").into_response();
                    }
                }
                
                tracing::error!("Database error in Brand module: {:?}", err);
                Redirect::to("/web/brands?error=db_error").into_response()
            }
        }
    }
}