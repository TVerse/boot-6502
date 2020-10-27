#![no_std]

use arduino_mega2560::prelude::*;
use avr_hal_generic::void::ResultVoidExt;

pub mod serial;

pub fn done() -> ! {
    let mut delay = arduino_mega2560::Delay::new();
    serial_print!("\n\u{04}");

    loop {
        delay.delay_ms(1000u16);
    }
}
