use arduino_mega2560::Serial;
use atmega2560_hal::port::mode::Floating;

pub static mut SERIAL: Option<Serial<Floating>> = None;

/// Puts the Serial in the static mut
///
/// # SAFETY
///
/// Does not do any synchronization.
/// Must be called before `serial_print` or `serial_println`.
pub unsafe fn init(serial: Serial<Floating>) {
    SERIAL = Some(serial);
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        match unsafe { &mut $crate::serial::SERIAL } {
            Some(serial) => ufmt::uwrite!(serial, $($arg)*).void_unwrap(),
            None => panic!(),
        };
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    ($($arg:tt)*) => {
        match unsafe { &mut $crate::serial::SERIAL } {
            Some(serial) => ufmt::uwriteln!(serial, $($arg)*).void_unwrap(),
            None => panic!(),
        };
    };
}
