//! Choreography API managing choreography

use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};

use crate::{
    choreography::{Choreography, Format},
    web::state::WebState,
};

/// V1 routes
pub fn v1() -> Router<WebState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:name", get(read))
        .route("/:name", put(update))
}

/// Choreography list
async fn list(State(state): State<WebState>) -> Json<List> {
    Json(List {
        choreography: state
            .choreography()
            .list()
            .into_iter()
            .map(|c| Info {
                name: c.name().into(),
                format: c.format(),
            })
            .collect(),
    })
}

/// Create choreography
async fn create(
    State(state): State<WebState>,
    Json(choreography): Json<Choreography>,
) -> Json<Choreography> {
    state.choreography().write(&choreography);
    Json(choreography)
}

/// Fetch choreography
async fn read(
    State(state): State<WebState>,
    Path(name): Path<String>,
) -> Json<Option<Choreography>> {
    Json(state.choreography().read(name))
}

/// Update choreography
async fn update(
    State(state): State<WebState>,
    Path(name): Path<String>,
    Json(update): Json<Update>,
) -> Json<Choreography> {
    let mut choreography = state.choreography().read(name).unwrap();

    if let Some(rename) = update.name {
        choreography = state.choreography().rename(choreography, rename);
    }

    if let Some(data) = update.data {
        let format = update.format.unwrap_or_else(|| choreography.format());
        choreography = state.choreography().update(choreography, format, data);
    }

    Json(choreography)
}

/// Choreography list
#[derive(Debug, serde::Serialize)]
pub struct List {
    /// Choreographies
    choreography: Vec<Info>,
}

#[derive(Debug, serde::Serialize)]
pub struct Info {
    /// Choreography name.
    name: String,

    /// Choreography format.
    format: Format,
}

/// Choreography update
#[derive(Debug, serde::Deserialize)]
pub struct Update {
    /// Rename choreography
    name: Option<String>,

    /// Change format.
    format: Option<Format>,

    /// Update choreography data.
    data: Option<String>,
}
