//! Web API

use axum::Router;

use crate::config::Config;

use super::state::WebState;

mod choreography;
mod orchestrator;
mod status;

/// API router
pub fn router(_config: &Config) -> Router<WebState> {
    Router::new()
        .route("/status", axum::routing::get(status::status))
        .nest("/v1", v1())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_methods(tower_http::cors::Any)
                .allow_origin(tower_http::cors::Any)
                .allow_headers([axum::http::header::CONTENT_TYPE]),
        )
}

/// V1 routes
fn v1() -> Router<WebState> {
    Router::new()
        .nest("/choreography", choreography::v1())
        .nest("/orchestrator", orchestrator::v1())
}
