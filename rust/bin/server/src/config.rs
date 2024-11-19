//! Server config

use std::time::Duration;

use clap::Parser;

/// Server config
#[derive(Debug, Parser)]
pub struct Config {
    // Orchestrator
    /// Choreography timeout in seconds.
    #[arg(short, long, default_value_t = 60.0)]
    timeout: f64,

    // Web
    /// Bind address
    #[arg(short, long, default_value = "0.0.0.0")]
    address: String,

    /// Bind port
    #[arg(short, long)]
    #[cfg_attr(debug_assertions, arg(default_value_t = 8080))]
    #[cfg_attr(not(debug_assertions), arg(default_value_t = 80))]
    port: u16,
}

impl Config {
    /// Load config from all sources.
    #[must_use]
    pub fn load() -> Self {
        Self::parse()
    }

    /// Bind address for HTTP server.
    #[must_use]
    pub fn addr(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    /// Choreography timeout.
    #[must_use]
    pub fn timeout(&self) -> Duration {
        Duration::from_secs_f64(self.timeout)
    }
}
