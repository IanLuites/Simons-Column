//! Web shared state

mod shared_orchestrator;
pub use shared_orchestrator::SharedOrchestrator;

use crate::config::Config;

/// Shared web state.
#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct WebState {
    /// Shared state orchestrator.
    orchestrator: SharedOrchestrator,
}

impl WebState {
    /// Create a new shared orchestrator.
    #[must_use]
    pub fn new(config: &Config) -> Self {
        Self {
            orchestrator: SharedOrchestrator::new(config),
        }
    }

    /// Shared orchestrator.
    #[must_use]
    pub const fn orchestrator(&self) -> &SharedOrchestrator {
        &self.orchestrator
    }
}
