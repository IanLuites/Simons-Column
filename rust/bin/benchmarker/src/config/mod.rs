//! Benchmark config

use std::num::NonZeroUsize;

use crate::benchmark::Arguments;

mod cli;
mod toml;

/// Benchmark config options.
#[derive(Debug, PartialEq, Eq)]
struct Benchmark {
    /// Benchmark id
    id: String,

    /// Label
    label: Option<String>,

    /// Warmup iterations.
    warmup_iterations: Option<usize>,

    /// Benchmark iterations.
    #[allow(clippy::struct_field_names)]
    benchmark_iterations: Option<NonZeroUsize>,

    /// Arguments
    arguments: Vec<Arguments>,
}

/// Implementation config options.
#[derive(Debug, PartialEq, Eq)]
struct Implementation {
    /// Implementation id
    id: String,

    /// Label
    label: Option<String>,

    /// Arguments
    directory: Option<std::path::PathBuf>,
}

/// Combined application config.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// Strict mode, panicking on invalid config.
    strict: bool,

    /// Implementation directory.
    implementation_directory: Option<std::path::PathBuf>,

    /// Warmup iterations.
    warmup_iterations: Option<usize>,

    /// Benchmark iterations.
    benchmark_iterations: Option<NonZeroUsize>,

    /// Benchmarks
    benchmarks: Vec<Benchmark>,

    /// Implementations
    implementations: Vec<Implementation>,
}

/// Config error.
///
/// Exits if config is marked as strict.
macro_rules! error {
    ($strict:expr, $fmt:expr $(, $($arg:tt)*)?) => {
        if $strict {
            eprintln!(concat!("Error: ", $fmt), $($($arg)*)?);
            std::process::exit(1)
        } else {
            eprintln!(concat!("Warning: ", $fmt), $($($arg)*)?);
        }
    };
}

impl Config {
    // Reading

    /// Parse config and load from all sources.
    #[must_use]
    pub fn parse() -> Self {
        let cli = cli::parse();
        let mut config = Self {
            strict: cli.strict,
            implementation_directory: cli.implementation_directory,
            warmup_iterations: cli.warmup_iterations,
            benchmark_iterations: cli.benchmark_iterations,
            benchmarks: vec![],
            implementations: vec![],
        };

        if cli.config_file.is_empty() {
            if std::fs::exists("config.toml").is_ok_and(|v| v) {
                config.merge(Self::from_toml_file("config.toml"));
            }
        } else {
            for file in cli.config_file {
                config.merge(Self::from_toml_file(file));
            }
        }

        config
    }

    /// Merge another config into self.
    fn merge(&mut self, mut other: Self) {
        self.strict |= other.strict;
        self.warmup_iterations = self.warmup_iterations.or(other.warmup_iterations);
        self.benchmark_iterations = self.benchmark_iterations.or(other.benchmark_iterations);
        self.benchmarks.append(&mut other.benchmarks);
        self.implementations.append(&mut other.implementations);
    }

    /// Load toml config.
    fn from_toml(toml: impl AsRef<str>) -> Self {
        toml::from_str(toml)
    }

    /// Load toml config file.
    fn from_toml_file(file: impl AsRef<std::path::Path>) -> Self {
        Self::from_toml(std::fs::read_to_string(file.as_ref()).expect("config"))
    }

    /// All configured benchmarks.
    #[must_use]
    pub fn benchmarks(&self) -> Vec<crate::Benchmark> {
        let mut benchmarks = Vec::with_capacity(self.benchmarks.len());

        for benchmark in &self.benchmarks {
            if benchmarks
                .iter()
                .any(|b: &crate::Benchmark| b.id() == benchmark.id)
            {
                error!(
                    self.strict,
                    "duplicate; skipping benchmark with id {:?}", benchmark.id
                );

                continue;
            }

            let mut builder = crate::Benchmark::build(&benchmark.id);

            for arguments in &benchmark.arguments {
                builder = builder.with_variant(arguments.clone());
            }

            if let Some(label) = &benchmark.label {
                builder = builder.with_label(label);
            }

            if let Some(warmup) = benchmark.warmup_iterations.or(self.warmup_iterations) {
                builder = builder.with_warmup(warmup);
            }

            if let Some(iterations) = benchmark.benchmark_iterations.or(self.benchmark_iterations) {
                builder = builder.with_iterations(iterations);
            }

            benchmarks.push(builder.build());
        }

        benchmarks
    }

    /// All configured implementations.
    #[must_use]
    pub fn implementations(&self) -> Vec<crate::Implementation> {
        let implementation_directory = self
            .implementation_directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().expect("working directory"));

        let mut implementations = Vec::with_capacity(self.implementations.len());

        for implementation in &self.implementations {
            if implementations
                .iter()
                .any(|b: &crate::Implementation| b.id() == implementation.id)
            {
                error!(
                    self.strict,
                    "duplicate; skipping implementation with id {:?}", implementation.id
                );

                continue;
            }

            let directory = implementation.directory.as_ref().map_or_else(
                || implementation_directory.join(&implementation.id),
                |custom| {
                    if custom.is_absolute() {
                        custom.clone()
                    } else {
                        implementation_directory.join(custom)
                    }
                },
            );

            if !directory.exists() {
                error!(
                    self.strict,
                    "implementation directory does not exist {:?}", directory
                );

                continue;
            }

            let mut builder = crate::Implementation::build(&implementation.id);
            if let Some(label) = &implementation.label {
                builder = builder.with_label(label);
            }
            if let Some(directory) = &implementation.directory {
                builder = builder.with_directory(directory);
            }

            implementations.push(builder.build());
        }

        implementations
    }
}

#[cfg(test)]
mod tests {
    use crate::Benchmark;

    use super::*;

    #[test]
    fn from_toml() {
        const TOML: &str = r#"
[benchmarks]
shift_bit = { label = "Shift Single Bit" }
single_pin = { label = "Single Pin", args = [17, 22, 27] }
custom = { warmup = 0, iterations = 100 }
"#;

        let expected = [
            Benchmark::build("shift_bit")
                .with_label("Shift Single Bit")
                .build(),
            Benchmark::build("single_pin")
                .with_label("Single Pin")
                .with_variant(["17"])
                .with_variant(["22"])
                .with_variant(["27"])
                .build(),
            Benchmark::build("custom")
                .with_warmup(0_usize)
                .with_iterations(100_usize)
                .build(),
        ];

        let config = Config::from_toml(TOML);
        let benchmarks = config.benchmarks();

        assert_eq!(benchmarks.len(), expected.len());

        for expect in &expected {
            assert!(
                benchmarks.contains(expect),
                "Missing: {expect:#?} in {benchmarks:#?}"
            );
        }
    }
}
