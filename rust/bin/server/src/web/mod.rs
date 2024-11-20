//! Web part of server.

use tokio::net::TcpListener;

use crate::config::Config;

mod api;
mod state;

/// Frontend static assets.
#[derive(rust_embed::RustEmbed, Clone)]
#[folder = "../../../node/ui/build"]
struct Frontend;

/// Server Web requests
pub async fn serve(config: &Config) -> std::io::Result<()> {
    let app = axum::Router::new()
        .nest_service("/", axum_embed::ServeEmbed::<Frontend>::new())
        .nest("/api", api::router(config))
        .with_state(state::WebState::new(config));

    let listener = TcpListener::bind(config.addr()).await?;

    axum::serve(listener, app).await
}
