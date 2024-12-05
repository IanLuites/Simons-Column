//! TPIC6C596 connectors

#[cfg(feature = "connector-emulator")]
mod emulator;

/// Controller connected to an emulator.
#[cfg(feature = "connector-emulator")]
pub type Emulator = crate::Controller<emulator::Emulator>;

#[cfg(feature = "connector-rpi")]
mod rpi;

/// Controller connected to the GPIO of a Raspberry Pi.
#[cfg(feature = "connector-rpi")]
pub type RPi = crate::Controller<rpi::RPi>;
