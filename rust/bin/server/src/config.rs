//! Server config

use clap::Parser;

/// Server config
#[derive(Debug, Parser)]
pub struct Config {}

impl Config {
    /// Load config from all sources.
    #[must_use]
    pub fn load() -> Self {
        Self::parse()
    }
}
