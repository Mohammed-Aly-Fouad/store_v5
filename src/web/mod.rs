pub mod brand;
pub mod category;
// pub mod home;
// pub mod product;
// pub mod product_variants;

use crate::{state::AppState};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        // .merge(home::router())
        .nest("/categories", category::router())
        // .nest("/products", product::router())
        .nest("/brands", brand::router())
        // .nest("/product_variants", product_variants::router())
}