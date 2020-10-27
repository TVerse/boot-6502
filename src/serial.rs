use arduino_mega2560::Serial;
use atmega2560_hal::port::mode::Floating;

pub static mut SERIAL: Option<Serial<Floating>> = None;

/// Puts the Serial in the static mut
///
/// # Safety
///
/// Does not do any synchronization.
/// `serial_print` and `serial_println` will be noops before this is called.
pub unsafe fn init(serial: Serial<Floating>) {
    SERIAL = Some(serial);
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        match unsafe { &mut $crate::serial::SERIAL } {
            Some(serial) => ufmt::uwrite!(serial, $($arg)*).void_unwrap(),
            None => (),
        };
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    ($($arg:tt)*) => {
        match unsafe { &mut $crate::serial::SERIAL } {
            Some(serial) => ufmt::uwriteln!(serial, $($arg)*).void_unwrap(),
            None => (),
        };
    };
}
