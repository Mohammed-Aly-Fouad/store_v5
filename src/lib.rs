pub mod error;
pub mod web;
pub mod domain;
pub mod startup;
pub mod state;
pub mod utils;

use axum::Router;
use state::AppState;
use tower_http::cors::CorsLayer;
use web::router;

/// Builds the complete Axum application router with all middleware, nests, and shared state.
/// This is separated into a library function so it can be reused in integration tests.
pub fn create_router(state: AppState) -> Router {
    // Configure CORS layers
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    Router::new()
        .nest("/web", router())
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .layer(cors)
        .with_state(state)
}