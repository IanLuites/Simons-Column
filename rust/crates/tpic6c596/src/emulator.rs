//! Emulator for testing

use crate::Pin;

/// Represents a register in the TPIC6C596 emulator.
///
/// The `Register` struct holds the state of a register, including its buffer,
/// current state, and whether it is on or off.
#[derive(Debug, Default, Clone, Copy)]
pub struct Register {
    /// The buffer value of the register.
    buffer: u8,

    /// The current state of the register.
    state: u8,

    /// Indicates whether the register is on or off.
    on: bool,
}

impl Register {
    /// Create a new register with initial value.
    #[must_use]
    const fn new(value: u8) -> Self {
        Self {
            buffer: value,
            state: value,
            on: false,
        }
    }

    /// Shift a bit into the register.
    ///
    /// Returns the overflow bit state.
    #[must_use]
    fn shift(&mut self, bit: bool) -> bool {
        let out = self.buffer & 0b1000_0000 != 0;
        self.buffer <<= 1;

        if bit {
            self.buffer += 1;
        }

        out
    }

    /// Commit buffer to state.
    fn commit(&mut self) {
        self.state = self.buffer;
    }

    /// Return state.
    ///
    /// Returns `0` if register is off.
    #[must_use]
    pub const fn state(self) -> u8 {
        if self.on {
            self.state
        } else {
            0
        }
    }

    /// Turn a register on or off.
    fn set_on(&mut self, on: bool) {
        self.on = on;
    }
}

/// Represents a set of pins in the TPIC6C596 emulator.
///
/// The `PinSet` struct holds the state of the clock, control, data, and latch pins.
#[derive(Debug, Default)]
struct PinSet<T> {
    /// Clock pin value.
    clock: T,

    /// Control pin value.
    control: T,

    /// Data pin value.
    data: T,

    /// Latch pin value.
    latch: T,
}

impl<T> PinSet<T> {
    /// Get pin value.
    #[must_use]
    pub const fn get(&self, pin: Pin) -> &T {
        match pin {
            Pin::Clock => &self.clock,
            Pin::Control => &self.control,
            Pin::Data => &self.data,
            Pin::Latch => &self.latch,
        }
    }

    /// Set pin value.
    pub fn set(&mut self, pin: Pin, value: T) {
        match pin {
            Pin::Clock => self.clock = value,
            Pin::Control => self.control = value,
            Pin::Data => self.data = value,
            Pin::Latch => self.latch = value,
        }
    }
}

/// Represents an emulator for the TPIC6C596 shift registers.
///
/// The `Emulator` struct provides methods to manipulate and test the behavior of the TPIC6C596 shift register.
/// It holds the state of the pins and registers, and allows for setting pin states and retrieving register states.
#[derive(Debug)]
pub struct Emulator {
    /// A set of pins represented as a `PinSet` of boolean values.
    /// Each pin can be either `true` (high) or `false` (low).
    pins: PinSet<bool>,

    /// Chain of registers.
    registers: Vec<Register>,
}

impl Emulator {
    /// Creates a new emulator with a specified number of registers in the chain.
    ///
    /// # Arguments
    ///
    /// * `chain` - The number of registers in the chain.
    ///
    /// # Returns
    ///
    /// A new instance of `Emulator`.
    #[must_use]
    pub fn new(chain: usize) -> Self {
        Self {
            pins: PinSet::default(),
            registers: vec![Register::new(0); chain],
        }
    }

    /// Sets the state of a specified pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to set.
    /// * `state` - The state to set the pin to (`true` for high, `false` for low).
    pub fn set_pin(&mut self, pin: Pin, state: bool) {
        if *self.pins.get(pin) != state {
            self.pins.set(pin, state);

            match (pin, state) {
                (Pin::Latch, false) => {
                    for register in &mut self.registers {
                        register.commit();
                    }
                }
                (Pin::Clock, true) => {
                    let mut over = self.pins.data;

                    for register in &mut self.registers {
                        over = register.shift(over);
                    }
                }
                (Pin::Control, on) => {
                    for register in &mut self.registers {
                        register.set_on(on);
                    }
                }
                (_, _) => {
                    // Nothing to do.
                }
            }
        }
    }

