//! Shared controller
#![allow(clippy::module_name_repetitions)]
#![allow(unsafe_code)]

/// Emulator Controller
#[cfg(feature = "emulator")]
type Controller = tpic6c596::connectors::Emulator;

/// Raspberry Pi GPIO Controller
#[cfg(feature = "rpi")]
type Controller = tpic6c596::connectors::RPi;

/// Connect a controller
fn connect(chain: usize) -> Option<Controller> {
    #[cfg(feature = "emulator")]
    {
        Controller::emulator(chain).ok()
    }
    #[cfg(feature = "rpi")]
    {
        Controller::rpi_gpio(17, 22, 27, 23, chain).ok()
    }
}

/// Create a controller.
#[no_mangle]
pub extern "C" fn controller_connect(chain: usize) -> *mut Controller {
    connect(chain).map_or_else(std::ptr::null_mut, |c| Box::into_raw(Box::new(c)))
}

/// Write bits into a controller.
#[no_mangle]
pub extern "C" fn controller_write(ptr: *mut Controller, data: u64) {
    if !ptr.is_null() {
        unsafe { &mut *ptr }.write(data);
    }
}

/// Write bits into a controller.
#[no_mangle]
pub extern "C" fn controller_loop(ptr: *mut Controller, on: u8, off: u8, loops: u8) {
    if !ptr.is_null() {
        let controller = unsafe { &mut *ptr };

        let all_1 = 2_u64.pow(controller.register_chain() as u32) - 1;
        let all_0 = 0_u64;

        for _ in 0..loops {
            for _ in 0..on {
                controller.write(all_1);
            }
            for _ in 0..off {
                controller.write(all_0);
            }
        }
    }
}

/// Write bits into a controller.
#[no_mangle]
pub extern "C" fn controller_test(ptr: *mut Controller, loops: u64) {
    if !ptr.is_null() {
        let controller = unsafe { &mut *ptr };

        let mut data = [0_u64; 24];
        for x in 0..24 {
            data[x] = !(2_u64.pow(x as u32 + 1) - 1);
            eprintln!("{:b}", data[x]);
        }

        for _ in 0..loops {
            for &x in &data {
                controller.write(x);
            }
        }
    }
}

/// Turn a controller on.
#[no_mangle]
pub extern "C" fn controller_on(ptr: *mut Controller) {
    if !ptr.is_null() {
        unsafe { &mut *ptr }.on();
    }
}

/// Turn a controller off.
#[no_mangle]
pub extern "C" fn controller_off(ptr: *mut Controller) {
    if !ptr.is_null() {
        unsafe { &mut *ptr }.off();
    }
}

/// Free memory (drop) used by a controller.
#[no_mangle]
pub extern "C" fn controller_free(ptr: *mut Controller) {
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) });
    }
}
