//! Run benchmarks and collect results.

mod benchmark;
mod config;
mod implementation;
mod util;

use implementation::Implementation;

fn main() {
    let config = config::Config::parse();
    let benchmarks = config.benchmarks();
    let implementations = config.implementations();

    if benchmarks.is_empty() {
        eprintln!("No benchmarks found.");
    } else {
        println!("Benchmarks:\n");
        for benchmark in benchmarks {
            println!(" - {}", benchmark.label());
        }
    }

    println!();

    if implementations.is_empty() {
        eprintln!("No implementations found.");
    } else {
        println!("Implementations:\n");
        for implementation in implementations {
            println!(
                " - {} ({})",
                implementation.label(),
                benchmarks
                    .iter()
                    .filter(|b| implementation.implements(b))
                    .map(benchmark::Suite::id)
                    .collect::<Vec<&str>>()
                    .join(", ")
            );
        }
    }
}