    /// Gets the state of a specified pin.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to set.
    #[must_use]
    pub const fn get_pin(&self, pin: Pin) -> bool {
        *self.pins.get(pin)
    }

    /// Checks if the control pin is on.
    ///
    /// # Returns
    ///
    /// `true` if the control pin is on, `false` otherwise.
    #[must_use]
    pub const fn is_on(&self) -> bool {
        self.pins.control
    }

    /// Retrieves a register at a specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the register to retrieve.
    ///
    /// # Returns
    ///
    /// The register at the specified index.
    #[must_use]
    pub fn register(&self, index: usize) -> Register {
        self.registers[index]
    }

    /// Retrieves the states of all registers.
    ///
    /// # Returns
    ///
    /// A slice of all registers.
    #[must_use]
    pub fn registers(&self) -> &[Register] {
        &self.registers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_write_commit() {
        let mut register = Register::new(0);
        let _ = register.shift(true);

        assert_eq!(register.state(), 0);
        register.commit();
        assert_eq!(register.state(), 0);
        register.set_on(true);
        assert_eq!(register.state(), 1);

        let _ = register.shift(true);
        assert_eq!(register.state(), 1);
        register.commit();
        assert_eq!(register.state(), 3);
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn register_overflow() {
        // Without commit
        for on in [true, false] {
            let mut register = Register::new(0);
            register.set_on(on);

            assert_eq!(register.shift(true), false);
            assert_eq!(register.shift(false), false);
            assert_eq!(register.shift(true), false);
            assert_eq!(register.shift(true), false);
            assert_eq!(register.shift(true), false);
            assert_eq!(register.shift(true), false);
            assert_eq!(register.shift(true), false);
            assert_eq!(register.shift(true), false);

            assert_eq!(register.shift(true), true);
            assert_eq!(register.shift(true), false);
            assert_eq!(register.shift(true), true);
        }

        // With commit
        for on in [true, false] {
            let mut register = Register::new(0);
            register.set_on(on);

            assert_eq!(register.shift(true), false);
            register.commit();
            assert_eq!(register.shift(false), false);
            register.commit();
            assert_eq!(register.shift(true), false);
            register.commit();
            assert_eq!(register.shift(true), false);
            register.commit();
            assert_eq!(register.shift(true), false);
            register.commit();
            assert_eq!(register.shift(true), false);
            register.commit();
            assert_eq!(register.shift(true), false);
            register.commit();
            assert_eq!(register.shift(true), false);
            register.commit();

            assert_eq!(register.shift(true), true);
            register.commit();
            assert_eq!(register.shift(true), false);
            register.commit();
            assert_eq!(register.shift(true), true);
        }
    }

    fn write_bits(emulator: &mut Emulator, mut data: u64, bits: u8) {
        emulator.set_pin(Pin::Latch, false);

        for _ in 0..bits {
            emulator.set_pin(Pin::Clock, false);
            emulator.set_pin(Pin::Data, data & 1 != 0);
            data >>= 1;
            emulator.set_pin(Pin::Clock, true);
        }

        // Commit
        emulator.set_pin(Pin::Latch, true);
        emulator.set_pin(Pin::Latch, false);
    }

    #[test]
    fn emulator() {
        let mut emulator = Emulator::new(3);
        write_bits(&mut emulator, 255, 8);

        assert_eq!(emulator.register(0).state(), 0);

        emulator.set_pin(Pin::Control, true);
        assert_eq!(emulator.register(0).state(), 255);
        assert_eq!(emulator.register(1).state(), 0);
        assert_eq!(emulator.register(2).state(), 0);

        write_bits(&mut emulator, 0, 8);
        assert_eq!(emulator.register(0).state(), 0);
        assert_eq!(emulator.register(1).state(), 255);
        assert_eq!(emulator.register(2).state(), 0);

        write_bits(&mut emulator, 0b0100_1100, 8);
        assert_eq!(emulator.register(0).state(), 0b0011_0010);
        assert_eq!(emulator.register(1).state(), 0);
        assert_eq!(emulator.register(2).state(), 255);
    }
}
