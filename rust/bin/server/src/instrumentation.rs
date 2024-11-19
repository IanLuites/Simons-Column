//! Logging, metrics, and tracing.

use tracing::info;

use crate::config::Config;

/// Setup logging, metrics, and tracing.
pub fn setup(config: &Config) {
    tracing_subscriber::fmt::init();

    info!("Starting Simon's Column server on {}", config.addr());
}
