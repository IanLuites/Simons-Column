use std::time::Instant;

mod shared;
use shared::{arg, controller, opt_arg, set_usage, usage};

fn main() {
    set_usage("[bit] [data_pin] [clock_pin] [latch_pin]");
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        usage(&args)
    }

    let warmup: usize = arg(&args, 1, "warmup must be a valid number");
    let benchmark: usize = arg(&args, 2, "benchmark must be a valid number");
    let bit: bool = opt_arg(&args, 3, 1) > 0;

    // Setup
    let mut connector = controller(&args, 1);

    // Warmup
    let warmup = {
        if bit {
            let start = Instant::now();
            for _ in 0..warmup {
                connector.shift_high();
            }
            start.elapsed().as_nanos()
        } else {
            let start = Instant::now();
            for _ in 0..warmup {
                connector.shift_low();
            }
            start.elapsed().as_nanos()
        }
    };

    // Benchmark
    let benchmark = {
        if bit {
            let start = Instant::now();
            for _ in 0..benchmark {
                connector.shift_high();
            }
            start.elapsed().as_nanos()
        } else {
            let start = Instant::now();
            for _ in 0..benchmark {
                connector.shift_low();
            }
            start.elapsed().as_nanos()
        }
    };

    println!("warmup: {warmup}");
    println!("benchmark: {benchmark}");
}
