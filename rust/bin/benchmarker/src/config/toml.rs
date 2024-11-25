//! TOML based config

use std::{collections::HashMap, num::NonZeroUsize};

use serde::Deserialize;

use crate::benchmark::Arguments;

use super::{Benchmark, Implementation};

/// Arguments
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Args {
    /// List of arguments.
    List(Vec<Arg>),
    /// Arguments whitespace delimited.
    String(String),
    /// Single value.
    Value(Arg),
}

impl From<Args> for Arguments {
    fn from(value: Args) -> Self {
        match value {
            Args::List(list) => Self::from_iter(list.into_iter()),
            Args::String(arg) => Self::from_iter(arg.split_ascii_whitespace()),
            Args::Value(arg) => Self::from_iter(std::iter::once(arg.to_string())),
        }
    }
}

/// Single argument value.
///
/// All converted to string, only split by type for parsing.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Arg {
    /// Bool value.
    Bool(bool),
    /// Floating point.
    Float(f64),
    /// Signed integer.
    Signed(i64),
    /// Unsigned integer.
    Unsigned(u64),
    /// String value.
    String(String),
}

impl std::fmt::Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => v.fmt(f),
            Self::Float(v) => v.fmt(f),
            Self::Signed(v) => v.fmt(f),
            Self::Unsigned(v) => v.fmt(f),
            Self::String(v) => v.fmt(f),
        }
    }
}

/// Benchmark options.
#[derive(Debug, Deserialize)]
struct BenchmarkConfig {
    /// Label
    label: Option<String>,

    /// Warmup iterations
    warmup: Option<usize>,

    /// Benchmark iterations
    iterations: Option<NonZeroUsize>,

    /// Arguments
    #[serde(default)]
    args: Vec<Args>,
}
/// Implementation options.
#[derive(Debug, Deserialize)]
struct ImplementationConfig {
    /// Label
    label: Option<String>,

    /// Implementation directory, defaults to id.
    directory: Option<std::path::PathBuf>,
}

/// Config defaults.
#[derive(Debug, Default, Deserialize)]
struct Defaults {
    /// Warmup iterations
    warmup: Option<usize>,

    /// Benchmark iterations
    iterations: Option<NonZeroUsize>,
}

/// TOML config format.
#[derive(Debug, Deserialize)]
struct Config {
    /// Strict mode
    #[serde(default)]
    strict: bool,

    /// Implementation directory.
    #[serde(default)]
    implementation_directory: Option<std::path::PathBuf>,

    /// Config defaults.
    #[serde(default)]
    defaults: Defaults,

    /// Benchmark definitions
    benchmarks: HashMap<String, BenchmarkConfig>,

    /// Implementation definitions
    implementations: HashMap<String, ImplementationConfig>,
}

/// Read TOML config from str.
#[must_use]
pub fn from_str(toml: impl AsRef<str>) -> super::Config {
    let config: Config = toml::from_str(toml.as_ref()).expect("a");

    super::Config {
        strict: config.strict,
        implementation_directory: config.implementation_directory,
        warmup_iterations: config.defaults.warmup,
        benchmark_iterations: config.defaults.iterations,
        benchmarks: config
            .benchmarks
            .into_iter()
            .map(|(k, v)| Benchmark {
                id: k,
                label: v.label,
                arguments: v.args.into_iter().map(std::convert::Into::into).collect(),
                warmup_iterations: v.warmup,
                benchmark_iterations: v.iterations,
            })
            .collect(),
        implementations: config
            .implementations
            .into_iter()
            .map(|(k, v)| Implementation {
                id: k,
                label: v.label,
                directory: v.directory,
            })
            .collect(),
    }
}
