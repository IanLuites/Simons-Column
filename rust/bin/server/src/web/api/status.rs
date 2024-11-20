//! Status endpoint

use axum::Json;

/// Status info
#[derive(Debug, serde::Serialize)]
pub struct Status {
    /// Server version
    version: String,
}

/// Server status.
pub async fn status() -> Json<Status> {
    Json(Status {
        version: std::env!("CARGO_PKG_VERSION").to_string(),
    })
}
