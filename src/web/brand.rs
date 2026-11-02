use axum::{
    Form, Router,
    extract::{Path, Query, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
};

use crate::{
    domain::brand::{
        dto::{
            BrandFlashParams, BrandResponseDTO,  BrandSearchResultsTemplate,
            BrandSearchQuery, BrandsTemplate, FormDTO, EditBrandTemplate, CreateBrandTemplate
        },
        error::BrandError,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_brands_page).post(create_brand))
        .route("/new", get(render_new_brand_page))
        .route("/{id}/edit", get(render_edit_page).post(edit_brand)) // Or put post on a separate route if edit form submits to /{id}/edit
        .route("/{id}/delete", post(delete_brand))
        .route("/search", get(search_brands))
}

// ============================================================================
// 1. FETCH ALL BRANDS (Helper)
// ============================================================================

async fn fetch_all_brands(state: &AppState) -> Result<Vec<BrandResponseDTO>, BrandError> {
    let brands = sqlx::query_as!(
        BrandResponseDTO,
        r#"
        SELECT id, name_en, name_ar, created_at, updated_at 
        FROM brands 
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(brands)
}

// ============================================================================
// 2. RENDER BRANDS LISTING PAGE
// ============================================================================

pub async fn render_brands_page(
    State(state): State<AppState>,
    Query(params): Query<BrandFlashParams>,
) -> impl IntoResponse {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إضافة البراند بنجاح".to_string()),
        Some("updated") => Some("تم تعديل البراند بنجاح".to_string()),
        Some("deleted") => Some("تم حذف البراند بنجاح".to_string()),
        _ => None,
    };

    let error_message = match params.error.as_deref() {
        Some("not_found") => Some("غير موجود بقاعدة البيانات".to_string()),
        Some("delete_failed") | Some("has_children") => Some("لا يمكن محو هذا السجل".to_string()),
        Some("duplicate") => Some("الماركة مسجلة بالفعل".to_string()),
        Some("db_error") => Some("خطأ عام بقاعدة البيانات".to_string()),
        _ => None,
    };

    let brands = fetch_all_brands(&state).await.unwrap_or_default();

    BrandsTemplate {
        brands,
        error_message,
        success_message,
        current_page: "brands".to_string(),
    }
}

// ============================================================================
// 3. REAL-TIME SEARCH (HTMX)
// ============================================================================

