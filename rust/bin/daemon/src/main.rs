//! Light daemon

#[cfg(all(feature = "emulator", feature = "rpi"))]
compile_error!("Feature emulator and rpi are mutually exclusive and cannot be enabled together");

/// Connect to a controller.
#[must_use]
fn connect(chain: usize) -> tpic6c596::Controller<impl tpic6c596::Connector> {
    #[cfg(feature = "emulator")]
    {
        println!("Daemon connecting to emulator");
        tpic6c596::Controller::emulator(chain).expect("Emulator connection")
    }
    #[cfg(feature = "rpi")]
    {
        println!("Daemon connecting to RPi GPIO");
        tpic6c596::Controller::rpi_gpio(17, 22, 27, 23, chain).expect("RPi GPIO light controller")
    }
    #[cfg(not(any(feature = "emulator", feature = "rpi")))]
    {
        compile_error!("Requires enabling either emulator or rpi feature");
    }
}

/// Blink the controller `times` with a given duration.
///
/// Optionally pass an off duration.
fn blink(
    controller: &mut tpic6c596::Controller<impl tpic6c596::Connector>,
    times: u8,
    duration: std::time::Duration,
    off: Option<std::time::Duration>,
) {
    if times == 0 {
        return;
    }

    #[allow(clippy::cast_possible_truncation)]
    let all = 2_u64.pow((controller.register_chain() * 8) as u32) - 1;
    let off = off.unwrap_or(duration);
    controller.off();
    controller.write(all);

    controller.on();
    std::thread::sleep(duration);

    for _ in 0..(times - 1) {
        controller.off();
        std::thread::sleep(off);
        controller.on();
    }

    controller.off();
    controller.write(0);
}

/// Connect and setup a controller.
#[must_use]
fn setup(chain: usize) -> tpic6c596::Controller<impl tpic6c596::Connector> {
    let mut controller = connect(chain);

    // Blink all lights 3 times to acknowledge connection / setup.
    blink(
        &mut controller,
        3,
        std::time::Duration::from_millis(400),
        None,
    );

    controller
}

fn main() {
    let mut _controller = setup(3);
}
