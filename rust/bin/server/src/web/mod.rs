//! Web part of server.

use tokio::net::TcpListener;

use crate::config::Config;

mod api;

/// Server Web requests
pub async fn serve(config: &Config) -> std::io::Result<()> {
    let app = axum::Router::new().nest_service("/api", api::router(config));
    let listener = TcpListener::bind(config.addr()).await?;

    axum::serve(listener, app).await
}
