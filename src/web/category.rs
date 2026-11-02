use core::error;

use axum::{Router, extract::{Query, State}, response::IntoResponse, routing::get};

use crate::{domain::category::{dto::{CategoryFlashParams, CategoryResponseDTO, CategoryTemplate, CategoryTree}, error::CategoryError}, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_categories_page))
        // .route("/{id}/branch", get(render_branch_page))
        // .route("/new", get(render_new_category_page))
        // .route("/", post(create_category))
        // .route("/{id}/edit", get(render_edit_category_form))
        // .route("/{id}/edit", post(edit_category))
        // .route("/{id}/delete", post(delete_category))
        // .route("/search", get(search_categories))
}


pub async fn fetch_all_categories(state: &AppState) -> Result<Vec<CategoryResponseDTO>, CategoryError> {
    let categories = sqlx::query_as!(
        CategoryResponseDTO,
        r#"
        SELECT c.id, c.parent_id, c.name_ar, c.name_en, c.created_at, c.updated_at 
        FROM categories c 
        ORDER BY c.id DESC;
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(categories)
}

pub async fn render_categories_page(
    State(state): State<AppState>,
    Query(params): Query<CategoryFlashParams>,
) -> Result<CategoryTemplate, CategoryError> {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("Category Created Successfully".to_string()),
        Some("updated") => Some("Category Updated Successfully".to_string()),
        Some("deleted") => Some("Category Deleted Successfully".to_string()),
        _ => None,
    };

    let error_message = match params.error.as_deref() {
        Some("not_found") => Some("غير موجود بقاعدة البيانات".to_string()),
        Some("delete_failed") | Some("has_children") => Some("لا يمكن محو هذا السجل".to_string()),
        Some("duplicate") => Some("التصنيف مسجل بالفعل".to_string()),
        Some("depth_exceeded") => Some("تم تجاوز الحد الأقصى للشجرة (3 مستويات)".to_string()),
        Some("self_parent") => Some("لا يمكن أن يكون التصنيف أبًا لنفسه".to_string()),
        Some("db_error") => Some("خطأ عام بقاعدة البيانات".to_string()),
        _ => None,
    };

    // Let the error propagate so CategoryError::into_response() handles DB failures gracefully
    let categories = fetch_all_categories(&state).await?;

    Ok(CategoryTemplate {
        category_tree: CategoryTree::build_tree(categories),
        error_message,
        success_message,
        current_page: "categories".to_string(),
    })
}