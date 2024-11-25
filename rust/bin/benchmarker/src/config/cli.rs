//! Cli config

use std::num::NonZeroUsize;

use clap::Parser;

/// Cli parser
#[derive(Debug, Parser)]
pub struct Cli {
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
