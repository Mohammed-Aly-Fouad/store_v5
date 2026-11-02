use askama::Template;
use askama_web::WebTemplate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;



#[derive(Deserialize, Serialize, FromRow, Clone, Debug)]
pub struct CategoryResponseDTO {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub parent_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryTree {
    pub main: CategoryResponseDTO,
    pub subs: Vec<CategoryResponseDTO>,
}

impl CategoryTree {
    pub fn build_tree(categories: Vec<CategoryResponseDTO>) -> Vec<Self> {
        let (mains, subs): (Vec<_>, Vec<_>) =
            categories.into_iter().partition(|c| c.parent_id.is_none());

        mains
            .into_iter()
            .map(|main_cat| {
                let child_subs = subs
                    .iter()
                    .filter(|s| s.parent_id == Some(main_cat.id))
                    .cloned()
                    .collect();

                CategoryTree {
                    main: main_cat,
                    subs: child_subs,
                }
            })
            .collect()
    }
}



// ============================================================================
// SECTION 5: ASKAMA TEMPLATES & UI RENDERING
// ============================================================================

/// Renders the main category management dashboard (`categorys/index.html`).
#[derive(Template, WebTemplate)]
#[template(path = "categories/index.html")]
pub struct CategoryTemplate {
    pub category_tree: Vec<CategoryTree>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,
}

// ============================================================================
// SECTION 4: QUERY PARAMETERS DTOs
// ============================================================================

/// URL Query parameter extractor for brand live-search requests.
#[derive(Debug, Deserialize)]
pub struct CategorySearchQuery {
    #[serde(default)]
    pub q: String,
}

/// Query parameters used to carry flash messages across redirects (`?action=created&error=not_found`).
#[derive(Debug, Deserialize)]
pub struct CategoryFlashParams {
    pub action: Option<String>,
    pub error: Option<String>,
}



/// Partial HTML snippet template for HTMX/Dynamic live category search.
#[derive(Template, WebTemplate)]
#[template(path = "categories/partials/search_results.html")]
pub struct CategorySearchResultsTemplate {
    pub categories: Vec<CategoryResponseDTO>,
    pub query: String,
}