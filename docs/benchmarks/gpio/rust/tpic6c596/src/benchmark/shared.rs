//! Shared logic

use tpic6c596::{Connector, Controller, Pin};

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

pub fn controller(args: &[String], chain: usize) -> Controller<GpioConnector> {
    let gpio = rppal::gpio::Gpio::new().expect("GPIO access");

    Controller::connect(
        GpioConnector {
            data: opt_pin(&gpio, args, 4, 17).into_output_low(),
            clock: opt_pin(&gpio, args, 5, 22).into_output_low(),
            latch: opt_pin(&gpio, args, 6, 27).into_output_low(),
            control: opt_pin(&gpio, args, 7, 12).into_output_low(),
        },
        chain,
    )
}

pub struct GpioConnector {
    data: rppal::gpio::OutputPin,
    clock: rppal::gpio::OutputPin,
    latch: rppal::gpio::OutputPin,
    control: rppal::gpio::OutputPin,
}

impl Connector for GpioConnector {
    fn get(&self, pin: Pin) -> bool {
        match pin {
            Pin::Clock => self.clock.is_set_high(),
            Pin::Control => self.control.is_set_high(),
            Pin::Data => self.data.is_set_high(),
            Pin::Latch => self.latch.is_set_high(),
        }
    }

    fn set(&mut self, pin: Pin, state: bool) {
        let pin = match pin {
            Pin::Clock => &mut self.clock,
            Pin::Control => &mut self.control,
            Pin::Data => &mut self.data,
            Pin::Latch => &mut self.latch,
        };

        if state {
            pin.set_high()
        } else {
            pin.set_low();
        }
    }
}
