#![no_std]

use arduino_mega2560::{DDR, Delay, Serial};
use arduino_mega2560::prelude::*;
use atmega2560_hal::port;
use atmega2560_hal::port::mode::{Floating, Input, Output};
use avr_hal_generic::void::ResultVoidExt;

type IncomingHandshake = port::portb::PB0<Input<Floating>>;
type OutgoingHandshake = port::portb::PB2<Output>;

type P0<A> = port::portl::PL6<A>;
type P1<A> = port::portg::PG0<A>;
type P2<A> = port::portg::PG2<A>;
type P3<A> = port::portc::PC0<A>;
type P4<A> = port::portc::PC2<A>;
type P5<A> = port::portc::PC4<A>;
type P6<A> = port::portc::PC6<A>;
type P7<A> = port::porta::PA7<A>;

pub struct Pins<'a> {
    handshake_pins: HandshakePins,
    data_pins: OutputDataPins<'a>,
    delay: Delay,
    serial: &'a mut Serial<Floating>,
}

impl<'a> Pins<'a> {
    pub fn new(
        ddr: &'a DDR,
        serial: &'a mut Serial<Floating>,
        incoming_handshake: IncomingHandshake,
        outgoing_handshake: OutgoingHandshake,
        p0: P0<Output>,
        p1: P1<Output>,
        p2: P2<Output>,
        p3: P3<Output>,
        p4: P4<Output>,
        p5: P5<Output>,
        p6: P6<Output>,
        p7: P7<Output>,
    ) -> Self {
        Self {
            handshake_pins: HandshakePins { incoming_handshake, outgoing_handshake },
            data_pins: OutputDataPins { ddr, p0,
                p1,
                p2,
                p3,
                p4,
                p5,
                p6,
                p7,},
            delay: Delay::new(),
            serial,
        }
    }

    pub fn execute(mut self, data: &str, expect_return: bool) -> (Self, ()) {
        ufmt::uwriteln!(self.serial, "Sending!").void_unwrap();

        let s = data.bytes();
        let len = (s.len() - 1) as u8; // TODO make this explicit!

        self.send_byte(len);

        self.delay.delay_us(100u8);

        for data in s {
            self.send_byte(data);
        }

        if expect_return {
            let mut inputpins = InputPins::from(self);

            let _result = inputpins.receive_byte();

            let pins = Self::from(inputpins);

            (pins, ())
        } else {
            (self, ())
        }
    }

    fn send_byte(&mut self, data: u8) {
        // Writing to serial here slows us down enough for this all to work?
        ufmt::uwriteln!(self.serial, "test").void_unwrap();

        // TODO verify handshake is correct
        // probably first handshake is wrong so second byte is read as first byte

        self.data_pins.prepare_data_for_send(data);

        // But here it does not?

        self.handshake_pins.do_write_handshake(&mut self.delay);
    }
}

impl<'a> From<InputPins<'a>> for Pins<'a> {
    fn from(ip: InputPins<'a>) -> Self {
        Self {
            handshake_pins: ip.handshake_pins,
            data_pins: OutputDataPins::from(ip.data_pins),
            delay: ip.delay,
            serial: ip.serial,
        }
    }
}

struct InputPins<'a> {
    handshake_pins: HandshakePins,
    data_pins: InputDataPins<'a>,
    delay: Delay,
    serial: &'a mut Serial<Floating>,
}

impl<'a> InputPins<'a> {
    fn receive_byte(&mut self) -> u8 {
        // Writing to serial here slows us down enough for this all to work?
        ufmt::uwriteln!(self.serial, "test").void_unwrap();

        // TODO verify handshake is correct
        // probably first handshake is wrong so second byte is read as first byte

        let res = self.data_pins.read_data();

        // But here it does not?

        self.handshake_pins.do_read_handshake(&mut self.delay);

        res
    }
}

impl<'a> From<Pins<'a>> for InputPins<'a> {
    fn from(p: Pins<'a>) -> Self {
        Self {
            handshake_pins: p.handshake_pins,
            data_pins: InputDataPins::from(p.data_pins),
            delay: p.delay,
            serial: p.serial,
        }
    }
}

