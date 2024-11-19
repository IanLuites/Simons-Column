//! Web API

use axum::Router;

use crate::config::Config;

/// API router
pub fn router(_config: &Config) -> Router {
    Router::new()
}
