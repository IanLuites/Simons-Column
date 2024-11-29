use std::time::Instant;

mod shared;
use shared::{arg, controller, opt_arg, set_usage, usage};

fn main() {
    set_usage("[data] [data_pin] [clock_pin] [latch_pin]");
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        usage(&args)
    }

    let warmup: usize = arg(&args, 1, "warmup must be a valid number");
    let benchmark: usize = arg(&args, 2, "benchmark must be a valid number");
    let data: u64 = opt_arg(&args, 3, 202);

    // Setup
    let mut connector = controller(&args, 1);

    // Warmup
    let warmup = {
        let start = Instant::now();
        for _ in 0..warmup {
            connector.write(data);
        }
        start.elapsed().as_nanos()
    };

    // Benchmark
    let benchmark = {
        let start = Instant::now();
        for _ in 0..benchmark {
            connector.write(data);
        }
        start.elapsed().as_nanos()
    };

    println!("warmup: {warmup}");
    println!("benchmark: {benchmark}");
}
