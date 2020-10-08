#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

use arduino_mega2560::prelude::*;
use atmega2560_hal::port;
use avr_hal_generic::hal::digital::v2::InputPin;
use avr_hal_generic::hal::digital::v2::OutputPin;
use avr_hal_generic::void::{ResultVoidExt, Void};

static mut PANIC_LED: MaybeUninit<port::portb::PB1<port::mode::Output>> = MaybeUninit::uninit();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };
    let mut delay = arduino_mega2560::Delay::new();
    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500u16);
    }
}

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();
    let mut delay = arduino_mega2560::Delay::new();
    let pins = arduino_mega2560::Pins::new(
        dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH, dp.PORTJ,
        dp.PORTK, dp.PORTL,
    );

    unsafe {
        PANIC_LED = MaybeUninit::new(pins.d52.into_output(&pins.ddr));
    };

    let mut serial =
        arduino_mega2560::Serial::new(dp.USART0, pins.d0, pins.d1.into_output(&pins.ddr), 57600);

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

    let mut handshake_pins = HandshakePins {
        incoming_handshake: &ca2,
        outgoing_handshake: &mut ca1,
    };

    let mut send_data_pins = SendDataPins {
        p0: &mut pa0,
        p1: &mut pa1,
        p2: &mut pa2,
        p3: &mut pa3,
        p4: &mut pa4,
        p5: &mut pa5,
        p6: &mut pa6,
        p7: &mut pa7,
    };

    let s = b"Hello world!";

    for data in s {
        ufmt::uwriteln!(&mut serial, "Sending data: {}", data).void_unwrap();
        send_byte(&mut handshake_pins, &mut send_data_pins, *data);
    }

    loop {
        delay.delay_ms(10000u16);
    }
}

struct HandshakePins<'a, I, O> {
    incoming_handshake: &'a I,
    outgoing_handshake: &'a mut O,
}

struct SendDataPins<'a, P0, P1, P2, P3, P4, P5, P6, P7> {
    p0: &'a mut P0,
    p1: &'a mut P1,
    p2: &'a mut P2,
    p3: &'a mut P3,
    p4: &'a mut P4,
    p5: &'a mut P5,
    p6: &'a mut P6,
    p7: &'a mut P7,
}

struct ReceiveDataPins<'a, P0, P1, P2, P3, P4, P5, P6, P7> {
    p0: &'a P0,
    p1: &'a P1,
    p2: &'a P2,
    p3: &'a P3,
    p4: &'a P4,
    p5: &'a P5,
    p6: &'a P6,
    p7: &'a P7,
}

fn send_byte<I, O, P0, P1, P2, P3, P4, P5, P6, P7>(
    handshake_pins: &mut HandshakePins<I, O>,
    data_pins: &mut SendDataPins<P0, P1, P2, P3, P4, P5, P6, P7>,
    data: u8,
) where
    I: In,
    O: Out,
    P0: Out,
    P1: Out,
    P2: Out,
    P3: Out,
    P4: Out,
    P5: Out,
    P6: Out,
    P7: Out,
{
    let mut _delay = arduino_mega2560::Delay::new();

    // TODO macro?
    if data & 0b00000001 == 0 {
        data_pins.p0.set_high().void_unwrap();
    } else {
        data_pins.p0.set_low().void_unwrap();
    }
    if data & 0b00000010 == 0 {
        data_pins.p1.set_high().void_unwrap();
    } else {
        data_pins.p1.set_low().void_unwrap();
    }
    if data & 0b00000100 == 0 {
        data_pins.p2.set_high().void_unwrap();
    } else {
        data_pins.p2.set_low().void_unwrap();
    }
    if data & 0b00001000 == 0 {
        data_pins.p3.set_high().void_unwrap();
    } else {
        data_pins.p3.set_low().void_unwrap();
    }
    if data & 0b00010000 == 0 {
        data_pins.p4.set_high().void_unwrap();
    } else {
        data_pins.p4.set_low().void_unwrap();
    }
    if data & 0b00100000 == 0 {
        data_pins.p5.set_high().void_unwrap();
    } else {
        data_pins.p5.set_low().void_unwrap();
    }
    if data & 0b01000000 == 0 {
        data_pins.p6.set_high().void_unwrap();
    } else {
        data_pins.p6.set_low().void_unwrap();
    }
    if data & 0b10000000 == 0 {
        data_pins.p7.set_high().void_unwrap();
    } else {
        data_pins.p7.set_low().void_unwrap();
    }

    handshake_pins.outgoing_handshake.set_low().void_unwrap();

    while handshake_pins.incoming_handshake.is_high().void_unwrap() {}

    handshake_pins.outgoing_handshake.set_high().void_unwrap();
}

trait Out: OutputPin<Error = Void> {}

impl<T> Out for T where T: OutputPin<Error = Void> {}

trait In: InputPin<Error = Void> {}

impl<T> In for T where T: InputPin<Error = Void> {}
