//! Cli config

use std::num::NonZeroUsize;

use clap::Parser;

/// Cli parser
#[derive(Debug, Parser)]
pub struct Cli {
    /// Strict mode, panics on config errors.
    #[clap(short, long, default_value_t = false)]
    pub strict: bool,

    /// Implementation directory.
    ///
    /// Uses current working directory by default.
    #[clap(long)]
    pub implementation_directory: Option<std::path::PathBuf>,

    /// TOML config file.
    #[clap(short, long)]
    pub config_file: Vec<std::path::PathBuf>,

    /// Warmup iterations.
    #[clap(short, long)]
    pub warmup_iterations: Option<usize>,

    /// Benchmark iterations.
    #[clap(short('i'), long)]
    pub benchmark_iterations: Option<NonZeroUsize>,
}

/// Parse cli arguments.
#[must_use]
pub fn parse() -> Cli {
    Cli::parse()
}
