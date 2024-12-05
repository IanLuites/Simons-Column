//! Raspberry Pi GPIO connector using `rppal` crate.

use rppal::gpio::{Gpio, OutputPin};

use crate::{Connector, Pin, Pins};

/// Raspberry Pi GPIO connector using `rppal` crate.
#[derive(Debug)]
pub struct RPi(Pins<OutputPin>);

impl Connector for RPi {
    fn get(&self, pin: Pin) -> bool {
        self.0.get_ref(pin).is_set_high()
    }

    fn set(&mut self, pin: Pin, state: bool) {
        if state {
            self.0.get_mut(pin).set_high();
        } else {
            self.0.get_mut(pin).set_low();
        }
    }
}

impl crate::Controller<RPi> {
    /// Connect to a TPIC6C596 chain using Raspberry Pi GPIO.
    ///
    /// # Errors
    ///
    /// Errors when access to the Raspberry Pi's GPIO peripheral or pins fails.
    pub fn rpi_gpio(
        data_pin: u8,
        clock_pin: u8,
        latch_pin: u8,
        control_pin: u8,
        chain: usize,
    ) -> Result<Self, rppal::gpio::Error> {
        let gpio = Gpio::new()?;

        let connector = RPi(Pins {
            data: gpio.get(data_pin)?.into_output_low(),
            clock: gpio.get(clock_pin)?.into_output_low(),
            latch: gpio.get(latch_pin)?.into_output_low(),
            control: gpio.get(control_pin)?.into_output_low(),
        });

        eprintln!("RPi connect:");
        eprintln!("  Data:    {}", connector.0.data.pin());
        eprintln!("  Clock:   {}", connector.0.clock.pin());
        eprintln!("  Latch:   {}", connector.0.latch.pin());
        eprintln!("  Control: {}", connector.0.control.pin());

        Ok(Self::connect(connector, chain))
    }
}
