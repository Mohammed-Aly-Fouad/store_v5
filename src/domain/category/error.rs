use axum::response::{IntoResponse, Redirect, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Category not found")]
    NotFound,

    #[error("Duplicate category name")]
    Duplicate,

    #[error("Category has sub-categories or is linked to products")]
    HasChildren, // تظهر عند محاولة حذف فئة تحتها فئات أبناء أو منتجات (بسبب ON DELETE RESTRICT)

    #[error("Category depth limit exceeded (Maximum 3 levels allowed)")]
    DepthExceeded, // تظهر عند محاولة تجاوز الحد الأقصى للشجرة (3 مستويات)

    #[error("Category cannot be its own parent")]
    SelfParent, // تظهر إذا حاول المستخدم جعل الفئة أبًا لنفسها
}

impl IntoResponse for CategoryError {
    fn into_response(self) -> Response {
        match self {
            CategoryError::NotFound => {
                Redirect::to("/web/categories?error=not_found").into_response()
            }
            CategoryError::HasChildren => {
                Redirect::to("/web/categories?error=has_children").into_response()
            }
            CategoryError::DepthExceeded => {
                Redirect::to("/web/categories?error=depth_exceeded").into_response()
            }
            CategoryError::SelfParent => {
                Redirect::to("/web/categories?error=self_parent").into_response()
            }
            CategoryError::Duplicate => {
                Redirect::to("/web/categories?error=duplicate").into_response()
            }
            CategoryError::Sqlx(ref err) => {
                // التقاط أكواد أخطاء قاعدة البيانات الشهيرة من بوستجريس
                if let sqlx::Error::Database(db_err) = err {
                    // خطأ انتهاك المفتاح الأجنبي (Foreign Key Violation) عند الحذف
                    if db_err.code().as_deref() == Some("23503") {
                        return Redirect::to("/web/categories?error=has_children").into_response();
                    }
                    // خطأ التكرار (Unique Violation)
                    if db_err.code().as_deref() == Some("23505") {
                        return Redirect::to("/web/categories?error=duplicate").into_response();
                    }

                    // التقاط رسائل الـ Triggers المخصصة التي برمجناها في الـ Migration
                    let msg = db_err.message();
                    if msg.contains("Category depth limit exceeded") {
                        return Redirect::to("/web/categories?error=depth_exceeded").into_response();
                    }
                    if msg.contains("Category cannot be its own parent") {
                        return Redirect::to("/web/categories?error=self_parent").into_response();
                    }
                }
                
                tracing::error!("Database error in Category module: {:?}", err);
                Redirect::to("/web/categories?error=db_error").into_response()
            }
        }
    }
}