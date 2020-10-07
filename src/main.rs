#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_mega2560::hal::port::mode::{Floating, Input, Output};
use arduino_mega2560::prelude::*;
use atmega2560_hal::port::portb;
use avr_hal_generic::hal::digital::v2::InputPin;
use avr_hal_generic::hal::digital::v2::OutputPin;
use avr_hal_generic::void::Void;
use ufmt;

/*
 * CA1: 51
 * CA2: 53
 * PA0: 43
 * PA1: 41
 * ...
 * PA7: 29
 */

struct SendHandshakePins<'a> {
    incoming_handshake: &'a dyn InputPin<Error=Void>,
    outgoing_handshake: &'a mut dyn OutputPin<Error=Void>,
}

struct SendDataPins<'a> {
    pins: [&'a mut dyn OutputPin<Error=Void>; 8],
}

struct SendPins<'a> {
    handshake_pins: SendHandshakePins<'a>,
    data_pins: SendDataPins<'a>,
}

fn send_byte(send_pins: &mut SendPins, byte: u8) {
    let SendPins {
        handshake_pins: handshake,
        data_pins: data,
    } = send_pins;

    for i in 0..8 {
        let mask = 1 << i;
        if byte & mask == 0 {
            data.pins[i].set_low().void_unwrap();
        } else {
            data.pins[i].set_high().void_unwrap();
        }
    }
    handshake.outgoing_handshake.set_low().void_unwrap();

    while handshake.incoming_handshake.is_high().void_unwrap() {}

    handshake.outgoing_handshake.set_high().void_unwrap();
}

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

    let handshake_pins = SendHandshakePins {
        incoming_handshake: &ca2,
        outgoing_handshake: &mut ca1,
    };

    let data_pins = SendDataPins {
        pins: [
            &mut pa0,
            &mut pa1,
            &mut pa2,
            &mut pa3,
            &mut pa4,
            &mut pa5,
            &mut pa6,
            &mut pa7,
        ],
    };

    let mut send_pins = SendPins {
        handshake_pins,
        data_pins,
    };

    let s = b"Hello world!";

    for data in s {
        ufmt::uwriteln!(&mut serial, "Sending data: {}", data).void_unwrap();
        send_byte(&mut send_pins, *data);
    }

    loop {
        delay.delay_ms(10000u16);
    }
}
