//! Shared logic

static USAGE: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn set_usage(value: impl Into<String>) {
    USAGE.set(value.into()).expect("Not set yet.")
}

pub fn usage(args: &[String]) -> ! {
    eprintln!(
        "Usage: {} <warmup> <benchmark> {}",
        args[0],
        USAGE.get().map(|s| s.as_str()).unwrap_or("")
    );
    std::process::exit(1);
}

pub fn arg<T: std::str::FromStr>(args: &[String], index: usize, error: &str) -> T {
    match args[index].replace('_', "").parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: {error}");
            usage(args);
        }
    }
}

pub fn opt_arg<T: std::str::FromStr>(args: &[String], index: usize, default: T) -> T {
    match args.get(index).map(|v| v.parse()) {
        Some(Ok(n)) => n,
        None | Some(Err(_)) => default,
    }
}

pub fn opt_pin(
    gpio: &rppal::gpio::Gpio,
    args: &[String],
    index: usize,
    default: u8,
) -> rppal::gpio::Pin {
    let pin: u8 = opt_arg(args, index, default);
    let pin_error = format!("pin {pin} access");

    gpio.get(pin).expect(&pin_error)
}
