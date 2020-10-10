#![no_std]

use arduino_mega2560::prelude::*;
use avr_hal_generic::hal::digital::v2::InputPin;
use avr_hal_generic::hal::digital::v2::OutputPin;
use avr_hal_generic::void::{ResultVoidExt, Void};

pub trait Out: OutputPin<Error = Void> {}

impl<T> Out for T where T: OutputPin<Error = Void> {}

pub trait In: InputPin<Error = Void> {}

impl<T> In for T where T: InputPin<Error = Void> {}

pub struct HandshakePins<'a, I, O>
where
    I: In,
    O: Out,
{
    incoming_handshake: &'a I,
    outgoing_handshake: &'a mut O,
}

impl<'a, I, O> HandshakePins<'a, I, O>
where
    I: In,
    O: Out,
{
    pub fn new(incoming_handshake: &'a I, outgoing_handshake: &'a mut O) -> Self {
        Self {
            incoming_handshake,
            outgoing_handshake,
        }
    }

    fn do_handshake(&mut self) -> () {
        let mut delay = arduino_mega2560::Delay::new();

        delay.delay_us(100u8); // TODO

        self.outgoing_handshake.set_low().void_unwrap();
        delay.delay_us(5u8); // TODO race condition somewhere? Too fast for the 6502?

        while self.incoming_handshake.is_high().void_unwrap() {}
        self.outgoing_handshake.set_high().void_unwrap();
        delay.delay_us(5u8); // TODO race condition somewhere? Too fast for the 6502?

        while self.incoming_handshake.is_low().void_unwrap() {}
        delay.delay_us(10u8); // TODO same
    }
}

pub struct SendDataPins<'a, P0, P1, P2, P3, P4, P5, P6, P7>
where
    P0: Out,
    P1: Out,
    P2: Out,
    P3: Out,
    P4: Out,
    P5: Out,
    P6: Out,
    P7: Out,
{
    p0: &'a mut P0,
    p1: &'a mut P1,
    p2: &'a mut P2,
    p3: &'a mut P3,
    p4: &'a mut P4,
    p5: &'a mut P5,
    p6: &'a mut P6,
    p7: &'a mut P7,
}

impl<'a, P0, P1, P2, P3, P4, P5, P6, P7> SendDataPins<'a, P0, P1, P2, P3, P4, P5, P6, P7>
where
    P0: Out,
    P1: Out,
    P2: Out,
    P3: Out,
    P4: Out,
    P5: Out,
    P6: Out,
    P7: Out,
{
    pub fn new(
        p0: &'a mut P0,
        p1: &'a mut P1,
        p2: &'a mut P2,
        p3: &'a mut P3,
        p4: &'a mut P4,
        p5: &'a mut P5,
        p6: &'a mut P6,
        p7: &'a mut P7,
    ) -> Self {
        Self {
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
        }
    }

    fn put_data_on_pins(&mut self, data: u8) {
        // TODO macro?
        if data & 0b00000001 != 0 {
            self.p0.set_high().void_unwrap();
        } else {
            self.p0.set_low().void_unwrap();
        }
        if data & 0b00000010 != 0 {
            self.p1.set_high().void_unwrap();
        } else {
            self.p1.set_low().void_unwrap();
        }
        if data & 0b00000100 != 0 {
            self.p2.set_high().void_unwrap();
        } else {
            self.p2.set_low().void_unwrap();
        }
        if data & 0b00001000 != 0 {
            self.p3.set_high().void_unwrap();
        } else {
            self.p3.set_low().void_unwrap();
        }
        if data & 0b00010000 != 0 {
            self.p4.set_high().void_unwrap();
        } else {
            self.p4.set_low().void_unwrap();
        }
        if data & 0b00100000 != 0 {
            self.p5.set_high().void_unwrap();
        } else {
            self.p5.set_low().void_unwrap();
        }
        if data & 0b01000000 != 0 {
            self.p6.set_high().void_unwrap();
        } else {
            self.p6.set_low().void_unwrap();
        }
        if data & 0b10000000 != 0 {
            self.p7.set_high().void_unwrap();
        } else {
            self.p7.set_low().void_unwrap();
        }
    }
}

// pub struct ReceiveDataPins<'a, P0, P1, P2, P3, P4, P5, P6, P7>
// where
//     P0: In,
//     P1: In,
//     P2: In,
//     P3: In,
//     P4: In,
//     P5: In,
//     P6: In,
//     P7: In,
// {
//     p0: &'a P0,
//     p1: &'a P1,
//     p2: &'a P2,
//     p3: &'a P3,
//     p4: &'a P4,
//     p5: &'a P5,
//     p6: &'a P6,
//     p7: &'a P7,
// }

// TODO IDEA: make this a trait
// Should make it easier to handle creation restrictions (fallible new)
// pub enum Command<'a> {
//     DisplayString { string: &'a str },
// }

pub struct SendData<'a, I, O, P0, P1, P2, P3, P4, P5, P6, P7>
where
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
    handshake_pins: &'a mut HandshakePins<'a, I, O>,
    data_pins: &'a mut SendDataPins<'a, P0, P1, P2, P3, P4, P5, P6, P7>,
    serial: &'a mut arduino_mega2560::Serial<atmega2560_hal::port::mode::Floating>,
}

impl<'a, I, O, P0, P1, P2, P3, P4, P5, P6, P7> SendData<'a, I, O, P0, P1, P2, P3, P4, P5, P6, P7>
where
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
    pub fn new(
        handshake_pins: &'a mut HandshakePins<'a, I, O>,
        data_pins: &'a mut SendDataPins<'a, P0, P1, P2, P3, P4, P5, P6, P7>,
        serial: &'a mut arduino_mega2560::Serial<atmega2560_hal::port::mode::Floating>,
    ) -> Self {
        Self {
            handshake_pins,
            data_pins,
            serial,
        }
    }

    pub fn send(&mut self, string: &str) {
        let s = string.bytes();
        let len = (s.len() - 1) as u8; // TODO make this explicit!

        self.send_byte(len);

        let mut delay = arduino_mega2560::Delay::new();
        delay.delay_us(100u8);

        for data in s {
            self.send_byte(data);
        }
    }

    fn send_byte(&mut self, data: u8) {
        // Writing to serial here slows us down enough for this all to work?
        ufmt::uwriteln!(self.serial, "test").void_unwrap();

        // TODO verify handshake is correct
        // probably first handshake is wrong so second byte is read as first byte

        self.data_pins.put_data_on_pins(data);

        // But here it does not?

        self.handshake_pins.do_handshake()
    }
}
