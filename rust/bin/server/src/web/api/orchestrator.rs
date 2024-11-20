//! Orchestrator API managing playing of choreography

use axum::{extract::State, routing::get, Json, Router};

use crate::{
    orchestrator::{self, Info},
    web::state::WebState,
};

/// V1 routes
pub fn v1() -> Router<WebState> {
    Router::new().route("/", get(status).post(start).delete(stop))
}

/// Orchestrator status
async fn status(State(state): State<WebState>) -> Json<Status> {
    state.orchestrator().info().into()
}

/// Start playing a choreography
async fn start(State(state): State<WebState>, Json(start): Json<StartRequest>) -> Json<Status> {
    let choreography = state.choreography().read(&start.choreography);

    state.orchestrator().start(&choreography.unwrap()).into()
}

/// Stop a playing choreography
async fn stop(State(state): State<WebState>) -> Json<Status> {
    state.orchestrator().stop().into()
}

impl From<Info> for Json<Status> {
    fn from(info: Info) -> Self {
        Self(Status {
            choreography: info.choreography().into(),
            status: info.status(),
            log: info.log(),
        })
    }
}

/// Status info
#[derive(Debug, serde::Serialize)]
pub struct Status {
    /// Currently playing choreography.
    choreography: String,

    /// Status.
    #[allow(clippy::struct_field_names)]
    status: orchestrator::Status,

    /// Choreography log.
    log: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct StartRequest {
    /// Choreography to play.
    choreography: String,
}
