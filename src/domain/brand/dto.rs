use askama::Template;
use askama_web::WebTemplate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;



// ============================================================================
// SECTION 2: DATABASE RESPONSE & LIST DTOs
// ============================================================================

/// Lightweight DTO for dropdowns or quick brand lists.
#[derive(Debug, Serialize, FromRow, Clone)]
pub struct BrandsListDTO {
    pub id: i64,
    pub name_ar: String,
}

/// Full database response DTO representing a brand record.
#[derive(Debug, Serialize, FromRow, Clone, Deserialize)]
pub struct BrandResponseDTO {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// SECTION 3: FORM DTOs, VALIDATION & ERRORS
// ============================================================================

/// Holds field-level validation error messages for the brand form.
#[derive(Default, Debug, Serialize, Clone)]
pub struct FormErrors {
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
}

impl FormErrors {
    pub fn has_errors(&self) -> bool {
        self.name_ar.is_some() || self.name_en.is_some()
    }
}

/// Incoming form payload for creating or editing a brand.
#[derive(Debug, Deserialize, Default, Serialize, Clone)]
pub struct FormDTO {
    pub name_ar: String,
    pub name_en: String,
}

impl FormDTO {
    /// Sanitize user input by trimming whitespace across all text fields.
    pub fn sanitize(&mut self) {
        self.name_ar = self.name_ar.trim().to_string();
        self.name_en = self.name_en.trim().to_string();
    }

    /// Validate form inputs and return structured field errors if any fail.
    pub fn validate(&self) -> Result<(), FormErrors> {
        let mut errors = FormErrors::default();

        let name_ar = self.name_ar.trim();
        if name_ar.is_empty() {
            errors.name_ar = Some("لا يمكن ترك الاسم خالياً".to_string());
        } else if name_ar.chars().count() > 100 {
            errors.name_ar = Some("لا يمكن أن تتعدى خانة الاسم 100 حرف".to_string());
        }

        let name_en = self.name_en.trim();
        if name_en.is_empty() {
            errors.name_en = Some("This field is not allowed to be empty".to_string());
        } else if name_en.chars().count() > 100 {
            errors.name_en = Some("This field is not allowed to be more than 100 characters".to_string());
        }

        if errors.has_errors() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// SECTION 4: QUERY PARAMETERS DTOs
// ============================================================================

/// URL Query parameter extractor for brand live-search requests.
#[derive(Debug, Deserialize)]
pub struct BrandSearchQuery {
    #[serde(default)]
    pub q: String,
}

/// Query parameters used to carry flash messages across redirects (`?action=created&error=not_found`).
#[derive(Debug, Deserialize)]
pub struct BrandFlashParams {
    pub action: Option<String>,
    pub error: Option<String>,
}

// ============================================================================
// SECTION 5: ASKAMA TEMPLATES & UI RENDERING
// ============================================================================

/// Renders the main brand management dashboard (`brands/index.html`).
#[derive(Template, WebTemplate)]
#[template(path = "brands/index.html")]
pub struct BrandsTemplate {
    pub brands: Vec<BrandResponseDTO>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,
}

/// Renders the "Create Brand" form page (supports form re-population and field errors).
#[derive(Template, WebTemplate)]
#[template(path = "brands/create.html")]
pub struct CreateBrandTemplate {
    pub form: FormDTO,
    pub errors: Option<FormErrors>,
    pub current_page: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
}

/// Renders the "Edit Brand" form page.
#[derive(Template, WebTemplate)]
#[template(path = "brands/edit.html")]
pub struct EditBrandTemplate {
    pub id: i64,
    pub brand: Option<BrandResponseDTO>,
    pub form: FormDTO,
    pub errors: Option<FormErrors>,
    pub current_page: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
}

/// Partial HTML snippet template for HTMX/Dynamic live brand search.
#[derive(Template, WebTemplate)]
#[template(path = "brands/partials/search_results.html")]
pub struct BrandSearchResultsTemplate {
    pub brands: Vec<BrandResponseDTO>,
    pub query: String,
}