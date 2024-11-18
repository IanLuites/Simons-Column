//! Light column emulator

use std::sync::{atomic::AtomicBool, Arc};

use clap::Parser;
use tpic6c596::{Emulator, Pin};

/// Message sender.
type Sender = std::sync::mpsc::Sender<Message>;

/// Stop signal
type StopSignal = Arc<AtomicBool>;

/// Control message
#[derive(Debug, Clone, Copy)]
pub struct Message {
    /// TPIC6C596 pin
    pub pin: Pin,

    /// Pin state on/off.
    pub state: bool,
}

#[cfg(unix)]
mod ipc;

/// Emulator config
#[derive(Debug, Parser)]
struct Config {
    #[cfg(unix)]
    /// Unix Datagram Socket
    #[arg(short, long, default_value = "/tmp/tpic6c596-emulator.sock")]
    socket: std::path::PathBuf,

    /// Chain length
    #[arg(short, long, default_value_t = 3)]
    chain: usize,
}

/// Print emulator state
fn print(emulator: &Emulator) {
    use std::io::Write;

    print!("\r  State:  ");
    for register in emulator.registers() {
        print!(" {:08b}", register.state());
    }
    std::io::stdout().flush().expect("To flush");
}

/// Start the emulator.
fn start_emulator(mut emulator: Emulator, stop: StopSignal) -> Sender {
    let (sender, receiver) = std::sync::mpsc::channel::<Message>();
    print(&emulator);

    std::thread::spawn(move || {
        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            if let Ok(message) = receiver.recv() {
                emulator.set_pin(message.pin, message.state);
                print(&emulator);
            }
        }
    });

    sender
}

/// Add exit hook
fn exit_hook(stop: StopSignal) {
    ctrlc::set_handler(move || {
        stop.store(true, std::sync::atomic::Ordering::SeqCst);
        println!("\n\nShutting down emulator...");
    })
    .expect("Error setting Ctrl-C handler");
}

fn main() {
    let stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let config: Config = Config::parse();
    let emulator = Emulator::new(config.chain);
    exit_hook(stop.clone());

    println!(
        "Starting TPIC6C596 shift register emulator\n\n  Socket:  {:#?}\n  Chain:   {}",
        config.socket, config.chain
    );

    let sender = start_emulator(emulator, stop.clone());

    #[cfg(unix)]
    {
        let ipc = ipc::bind(&config.socket);
        ipc.listen(stop, &sender);
    }
}
