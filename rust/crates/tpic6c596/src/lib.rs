//! Control TPIC6C596 power logic 8-bit shift register.
//!
//! This crate provides a `Controller` to manage a chain of TPIC6C596 shift registers.
//! The controller can turn the registers on and off, shift bits into the registers,
//! and reset the registers. It uses a `Connector` trait to abstract the connection
//! to the actual hardware pins.
//!
//! # Features
//!
//! - `emulator`: Enables an emulator for testing purposes. When this feature is enabled,
//!   the `Emulator` and `Register` types are available for use.
//! - `delay`: Adds a small delay after latching to ensure the TPIC6C596 properly detects
//!   the latch. This feature is useful for certain hardware configurations that require
//!   a delay to function correctly.
//! - `connector-emulator`: Adds a build in connector for the emulator. Useable
//!    using `Connector::emulator` or `Connector::emulator_on_socket`.
//! - `connector-rpi`: Adds a build in connector for the Raspberry Pi GPIO.
//!    Useable using `Connector::rpi_gpio`.
//!
//! # Example
//!
//! ```rust
//! use tpic6c596::{Controller, Connector, Pin};
//!
//! struct MyConnector;
//!
//! impl Connector for MyConnector {
//!     fn set(&mut self, pin: Pin, state: bool) {
//!         // Set the pin state
//!     }
//!
//!     fn get(&self, pin: Pin) -> bool {
//!         // Get the pin state
//!         false
//!     }
//! }
//!
//! let connector = MyConnector;
//! let mut controller = Controller::connect(connector, 3);
//! controller.on();
//! controller.shift(0b10101010, 8);
//! controller.off();
//! ```
//!
//! # Testing
//!
//! When the `emulator` feature is enabled, the crate includes tests that use the `Emulator`
//! to verify the functionality of the `Controller`. These tests ensure that the controller
//! correctly shifts bits, turns the registers on and off, and resets the registers.

#[cfg(feature = "emulator")]
mod emulator;

#[cfg(feature = "emulator")]
pub use emulator::{Emulator, Register};

#[cfg(any(feature = "connector-emulator", feature = "connector-rpi"))]
mod connectors;

/// Represents the pins of the TPIC6C596 shift register.
#[derive(Debug, Clone, Copy)]
pub enum Pin {
    /// Clock pin.
    Clock,
    /// Control pin.
    Control,
    /// Data pin.
    Data,
    /// Latch pin.
    Latch,
}

#[cfg(any(feature = "connector-emulator", feature = "connector-rpi"))]
/// Pin container.
#[derive(Debug, Default)]
struct Pins<T> {
    /// Clock pin.
    clock: T,
    /// Control pin.
    control: T,
    /// Data pin.
    data: T,
    /// Latch pin.
    latch: T,
}

#[cfg(any(feature = "connector-emulator", feature = "connector-rpi"))]
impl<T> Pins<T> {
    /// Get a ref to a pin value.
    ///
    /// For `Copy` types see `get/1`.
    #[must_use]
    pub const fn get_ref(&self, pin: Pin) -> &T {
        match pin {
            Pin::Clock => &self.clock,
            Pin::Control => &self.control,
            Pin::Data => &self.data,
            Pin::Latch => &self.latch,
        }
    }

    /// Get a mutable ref to a pin value.
    ///
    /// See also: `set/2`.
    #[must_use]
    pub fn get_mut(&mut self, pin: Pin) -> &mut T {
        match pin {
            Pin::Clock => &mut self.clock,
            Pin::Control => &mut self.control,
            Pin::Data => &mut self.data,
            Pin::Latch => &mut self.latch,
        }
    }

    /// Set a pin value.
    pub fn set(&mut self, pin: Pin, value: T) {
        match pin {
            Pin::Clock => self.clock = value,
            Pin::Control => self.control = value,
            Pin::Data => self.data = value,
            Pin::Latch => self.latch = value,
        }
    }
}

#[cfg(any(feature = "connector-emulator", feature = "connector-rpi"))]
impl<T: Copy> Pins<T> {
    /// Get a pin value.
    #[must_use]
    pub const fn get(&self, pin: Pin) -> T {
        match pin {
            Pin::Clock => self.clock,
            Pin::Control => self.control,
            Pin::Data => self.data,
            Pin::Latch => self.latch,
        }
    }
}

/// Connector between the controller and IO pins.
pub trait Connector {
    /// Set a pin's state.
    fn set(&mut self, pin: Pin, state: bool);

    /// Get a pin's state.
    #[must_use]
    fn get(&self, pin: Pin) -> bool;
}

/// A controller to manage a TPIC6C596 register chain.
#[derive(Debug)]
pub struct Controller<C: Connector> {
    /// Connector to connect the controller to the TPIC6C596 pins.
    connector: C,

    /// TPIC6C596 register chain length.
    chain: usize,

    /// Bits to write to chain.
    bits: usize,

    // Local State
    /// On/off state of the TPIC6C596 registers.
    on: bool,
}

impl<C: Connector> Controller<C> {
    /// Controller connector.
    #[must_use]
    pub const fn connector(&self) -> &C {
        &self.connector
    }