struct HandshakePins {
    incoming_handshake: IncomingHandshake,
    outgoing_handshake: OutgoingHandshake,
}

impl HandshakePins {
    fn do_write_handshake(&mut self, delay: &mut Delay) {
        delay.delay_us(100u8); // TODO

        self.outgoing_handshake.set_low().void_unwrap();
        delay.delay_us(5u8); // TODO race condition somewhere? Too fast for the 6502?

        while self.incoming_handshake.is_high().void_unwrap() {}
        self.outgoing_handshake.set_high().void_unwrap();
        delay.delay_us(5u8); // TODO race condition somewhere? Too fast for the 6502?

        while self.incoming_handshake.is_low().void_unwrap() {}
        delay.delay_us(10u8); // TODO same
    }

    fn do_read_handshake(&mut self, _delay: &mut Delay) {
        unimplemented!()
    }
}

struct InputDataPins<'a> {
    ddr: &'a DDR,
    p0: P0<Input<Floating>>,
    p1: P1<Input<Floating>>,
    p2: P2<Input<Floating>>,
    p3: P3<Input<Floating>>,
    p4: P4<Input<Floating>>,
    p5: P5<Input<Floating>>,
    p6: P6<Input<Floating>>,
    p7: P7<Input<Floating>>,
}

impl<'a> InputDataPins<'a> {
    fn read_data(&self) -> u8 {
        let mut out = 0u8;
        if self.p0.is_high().void_unwrap() {
            out |= 0b00000001;
        }
        if self.p1.is_high().void_unwrap() {
            out |= 0b00000010;
        }
        if self.p2.is_high().void_unwrap() {
            out |= 0b00000100;
        }
        if self.p3.is_high().void_unwrap() {
            out |= 0b00001000;
        }
        if self.p4.is_high().void_unwrap() {
            out |= 0b00010000;
        }
        if self.p5.is_high().void_unwrap() {
            out |= 0b00100000;
        }
        if self.p6.is_high().void_unwrap() {
            out |= 0b01000000;
        }
        if self.p7.is_high().void_unwrap() {
            out |= 0b10000000;
        }
        out
    }
}

impl<'a> From<InputDataPins<'a>> for OutputDataPins<'a> {
    fn from(odp: InputDataPins<'a>) -> Self {
        Self {
            ddr: odp.ddr,
            p0: odp.p0.into_output(odp.ddr),
            p1: odp.p1.into_output(odp.ddr),
            p2: odp.p2.into_output(odp.ddr),
            p3: odp.p3.into_output(odp.ddr),
            p4: odp.p4.into_output(odp.ddr),
            p5: odp.p5.into_output(odp.ddr),
            p6: odp.p6.into_output(odp.ddr),
            p7: odp.p7.into_output(odp.ddr),
        }
    }
}

struct OutputDataPins<'a> {
    ddr: &'a DDR,
    p0: P0<Output>,
    p1: P1<Output>,
    p2: P2<Output>,
    p3: P3<Output>,
    p4: P4<Output>,
    p5: P5<Output>,
    p6: P6<Output>,
    p7: P7<Output>,
}

impl<'a> OutputDataPins<'a> {
    fn prepare_data_for_send(&mut self, data: u8) {
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

impl<'a> From<OutputDataPins<'a>> for InputDataPins<'a> {
    fn from(odp: OutputDataPins<'a>) -> Self {
        Self {
            ddr: odp.ddr,
            p0: odp.p0.into_floating_input(odp.ddr),
            p1: odp.p1.into_floating_input(odp.ddr),
            p2: odp.p2.into_floating_input(odp.ddr),
            p3: odp.p3.into_floating_input(odp.ddr),
            p4: odp.p4.into_floating_input(odp.ddr),
            p5: odp.p5.into_floating_input(odp.ddr),
            p6: odp.p6.into_floating_input(odp.ddr),
            p7: odp.p7.into_floating_input(odp.ddr),
        }
    }
}
