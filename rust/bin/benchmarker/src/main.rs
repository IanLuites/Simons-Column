//! Run benchmarks and collect results.

mod benchmark;
mod config;
mod implementation;
mod runner;
mod util;

use implementation::Implementation;

fn main() {
    let config = config::Config::parse();
    let runner = runner::Runner::new(config);
    let results = runner.start().unwrap();

    std::fs::write("results.json", results.to_json()).unwrap();
}
