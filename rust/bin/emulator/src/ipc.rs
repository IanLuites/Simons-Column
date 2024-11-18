//! Unix Datagram IPC

use std::{
    os::unix::net::UnixDatagram,
    path::{Path, PathBuf},
    sync::atomic::Ordering,
    time::Duration,
};

use tpic6c596::Pin;

use crate::{Sender, StopSignal};

/// IPC through datagrams.
#[derive(Debug)]
pub struct Ipc {
    /// Datagram socket
    socket: UnixDatagram,

    /// Socket path.
    path: PathBuf,
}

impl Ipc {
    /// Listen for datagram messages
    #[allow(clippy::needless_pass_by_value)]
    pub fn listen(self, stop: StopSignal, sender: &Sender) {
        let mut buffer = vec![0; 1024];

        while !stop.load(Ordering::Relaxed) {
            if let Ok(received) = self.socket.recv(&mut buffer) {
                for message in buffer.iter().take(received) {
                    let pin = match message & 0b0000_1111 {
                        1 => Some(Pin::Data),
                        2 => Some(Pin::Control),
                        3 => Some(Pin::Clock),
                        4 => Some(Pin::Latch),
                        _ => None,
                    };

                    if let Some(pin) = pin {
                        let state = (message & 0b1000_0000) != 0;
                        let _ = sender.send(crate::Message { pin, state });
                    }
                }
            }
        }
    }
}

impl Drop for Ipc {
    fn drop(&mut self) {
        let _ = self.socket.shutdown(std::net::Shutdown::Both);
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Bind to datagram socket.
#[must_use]
pub fn bind(socket: impl AsRef<Path>) -> Ipc {
    let path = socket.as_ref();
    let _ = std::fs::remove_file(path);

    let socket = UnixDatagram::bind(socket.as_ref()).expect("open the datagram socket");

    socket
        .set_read_timeout(Some(Duration::from_millis(500)))
        .expect("set_read_timeout function failed");

    Ipc {
        socket,
        path: path.into(),
    }
}
