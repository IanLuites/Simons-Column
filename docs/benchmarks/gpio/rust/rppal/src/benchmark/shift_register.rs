use std::time::Instant;

use rppal::gpio::{self, Gpio};

mod shared;
use shared::{arg, opt_arg, opt_pin, set_usage, usage};

fn main() {
    set_usage("[data] [data_pin] [clock_pin] [latch_pin]");
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        usage(&args)
    }

    let warmup: usize = arg(&args, 1, "warmup must be a valid number");
    let benchmark: usize = arg(&args, 2, "benchmark must be a valid number");
    let data: u8 = opt_arg(&args, 3, 202);

    // Setup
    let gpio = Gpio::new().expect("GPIO access");
    let mut data_pin = opt_pin(&gpio, &args, 4, 17).into_output_low();
    let mut clock_pin = opt_pin(&gpio, &args, 5, 22).into_output_low();
    let mut latch_pin = opt_pin(&gpio, &args, 6, 27).into_output_low();

    // Warmup
    let warmup = {
        let start = Instant::now();
        for _ in 0..warmup {
            shift_byte(&mut data_pin, &mut clock_pin, &mut latch_pin, data);
        }
        start.elapsed().as_nanos()
    };

    // Benchmark
    let benchmark = {
        let start = Instant::now();
        for _ in 0..benchmark {
            shift_byte(&mut data_pin, &mut clock_pin, &mut latch_pin, data);
        }
        start.elapsed().as_nanos()
    };

    println!("warmup: {warmup}");
    println!("benchmark: {benchmark}");
}

fn shift_byte(
    data_pin: &mut gpio::OutputPin,
    clock_pin: &mut gpio::OutputPin,
    latch_pin: &mut gpio::OutputPin,
    mut byte: u8,
) {
    clock_pin.set_low();

    for _ in 0..7 {
        if byte & 1 == 1 {
            data_pin.set_high()
        } else {
            data_pin.set_low()
        }

        byte >>= 1;
    }

    clock_pin.set_high();

    latch_pin.set_high();
    latch_pin.set_low();
}
