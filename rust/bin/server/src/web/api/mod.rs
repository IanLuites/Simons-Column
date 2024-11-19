//! Web API

use axum::Router;

use crate::config::Config;

mod choreography;
mod orchestrator;
mod status;

/// API router
pub fn router(_config: &Config) -> Router {
    Router::new()
        .route("/status", axum::routing::get(status::status))
        .nest_service("/v1", v1())
}

/// V1 routes
fn v1() -> Router {
    Router::new()
        .nest("/choreography", choreography::v1())
        .nest("/orchestrator", orchestrator::v1())
}