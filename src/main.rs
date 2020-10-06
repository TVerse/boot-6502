#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_mega2560::prelude::*;
use ufmt;

/*
 * CA1: 51
 * CA2: 53
 * PA0: 43
 * PA1: 41
 * ...
 * PA7: 29
 */

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();
    let mut delay = arduino_mega2560::Delay::new();
    let mut pins = arduino_mega2560::Pins::new(
        dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH, dp.PORTJ,
        dp.PORTK, dp.PORTL,
    );
    // set up serial interface for text output
    let mut serial = arduino_mega2560::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    let mut ca1 = pins.d51.into_output(&mut pins.ddr);
    let ca2 = pins.d53;
    let mut pa0 = pins.d43.into_output(&mut pins.ddr);
    let mut pa1 = pins.d41.into_output(&mut pins.ddr);
    let mut pa2 = pins.d39.into_output(&mut pins.ddr);
    let mut pa3 = pins.d37.into_output(&mut pins.ddr);
    let mut pa4 = pins.d35.into_output(&mut pins.ddr);
    let mut pa5 = pins.d33.into_output(&mut pins.ddr);
    let mut pa6 = pins.d31.into_output(&mut pins.ddr);
    let mut pa7 = pins.d29.into_output(&mut pins.ddr);

    let s = b"Hello world!";

    let start = micros();

    for data in s {
        ufmt::uwriteln!(&mut serial, "Sending data: {}", data).void_unwrap();

        if data & 0b10000000 != 0 {
            pa7.set_high().void_unwrap();
        } else {
            pa7.set_low().void_unwrap();
        }
        if data & 0b01000000 != 0 {
            pa6.set_high().void_unwrap();
        } else {
            pa6.set_low().void_unwrap();
        }
        if data & 0b00100000 != 0 {
            pa5.set_high().void_unwrap();
        } else {
            pa5.set_low().void_unwrap();
        }
        if data & 0b00010000 != 0 {
            pa4.set_high().void_unwrap();
        } else {
            pa4.set_low().void_unwrap();
        }
        if data & 0b00001000 != 0 {
            pa3.set_high().void_unwrap();
        } else {
            pa3.set_low().void_unwrap();
        }
        if data & 0b00000100 != 0 {
            pa2.set_high().void_unwrap();
        } else {
            pa2.set_low().void_unwrap();
        }
        if data & 0b00000010 != 0 {
            pa1.set_high().void_unwrap();
        } else {
            pa1.set_low().void_unwrap();
        }
        if data & 0b00000001 != 0 {
            pa0.set_high().void_unwrap();
        } else {
            pa0.set_low().void_unwrap();
        }

        ca1.set_low().void_unwrap();

        while ca2.is_high().void_unwrap() {}

        ca1.set_high().void_unwrap();
    }

    let end = micros();

    ufmt::uwriteln!(&mut serial, "Micros taken: {}", end - start).void_unwrap();

    loop {
        delay.delay_ms(10000u16);
    }
}

fn micros() -> u64 {
    0
}
