//! Run benchmarks and collect results.

mod benchmark;
mod config;

fn main() {
    let config = config::Config::parse();
    let benchmarks = config.benchmarks();

    if benchmarks.is_empty() {
        eprintln!("No benchmarks found.");
    } else {
        println!("Benchmarks:\n");
        for benchmark in benchmarks {
            println!(" - {}", benchmark.label());
        }
    }
}
