//! Shared orchestrator access

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    choreography::Choreography,
    config::Config,
    orchestrator::{Info, Orchestrator},
};

/// Application state.
#[derive(Debug, Clone)]
pub struct SharedOrchestrator(Arc<RwLock<Orchestrator>>);

impl SharedOrchestrator {
    /// Create a new shared orchestrator.
    #[must_use]
    pub fn new(config: &Config) -> Self {
        Self(Arc::new(RwLock::new(Orchestrator::new(config))))
    }

    /// Orchestrator info.
    #[must_use]
    pub fn info(&self) -> Info {
        self.0
            .try_write()
            .expect("info access to orchestrator")
            .info()
    }

    /// Stop orchestrator and return info.
    #[must_use]
    pub fn stop(&self) -> Info {
        let mut orchestrator = self.0.try_write().expect("stop access to orchestrator");

        orchestrator.stop();
        orchestrator.info()
    }

    /// Orchestrator start and return info.
    #[must_use]
    pub fn start(&self, choreography: &Choreography) -> Info {
        let mut orchestrator = self.0.try_write().expect("start access to orchestrator");
        orchestrator.start(choreography);
        orchestrator.info()
    }
}
