#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
// extern crate panic_halt;

use arduino_mega2560::prelude::*;
use avr_hal_generic::hal::digital::v2::InputPin;
use avr_hal_generic::hal::digital::v2::OutputPin;
use avr_hal_generic::void::Void;

/*
 * CA1: 51
 * CA2: 53
 * PA0: 43
 * PA1: 41
 * ...
 * PA7: 29
 */

struct SendHandshakePins<'a> {
    incoming_handshake: &'a dyn InputPin<Error = Void>,
    outgoing_handshake: &'a mut dyn OutputPin<Error = Void>,
}

struct SendDataPins<'a> {
    pins: [&'a mut dyn OutputPin<Error = Void>; 8],
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

    let mut _delay = arduino_mega2560::Delay::new();

    for i in 0..8 {
        let mask = 1 << i;
        //if byte & mask == 0 {
        data.pins[i].set_high().void_unwrap();
        //} else {
        //data.pins[i].set_low().void_unwrap();
        //}
    }

    handshake.outgoing_handshake.set_low().void_unwrap();

    while handshake.incoming_handshake.is_high().void_unwrap() {}

    handshake.outgoing_handshake.set_high().void_unwrap();
}

use atmega2560_hal::port;
use core::mem::MaybeUninit;

static mut PANIC_LED: MaybeUninit<port::portb::PB1<port::mode::Output>> = MaybeUninit::uninit();

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();
    let mut delay = arduino_mega2560::Delay::new();
    let pins = arduino_mega2560::Pins::new(
        dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH, dp.PORTJ,
        dp.PORTK, dp.PORTL,
    );
    // set up serial interface for text output
    let mut serial =
        arduino_mega2560::Serial::new(dp.USART0, pins.d0, pins.d1.into_output(&pins.ddr), 57600);

    let panic = pins.d52.into_output(&pins.ddr);

    unsafe {
        PANIC_LED = MaybeUninit::new(panic);
    };

    let mut ca1 = pins.d51.into_output(&pins.ddr);
    let ca2 = pins.d53;
    let mut pa0 = pins.d43.into_output(&pins.ddr);
    let mut pa1 = pins.d41.into_output(&pins.ddr);
    let mut pa2 = pins.d39.into_output(&pins.ddr);
    let mut pa3 = pins.d37.into_output(&pins.ddr);
    let mut pa4 = pins.d35.into_output(&pins.ddr);
    let mut pa5 = pins.d33.into_output(&pins.ddr);
    let mut pa6 = pins.d31.into_output(&pins.ddr);
    let mut pa7 = pins.d29.into_output(&pins.ddr);

    ca1.set_high().void_unwrap();

    let mut send_pins = {
        let handshake_pins = SendHandshakePins {
            incoming_handshake: &ca2,
            outgoing_handshake: &mut ca1,
        };

        let data_pins = SendDataPins {
            pins: [
                &mut pa0, &mut pa1, &mut pa2, &mut pa3, &mut pa4, &mut pa5, &mut pa6, &mut pa7,
            ],
        };

        SendPins {
            handshake_pins,
            data_pins,
        }
    };

    ufmt::uwriteln!(&mut serial, "??").void_unwrap();

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    for p in send_pins.data_pins.pins.iter_mut() {
        p.set_high().void_unwrap();
    }

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    ufmt::uwriteln!(&mut serial, "???").void_unwrap();

    panic!("test");
    loop {}

    let s = b"Hello world!";

    for data in s {
        ufmt::uwriteln!(&mut serial, "Sending data: {}", data).void_unwrap();
        //        send_byte(&mut send_pins, *data);
    }

    loop {
        delay.delay_ms(10000u16);
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };
    let mut delay = arduino_mega2560::Delay::new();
    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500u16);
    }
}
