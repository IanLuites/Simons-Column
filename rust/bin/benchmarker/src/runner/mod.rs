//! Benchmark runner

use crate::config::Config;

mod execution;
mod output;
mod results;

pub use results::Results;

/// Benchmark runner.
#[derive(Debug)]
pub struct Runner {
    /// Runner config.
    config: Config,
}

impl Runner {
    /// Create a new runner.
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Start benchmarks
    pub fn start(&self) -> std::io::Result<Results> {
        let mut progress = ProgressWriter::new(true);
        let mut results = Results::default();

        progress.overview(&self.config)?;
        progress.start()?;

        for implementation in self.config.implementations() {
            progress.start_implementation(implementation)?;

            for suite in self.config.benchmarks() {
                if !execution::implements_cases(implementation, suite).any(|_| true) {
                    continue;
                }
                progress.start_suite(suite)?;

                for case in execution::implements_cases(implementation, suite) {
                    progress.start_case(&case)?;

                    if let Some(timings) = execution::run(implementation, &case).timings() {
                        results.record(&case, implementation, timings);
                        progress.complete_case(true)?;
                    } else {
                        progress.complete_case(false)?;
                    }
                }
            }
        }

        Ok(results)
    }
}

/// Write progress to stdout.
enum ProgressWriter {
    /// Do not write progress
    Disabled,
    /// Enabled, writing to stdout.
    Stdout(std::io::Stdout, Option<String>),
}
use std::io::Write;

impl ProgressWriter {
    /// Create a new progress writer.
    #[must_use]
    fn new(enabled: bool) -> Self {
        if enabled {
            Self::Stdout(std::io::stdout(), None)
        } else {
            Self::Disabled
        }
    }

    /// Print a CLI overview before running benchmarks.
    fn overview(&mut self, config: &Config) -> std::io::Result<()> {
        if let Self::Stdout(out, _) = self {
            writeln!(out, "Benchmark suites:\n")?;
            for suite in config.benchmarks() {
                writeln!(out, "  - {}", suite.label())?;
            }

            writeln!(out)?;

            writeln!(out, "Implementations:\n")?;
            for implementation in config.implementations() {
                writeln!(out, "  - {}", implementation.label())?;
            }

            writeln!(out)?;
        }
        Ok(())
    }

    /// Progress started
    fn start(&mut self) -> std::io::Result<()> {
        if let Self::Stdout(out, _) = self {
            writeln!(out, "Running:\n")?;
        }

        Ok(())
    }

    /// Start running on a specific implementation.
    fn start_implementation(
        &mut self,
        implementation: &crate::Implementation,
    ) -> std::io::Result<()> {
        if let Self::Stdout(out, _) = self {
            writeln!(out, "📂  {}", implementation.label())?;
        }

        Ok(())
    }

    /// Start a new suite.
    fn start_suite(&mut self, suite: &crate::benchmark::Suite) -> std::io::Result<()> {
        if let Self::Stdout(out, current) = self {
            if suite.cases().count() == 1 {
                let label = format!("    {}", suite.short_label());
                write!(out, "⏳{label}")?;
                *current = Some(label);

                out.flush()?;
            } else {
                writeln!(out, "📂    {}", suite.label())?;
            }
        }

        Ok(())
    }

    /// Start a new case.
    fn start_case(&mut self, case: &crate::benchmark::Case) -> std::io::Result<()> {
        if let Self::Stdout(out, current) = self {
            if case.is_matrix() {
                let label = format!("      {}", case.short_label());
                write!(out, "⏳{label}")?;
                *current = Some(label);
                out.flush()?;
            }
        }

        Ok(())
    }

    /// Start a new case.
    fn complete_case(&mut self, success: bool) -> std::io::Result<()> {
        if let Self::Stdout(out, current) = self {
            let mut label = None;
            std::mem::swap(current, &mut label);
            let label = label.expect("a case label to be set");

            writeln!(out, "\r{}{label}", if success { '✅' } else { '❌' })?;
        }

        Ok(())
    }
}
