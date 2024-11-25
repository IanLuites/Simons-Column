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

/// Combined application config.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// Warmup iterations.
    warmup_iterations: Option<usize>,

    /// Benchmark iterations.
    benchmark_iterations: Option<NonZeroUsize>,

    /// Benchmarks
    benchmarks: Vec<Benchmark>,
}

impl Config {
    // Reading

    /// Parse config and load from all sources.
    #[must_use]
    pub fn parse() -> Self {
        let cli = cli::parse();
        let mut config = Self {
            warmup_iterations: cli.warmup_iterations,
            benchmark_iterations: cli.benchmark_iterations,
            benchmarks: vec![],
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
        self.warmup_iterations = self.warmup_iterations.or(other.warmup_iterations);
        self.benchmark_iterations = self.benchmark_iterations.or(other.benchmark_iterations);
        self.benchmarks.append(&mut other.benchmarks);
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
    pub fn benchmarks(&self) -> Vec<crate::benchmark::Benchmark> {
        let mut benchmarks = Vec::with_capacity(self.benchmarks.len());

        for benchmark in &self.benchmarks {
            if benchmarks
                .iter()
                .any(|b: &crate::benchmark::Benchmark| b.id() == benchmark.id)
            {
                eprintln!(
                    "Warning: duplicate; skipping benchmark with id {:?} ",
                    benchmark.id
                );

                continue;
            }

            let mut builder = crate::benchmark::Benchmark::build(&benchmark.id);

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
}

#[cfg(test)]
mod tests {
    use crate::benchmark::Benchmark;

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
