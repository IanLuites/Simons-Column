//! Benchmark config

use crate::benchmark::{self, Iterations, Suite};

mod cli;
mod toml;

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
#[derive(Debug)]
pub struct Config {
    /// Benchmarks
    benchmarks: Vec<Suite>,

    /// Implementations
    implementations: Vec<crate::Implementation>,
}

/// Combined application config.
#[derive(Debug)]
struct Builder {
    /// Strict mode, panicking on invalid config.
    strict: bool,

    /// Implementation directory.
    implementation_directory: Option<std::path::PathBuf>,

    /// Iterations.
    iterations: Iterations,

    /// Benchmarks
    benchmarks: Vec<benchmark::Builder>,

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

impl Builder {
    /// Parse config and load from all sources.
    #[must_use]
    pub fn parse() -> Self {
        let cli = cli::parse();
        let mut config = Self {
            strict: cli.strict,
            implementation_directory: cli.implementation_directory,
            iterations: Iterations::default(),
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

    /// Load toml config.
    fn from_toml(toml: impl AsRef<str>) -> Self {
        toml::from_str(toml)
    }

    /// Load toml config file.
    fn from_toml_file(file: impl AsRef<std::path::Path>) -> Self {
        Self::from_toml(std::fs::read_to_string(file.as_ref()).expect("config"))
    }
    /// Merge another config into self.
    fn merge(&mut self, mut other: Self) {
        self.strict |= other.strict;
        self.iterations |= self.iterations;
        self.benchmarks.append(&mut other.benchmarks);
        self.implementations.append(&mut other.implementations);
    }

    /// Generate a fully parsed config.
    #[must_use]
    pub fn build(self) -> Config {
        let mut benchmarks = Vec::with_capacity(self.benchmarks.len());

        for mut builder in self.benchmarks {
            if self.iterations.has_warmup() {
                builder = builder.warmup(self.iterations.warmup());
            }
            if self.iterations.has_benchmark() {
                builder = builder.iterations(self.iterations.benchmark().get());
            }

            let benchmark = builder.build();

            if benchmarks.iter().any(|s: &Suite| s.id() == benchmark.id()) {
                error!(
                    self.strict,
                    "duplicate; skipping benchmark with id {:?}",
                    benchmark.id()
                );

                continue;
            }

            benchmarks.push(benchmark);
        }

        let implementation_directory = self
            .implementation_directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().expect("working directory"));

        let mut implementations = Vec::with_capacity(self.implementations.len());

        for implementation in self.implementations {
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
                builder = builder.label(label);
            }
            if let Some(directory) = &implementation.directory {
                builder = builder.directory(directory);
            }

            implementations.push(builder.build());
        }

        Config {
            benchmarks,
            implementations,
        }
    }
}

impl Config {
    #[cfg(test)]
    /// Load toml config.
    fn from_toml(toml: impl AsRef<str>) -> Self {
        Builder::from_toml(toml).build()
    }

    /// Parse config and load from all sources.
    #[must_use]
    pub fn parse() -> Self {
        Builder::parse().build()
    }

    /// Configured benchmark suites.
    #[must_use]
    pub fn benchmarks(&self) -> &[Suite] {
        &self.benchmarks
    }

    /// Configured benchmark implementations.
    #[must_use]
    pub fn implementations(&self) -> &[crate::Implementation] {
        &self.implementations
    }
}

#[cfg(test)]
mod tests {
    use crate::benchmark::Suite;

    use super::*;

    #[test]
    fn from_toml() {
        const TOML: &str = r#"
[benchmarks]
  custom = { warmup = 0, iterations = 100 }
  shift_bit = { label = "Shift Single Bit" }

  [benchmarks.single_pin]
    label = "Single Pin"

    [benchmarks.single_pin.matrix]
      gpio17 = { args = [17] }
      gpio22 = { args = [22] }
      gpio27 = { args = [27] }
"#;

        let expected = [
            Suite::build("shift_bit").label("Shift Single Bit").build(),
            Suite::build("single_pin")
                .label("Single Pin")
                .case("gpio17", |case| case.arg("17"))
                .case("gpio22", |case| case.arg("22"))
                .case("gpio27", |case| case.arg("27"))
                .build(),
            Suite::build("custom")
                .warmup(0_usize)
                .iterations(100_usize)
                .build(),
        ];

        let config = Config::from_toml(TOML);
        let benchmarks = config.benchmarks();

        assert_eq!(benchmarks.len(), expected.len());

        for expect in &expected {
            let matching = benchmarks
                .iter()
                .find(|b| b.id() == expect.id())
                .expect("matching suite");

            for case in expect {
                assert!(
                    matching.cases().contains(&case),
                    "Missing: {case:#?} in {matching:#?}"
                );
            }
        }
    }
}