pub async fn search_brands(
    State(state): State<AppState>,
    Query(query): Query<BrandSearchQuery>,
) -> Result<impl IntoResponse, BrandError> {
    let q = query.q.trim();

    if q.is_empty() {
        return Ok(BrandSearchResultsTemplate {
            brands: vec![],
            query: String::new(),
        });
    }

    let search_pattern = format!("%{}%", q);

    let brands = sqlx::query_as!(
        BrandResponseDTO,
        r#"
        SELECT id, name_en, name_ar, created_at, updated_at
        FROM brands
        WHERE name_en ILIKE $1 
           OR normalize_text(name_ar) LIKE normalize_text($1)
        ORDER BY name_ar ASC
        LIMIT 10
        "#,
        search_pattern
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(BrandSearchResultsTemplate {
        brands,
        query: q.to_string(),
    })
}

// ============================================================================
// 4. CREATE BRAND FLOW
// ============================================================================

/// Renders the empty "Create Brand" form page (GET /web/brands/new)
pub async fn render_new_brand_page() -> impl IntoResponse {
    CreateBrandTemplate {
        form: FormDTO::default(),
        errors: None,
        current_page: "brands".to_string(),
        error_message: None,
        success_message: None,
    }
}

/// Handles brand creation submission (POST /web/brands)
pub async fn create_brand(
    State(state): State<AppState>,
    Form(mut form): Form<FormDTO>,
) -> impl IntoResponse {
    form.sanitize();

    if let Err(errors) = form.validate() {
        return CreateBrandTemplate {
            form,
            errors: Some(errors),
            error_message: Some("من فضلك عدل الأخطاء لاستكمال التسجيل".to_string()),
            success_message: None,
            current_page: "brands".to_string(),
        }
        .into_response();
    }

    // 🔍 التحقق المسبق: هل الاسم (عربي أو إنجليزي) موجود مسبقاً في أي من الخانتين؟
    let duplicate_check = sqlx::query!(
        r#"
        SELECT id FROM brands 
        WHERE normalize_text(name_en) = normalize_text($1) 
           OR normalize_text(name_ar) = normalize_text($1)
           OR normalize_text(name_en) = normalize_text($2) 
           OR normalize_text(name_ar) = normalize_text($2)
        LIMIT 1
        "#,
        form.name_en,
        form.name_ar
    )
    .fetch_optional(&state.pool)
    .await;

    match duplicate_check {
        Ok(Some(_)) => {
            return CreateBrandTemplate {
                form,
                errors: None,
                error_message: Some("هذا الاسم (أو ما يطابقه بالحروف) مسجل بالفعل في الماركات".to_string()),
                success_message: None,
                current_page: "brands".to_string(),
            }
            .into_response();
        }
        Err(err) => return BrandError::Sqlx(err).into_response(),
        Ok(None) => {} // لا يوجد تكرار، نكمل عملية الإدخال طبيعياً
    }

    let insert_result = sqlx::query!(
        r#"
        INSERT INTO brands (name_en, name_ar)
        VALUES ($1, $2)
        "#,
        form.name_en,
        form.name_ar,
    )
    .execute(&state.pool)
    .await;

    match insert_result {
        Ok(_) => Redirect::to("/web/brands?action=created").into_response(),
        Err(err) => BrandError::Sqlx(err).into_response(),
    }
}

// ============================================================================
// 5. EDIT BRAND FLOW
// ============================================================================

/// Renders the edit brand form page (GET /web/brands/{id}/edit)
pub async fn render_edit_page(
    State(state): State<AppState>, 
    Path(id): Path<i64>
) -> Result<impl IntoResponse, BrandError> {
    let brand = sqlx::query_as!(
        BrandResponseDTO,
        r#"SELECT id, name_en, name_ar, created_at, updated_at FROM brands WHERE id = $1"#,
        id
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(BrandError::NotFound)?;

    let form = FormDTO {
        name_en: brand.name_en.clone(),
        name_ar: brand.name_ar.clone(),
    };

    Ok(EditBrandTemplate {
        id,
        brand: Some(brand),
        form,
        errors: None,
        error_message: None,
        success_message: None,
        current_page: "brand".to_string(),
    })
}

/// Handles brand update submission (POST /web/brands/{id}/edit)
pub async fn edit_brand(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(mut form): Form<FormDTO>,
) -> impl IntoResponse {
    form.sanitize();

    if let Err(errors) = form.validate() {
        let brand = sqlx::query_as!(
            BrandResponseDTO,
            r#"SELECT id, name_en, name_ar, created_at, updated_at FROM brands WHERE id = $1"#,
            id
        )
        .fetch_optional(&state.pool)
        .await
        .ok()
        .flatten();

        return EditBrandTemplate {
            id,
            brand,
            form,
            errors: Some(errors),
            error_message: Some("من فضلك عدل الأخطاء لاستكمال التعديل".to_string()),
            success_message: None,
            current_page: "brand".to_string(),
        }
        .into_response();
    }

    // 🔍 التحقق المسبق للتعديل: استثناء السجل الحالي (id != $3)
    let duplicate_check = sqlx::query!(
        r#"
        SELECT id FROM brands 
        WHERE (
            normalize_text(name_en) = normalize_text($1) 
            OR normalize_text(name_ar) = normalize_text($1)
            OR normalize_text(name_en) = normalize_text($2) 
            OR normalize_text(name_ar) = normalize_text($2)
        ) AND id != $3
        LIMIT 1
        "#,
        form.name_en,
        form.name_ar,
        id
    )
    .fetch_optional(&state.pool)
    .await;

    match duplicate_check {
        Ok(Some(_)) => {
            let brand = sqlx::query_as!(
                BrandResponseDTO,
                r#"SELECT id, name_en, name_ar, created_at, updated_at FROM brands WHERE id = $1"#,
                id
            )
            .fetch_optional(&state.pool)
            .await
            .ok()
            .flatten();

            return EditBrandTemplate {
                id,
                brand,
                form,
                errors: None,
                error_message: Some("هذا الاسم مستخدم بالفعل في ماركة أخرى".to_string()),
                success_message: None,
                current_page: "brand".to_string(),
            }
            .into_response();
        }
        Err(err) => return BrandError::Sqlx(err).into_response(),
        Ok(None) => {}
    }

    let update_result = sqlx::query!(
        r#"
        UPDATE brands 
        SET name_en = $1, name_ar = $2 
        WHERE id = $3
        "#,
        form.name_en,
        form.name_ar,
        id
    )
    .execute(&state.pool)
    .await;

    match update_result {
        Ok(_) => Redirect::to("/web/brands?action=updated").into_response(),
        Err(err) => BrandError::Sqlx(err).into_response(),
    }
}

// ============================================================================
// 6. DELETE BRAND
// ============================================================================

pub async fn delete_brand(
    State(state): State<AppState>, 
    Path(id): Path<i64>
) -> Result<impl IntoResponse, BrandError> {
    sqlx::query!(r#"DELETE FROM brands WHERE id = $1"#, id)
        .execute(&state.pool)
        .await?;

    Ok(Redirect::to("/web/brands?action=deleted"))
}