    /// Connect controller to TPIC6C596 shift registers.
    #[must_use]
    pub fn connect(mut connector: C, chain: usize) -> Self {
        connector.set(Pin::Latch, false);

        Self {
            on: connector.get(Pin::Control),
            connector,
            bits: chain * 8,
            chain,
        }
    }

    /// The length of the register chain.
    #[must_use]
    pub const fn register_chain(&self) -> usize {
        self.chain
    }

    /// Turn shift registers on.
    pub fn on(&mut self) {
        if !self.on {
            self.connector.set(Pin::Control, true);
            self.on = true;
        }
    }

    /// Turn shift registers off.
    pub fn off(&mut self) {
        if self.on {
            self.connector.set(Pin::Control, false);
            self.on = false;
        }
    }

    /// Shift bits into TPIC6C596 shift registers.
    pub fn shift(&mut self, data: u64, len: usize) {
        shift(&mut self.connector, data, len);
    }

    /// Shift a single high (1) bit into TPIC6C596 shift registers.
    pub fn shift_high(&mut self) {
        shift(&mut self.connector, 1, 1);
    }

    /// Shift a single low (0) bit into TPIC6C596 shift registers.
    pub fn shift_low(&mut self) {
        shift(&mut self.connector, 0, 1);
    }

    /// Write bits into TPIC6C596 shift registers.
    ///
    /// This always shifts the exact number of bits to match the register count.
    pub fn write(&mut self, data: u64) {
        shift(&mut self.connector, data, self.bits);
    }

    /// Reset shift registers to 0.
    ///
    /// Same as `write(0)`.
    pub fn reset(&mut self) {
        self.write(0);
    }
}

#[cfg(feature = "delay")]
/// Latch delay to make the TPIC6C596 properly detect the latch.
const LATCH_DELAY: std::time::Duration = std::time::Duration::from_nanos(1);

/// Write bits to the shift register.
fn shift<C: Connector>(connector: &mut C, mut data: u64, len: usize) {
    for _ in 0..len {
        connector.set(Pin::Clock, false);
        connector.set(Pin::Data, data & 0b1 == 1);
        connector.set(Pin::Clock, true);

        data >>= 1;
    }

    connector.set(Pin::Latch, true);
    #[cfg(feature = "delay")]
    std::thread::sleep(LATCH_DELAY);
    connector.set(Pin::Latch, false);
}

#[cfg(all(test, feature = "emulator"))]
mod tests {
    use super::*;

    /// Create a controller attach to an emulator for testing.
    fn emulator_controller() -> Controller<Emulator> {
        const CHAIN: usize = 3;
        let emulator = Emulator::new(CHAIN);
        let mut controller = Controller::connect(emulator, CHAIN);
        controller.on();

        controller
    }

    #[test]
    fn shift() {
        // Mirrors the `emulator` test in `./emulators.rs`.
        let mut controller = emulator_controller();
        controller.off();

        controller.shift(255, 8);
        assert_eq!(controller.connector().register(0).state(), 0);

        controller.on();
        assert_eq!(controller.connector().register(0).state(), 255);
        assert_eq!(controller.connector().register(1).state(), 0);
        assert_eq!(controller.connector().register(2).state(), 0);

        controller.shift(0, 8);
        assert_eq!(controller.connector().register(0).state(), 0);
        assert_eq!(controller.connector().register(1).state(), 255);
        assert_eq!(controller.connector().register(2).state(), 0);

        controller.shift(0b0100_1100, 8);
        assert_eq!(controller.connector().register(0).state(), 0b0011_0010);
        assert_eq!(controller.connector().register(1).state(), 0);
        assert_eq!(controller.connector().register(2).state(), 255);
    }

    #[test]
    fn shift_high_and_low() {
        let mut controller = emulator_controller();

        assert_eq!(controller.connector().register(0).state(), 0b0000_0000);

        controller.shift_low();
        assert_eq!(controller.connector().register(0).state(), 0b0000_0000);

        controller.shift_high();
        assert_eq!(controller.connector().register(0).state(), 0b0000_0001);

        controller.shift_low();
        assert_eq!(controller.connector().register(0).state(), 0b0000_0010);

        controller.shift_high();
        assert_eq!(controller.connector().register(0).state(), 0b0000_0101);

        for _ in 0..4 {
            controller.shift_high();
        }
        assert_eq!(controller.connector().register(0).state(), 0b0101_1111);

        for _ in 0..4 {
            controller.shift_low();
        }
        assert_eq!(controller.connector().register(0).state(), 0b1111_0000);
        assert_eq!(controller.connector().register(1).state(), 0b0000_0101);
    }

    #[test]
    fn reset() {
        let mut controller = emulator_controller();
        controller.write(0b0000_1111_1010_0000_0000_0101);

        assert_eq!(controller.connector().register(0).state(), 0b1111_0000);
        assert_eq!(controller.connector().register(1).state(), 0b0000_0101);
        assert_eq!(controller.connector().register(2).state(), 0b1010_0000);

        controller.reset();
        assert_eq!(controller.connector().register(0).state(), 0);
        assert_eq!(controller.connector().register(1).state(), 0);
        assert_eq!(controller.connector().register(2).state(), 0);
    }
}
