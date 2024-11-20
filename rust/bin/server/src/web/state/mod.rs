//! Web shared state

mod shared_orchestrator;
pub use shared_orchestrator::SharedOrchestrator;

use crate::{choreography::Storage, config::Config};

/// Shared web state.
#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct WebState {
    /// Storage
    choreography: std::sync::Arc<Storage>,

    /// Shared state orchestrator.
    orchestrator: SharedOrchestrator,
}

impl WebState {
    /// Create a new shared orchestrator.
    #[must_use]
    pub fn new(config: &Config) -> Self {
        Self {
            choreography: std::sync::Arc::new(Storage::open(config.storage())),
            orchestrator: SharedOrchestrator::new(config),
        }
    }

    /// Choreography storage.
    #[must_use]
    pub fn choreography(&self) -> &Storage {
        &self.choreography
    }

    /// Shared orchestrator.
    #[must_use]
    pub const fn orchestrator(&self) -> &SharedOrchestrator {
        &self.orchestrator
    }
}
