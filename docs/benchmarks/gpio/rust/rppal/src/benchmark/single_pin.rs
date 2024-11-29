use std::time::Instant;

use rppal::gpio::Gpio;

mod shared;
use shared::{arg, opt_pin, set_usage, usage};

fn main() {
    set_usage("[pin]");
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        usage(&args)
    }

    let warmup: usize = arg(&args, 1, "warmup must be a valid number");
    let benchmark: usize = arg(&args, 2, "benchmark must be a valid number");

    // Setup
    let gpio = Gpio::new().expect("GPIO access");
    let mut pin = opt_pin(&gpio, &args, 3, 17).into_output_low();

    // Warmup
    let warmup = {
        let start = Instant::now();
        for _ in 0..warmup {
            pin.set_high();
            pin.set_low();
        }
        start.elapsed().as_nanos()
    };

    // Benchmark
    let benchmark = {
        let start = Instant::now();
        for _ in 0..benchmark {
            pin.set_high();
            pin.set_low();
        }
        start.elapsed().as_nanos()
    };

    println!("warmup: {warmup}");
    println!("benchmark: {benchmark}");
}
