//! Emulator connector.

use std::os::unix::net::{SocketAddr, UnixDatagram};

use crate::{Connector, Pin, Pins};

///  Emulator connector.
#[derive(Debug)]
struct Emulator {
    /// Socket
    socket: UnixDatagram,

    /// Emulator socket address.
    address: SocketAddr,

    /// Local pin state.
    state: Pins<bool>,
}

impl Connector for Emulator {
    fn get(&self, pin: Pin) -> bool {
        self.state.get(pin)
    }

    fn set(&mut self, pin: Pin, state: bool) {
        self.state.set(pin, state);

        let state = if state { 0b1000_0000_u8 } else { 0 };
        let byte = match pin {
            Pin::Data => state | 1,
            Pin::Control => state | 2,
            Pin::Clock => state | 3,
            Pin::Latch => state | 4,
        };

        let _ = self.socket.send_to_addr(&[byte], &self.address);
    }
}

impl crate::Controller<Emulator> {
    /// Connect to a TPIC6C596 chain emulator.
    ///
    /// # Errors
    ///
    /// Errors on invalid socket address or failure to creates a Unix Datagram socket.
    pub fn emulator(chain: usize) -> std::io::Result<Self> {
        Self::emulator_on_socket("/tmp/tpic6c596-emulator.sock", chain)
    }

    /// Connect to a TPIC6C596 chain emulator on a specific socket.
    ///
    /// # Errors
    ///
    /// Errors on invalid socket address or failure to creates a Unix Datagram socket.
    pub fn emulator_on_socket(
        socket: impl AsRef<std::path::Path>,
        chain: usize,
    ) -> std::io::Result<Self> {
        Ok(Self::connect(
            Emulator {
                socket: UnixDatagram::unbound()?,
                address: SocketAddr::from_pathname(socket)?,
                state: Pins::default(),
            },
            chain,
        ))
    }
}
