//! Control TPIC6C596 power logic 8-bit shift register.

mod emulator;
pub use emulator::{Emulator, Register};

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
