//! Benchmark output.

/// Execution output.
#[derive(Debug)]
pub struct Output {
    /// Execution success.
    success: bool,

    /// Run log. All output over
    _log: String,

    /// Benchmark timings. (results)
    timings: Option<Timings>,
}

#[derive(Debug, Clone, Copy)]
pub struct Timings {
    /// Total execution time in nanoseconds.
    pub execution: u128,

    /// Warmup time in nanoseconds.
    pub warmup: u128,

    /// Benchmark time in nanoseconds.
    pub benchmark: u128,
}

impl Output {
    /// Success
    #[must_use]
    pub const fn success(log: String, timings: Timings) -> Self {
        Self {
            success: true,
            _log: log,
            timings: Some(timings),
        }
    }

    /// Error: implementation not found.
    #[must_use]
    pub fn implementation_not_found() -> Self {
        Self::failure("Implementation not found.")
    }

    /// Failure to run.
    #[must_use]
    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            _log: error.into(),
            timings: None,
        }
    }

    /// Return timings on success.
    #[must_use]
    pub const fn timings(&self) -> Option<Timings> {
        if self.success {
            self.timings
        } else {
            None
        }
    }
}
