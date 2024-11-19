//! Orchestrator API managing playing of choreography

use axum::{routing::get, Json, Router};

/// V1 routes
pub fn v1() -> Router {
    Router::new().route("/", get(status).post(start).delete(stop))
}

/// Orchestrator status
async fn status() -> Json<Status> {
    Json(Status { current: None })
}

/// Start playing a choreography
async fn start(Json(start): Json<StartRequest>) -> Json<Status> {
    Json(Status {
        current: Some(start.choreography),
    })
}

/// Stop a playing choreography
async fn stop() -> Json<Status> {
    Json(Status { current: None })
}

/// Status info
#[derive(Debug, serde::Serialize)]
pub struct Status {
    /// Currently playing choreography.
    current: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct StartRequest {
    /// Choreography to play.
    choreography: String,
}
