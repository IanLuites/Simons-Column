//! TOML based config

use std::{collections::HashMap, num::NonZeroUsize};

use serde::Deserialize;

use crate::benchmark::{Arguments, Iterations};

use super::Implementation;

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

/// Basic benchmark config.
#[derive(Debug, Deserialize)]
struct Base {
    /// Label
    label: Option<String>,

    /// Warmup iterations
    warmup: Option<usize>,

    /// Benchmark iterations
    iterations: Option<NonZeroUsize>,

    /// Arguments
    #[serde(default)]
    args: Option<Args>,
}

/// Benchmark options.
#[derive(Debug, Deserialize)]
struct BenchmarkConfig {
    /// Base config for a suite.
    #[serde(flatten)]
    base: Base,

    /// Matrix
    matrix: Option<std::collections::HashMap<String, Base>>,
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
    #[serde(default)]
    benchmarks: HashMap<String, BenchmarkConfig>,

    /// Implementation definitions
    #[serde(default)]
    implementations: HashMap<String, ImplementationConfig>,
}

/// Read TOML config from str.
#[must_use]
pub fn from_str(toml: impl AsRef<str>) -> super::Builder {
    let config: Config = toml::from_str(toml.as_ref()).expect("a");

    let mut benchmarks = Vec::with_capacity(config.benchmarks.len());

    for (key, value) in config.benchmarks {
        let mut builder = crate::benchmark::Suite::build(key);
        if let Some(label) = value.base.label {
            builder = builder.label(label);
        }
        if let Some(warmup) = value.base.warmup {
            builder = builder.warmup(warmup);
        }
        if let Some(iterations) = value.base.iterations {
            builder = builder.iterations(iterations.get());
        }
        if let Some(args) = value.base.args {
            builder = builder.args(args);
        }

        for (id, config) in value.matrix.unwrap_or_default() {
            builder = builder.case(id, |mut case| {
                if let Some(label) = config.label {
                    case = case.label(label);
                }
                if let Some(warmup) = config.warmup {
                    case = case.warmup(warmup);
                }
                if let Some(iterations) = config.iterations {
                    case = case.iterations(iterations.get());
                }
                if let Some(args) = config.args {
                    case = case.args(args);
                }

                case
            });
        }

        benchmarks.push(builder);
    }

    super::Builder {
        strict: config.strict,
        implementation_directory: config.implementation_directory,
        iterations: Iterations::new(config.defaults.warmup, config.defaults.iterations),
        benchmarks,
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
