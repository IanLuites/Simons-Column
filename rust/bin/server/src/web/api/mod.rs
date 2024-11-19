//! Web API

use axum::Router;

use crate::config::Config;

mod status;

/// API router
pub fn router(_config: &Config) -> Router {
    Router::new().route("/status", axum::routing::get(status::status))
}
