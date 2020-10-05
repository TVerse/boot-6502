#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_mega2560::prelude::*;
use arduino_mega2560::{Delay, Pins};

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut delay = arduino_mega2560::Delay::new();
    let pins = arduino_mega2560::Pins::new(
        dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH, dp.PORTJ,
        dp.PORTK, dp.PORTL,
    );

    test_shift(pins, &mut delay);

    loop {}
}

fn test_shift(mut pins: Pins, delay: &mut Delay) -> () {
    let mut clock_pin = pins.d52.into_output(&mut pins.ddr);
    let mut data_pin = pins.d53.into_output(&mut pins.ddr);
    clock_pin.set_high().void_unwrap();

    let mut data: [u8; 15] = b"Hello from Ard!".clone();

    for d in data.iter_mut() {
        for _ in 0..8 {
            clock_pin.set_low().void_unwrap();
            delay.delay_ms(1u8);
            let to_write = *d & 0x80;
            *d = *d << 1;
            if to_write == 0 {
                data_pin.set_low().void_unwrap();
            } else {
                data_pin.set_high().void_unwrap();
            }
            delay.delay_ms(1u8);
            clock_pin.set_high().void_unwrap();
            delay.delay_ms(1u8);
        }
    }

    ()
}
