//! Choreography API managing choreography

use axum::{
    extract::Path,
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
        .route("/:name", get(choreography))
        .route("/:name", put(update))
}

/// Choreography list
async fn list() -> Json<List> {
    Json(List {
        choreography: vec![],
    })
}

/// Create choreography
async fn create(Json(choreography): Json<Choreography>) -> Json<Choreography> {
    Json(choreography)
}

/// Fetch choreography
async fn choreography(Path(name): Path<String>) -> Json<Choreography> {
    Json(Choreography {
        name,
        format: Format::Python,
        data: String::new(),
    })
}

/// Update choreography
async fn update(Path(name): Path<String>, Json(_update): Json<Update>) -> Json<Choreography> {
    Json(Choreography {
        name,
        format: Format::Python,
        data: String::new(),
    })
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
