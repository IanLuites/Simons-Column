//! Orchestrator

use std::{
    io::Read,
    process::{Child, Command, ExitStatus, Stdio},
    time::Duration,
};

use tracing::{debug, info};

use crate::{choreography::Choreography, config::Config};

/// Current orchestrator status.
#[derive(Debug, PartialEq, Eq, serde::Serialize)]
pub enum Status {
    /// Playing a choreography.
    Playing,

    /// Finished playing a choreography successfully.
    Stopped,

    /// Errored while playing a choreography.
    Errored,
}

/// Orchestrator state info.
#[derive(Debug, Clone)]
pub struct Info {
    /// Name of the currently or last played choreography.
    choreography: String,

    /// Logs from the execution.
    stdout: String,

    /// Error logs from the execution.
    stderr: String,

    /// Exit status.
    status: Option<ExitStatus>,
}

impl Info {
    /// New
    #[must_use]
    fn new(name: &str) -> Self {
        Self {
            choreography: name.into(),
            stdout: String::new(),
            stderr: String::new(),
            status: None,
        }
    }

    /// Choreography
    #[must_use]
    pub fn choreography(&self) -> &str {
        &self.choreography
    }

    /// Log
    #[must_use]
    pub fn log(&self) -> String {
        format!("{}{}", self.stdout, self.stderr)
    }

    /// Status
    #[must_use]
    pub fn status(&self) -> Status {
        match self.status {
            None => Status::Playing,
            Some(result) if result.success() => Status::Stopped,
            _ => Status::Errored,
        }
    }
}

/// Orchestrator that manages the execution of light choreography.
#[derive(Debug)]
pub struct Orchestrator {
    /// Choreography timeout. (Not implemented)
    _timeout: Duration,

    /// Currently executing choreography process.
    current: Option<Child>,

    /// Info
    info: Info,
}

impl Orchestrator {
    /// Create a new orchestrator.
    #[must_use]
    pub fn new(config: &Config) -> Self {
        let mut info = Info::new("Startup");
        info.status = Some(ExitStatus::default());

        Self {
            _timeout: config.timeout(),
            current: None,
            info,
        }
    }

    /// Get current info.
    #[must_use]
    pub fn info(&mut self) -> Info {
        self.process();

        self.info.clone()
    }

    /// Start playing a choreography
    pub fn start(&mut self, choreography: &Choreography) {
        self.stop();

        info!(
            "Start playing: {:#} ({:#?})",
            choreography.name(),
            choreography.format()
        );

        // Hardcode python for now
        std::fs::write("run.py", choreography.compile()).expect("write choreography script");
        let child = Command::new("python3")
            .current_dir(".")
            .args(["run.py"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("child");

        self.current = Some(child);
        self.info = Info::new(choreography.name());
    }

    /// Stop the currently playing choreography.
    pub fn stop(&mut self) {
        self.process();

        if let Some(child) = self.current.as_mut() {
            let _ = child.kill();
            self.process();
        }
    }

    /// Process current running child info.
    ///
    /// Cleans up if child exited.
    fn process(&mut self) {
        if let Some(child) = self.current.as_mut() {
            if let Some(status) = child.try_wait().expect("process status") {
                debug!(
                    "Ended playing: {:#} ({:?})",
                    self.info.choreography,
                    status.code().unwrap_or(0)
                );

                if let Some(stdout) = child.stdout.as_mut() {
                    stdout
                        .read_to_string(&mut self.info.stdout)
                        .expect("read stdout");
                }
                if let Some(stderr) = child.stderr.as_mut() {
                    stderr
                        .read_to_string(&mut self.info.stderr)
                        .expect("read stderr");
                }

                self.current = None;
                self.info.status = Some(status);
            }
        }
    }
}
