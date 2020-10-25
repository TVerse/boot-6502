use arduino_mega2560::prelude::*;
use arduino_mega2560::{Delay, DDR};
use atmega2560_hal::port;
use atmega2560_hal::port::mode::{Floating, Input, Output};
use avr_hal_generic::void::ResultVoidExt;

use crate::{DelayMs, ReadByte, SendByte, WithHandshake};

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

pub struct WrappedDelay {
    pub delay: Delay,
}

impl DelayMs<u8> for WrappedDelay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay.delay_ms(ms);
    }
}

pub struct Read<'a> {
    pub ddr: &'a DDR,
    pub p0: P0<Input<Floating>>,
    pub p1: P1<Input<Floating>>,
    pub p2: P2<Input<Floating>>,
    pub p3: P3<Input<Floating>>,
    pub p4: P4<Input<Floating>>,
    pub p5: P5<Input<Floating>>,
    pub p6: P6<Input<Floating>>,
    pub p7: P7<Input<Floating>>,
}

impl<'a> ReadByte for Read<'a> {
    type IntoSend = Write<'a>;

    fn read(&self) -> u8 {
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

    fn into_send(self) -> Self::IntoSend {
        Self::IntoSend {
            ddr: self.ddr,
            p0: self.p0.into_output(self.ddr),
            p1: self.p1.into_output(self.ddr),
            p2: self.p2.into_output(self.ddr),
            p3: self.p3.into_output(self.ddr),
            p4: self.p4.into_output(self.ddr),
            p5: self.p5.into_output(self.ddr),
            p6: self.p6.into_output(self.ddr),
            p7: self.p7.into_output(self.ddr),
        }
    }
}

pub struct Write<'a> {
    pub ddr: &'a DDR,
    pub p0: P0<Output>,
    pub p1: P1<Output>,
    pub p2: P2<Output>,
    pub p3: P3<Output>,
    pub p4: P4<Output>,
    pub p5: P5<Output>,
    pub p6: P6<Output>,
    pub p7: P7<Output>,
}

impl<'a> SendByte for Write<'a> {
    type IntoRead = Read<'a>;

    fn send(&mut self, byte: u8) {
        if byte & 0b00000001 != 0 {
            self.p0.set_high().void_unwrap();
        } else {
            self.p0.set_low().void_unwrap();
        }
        if byte & 0b00000010 != 0 {
            self.p1.set_high().void_unwrap();
        } else {
            self.p1.set_low().void_unwrap();
        }
        if byte & 0b00000100 != 0 {
            self.p2.set_high().void_unwrap();
        } else {
            self.p2.set_low().void_unwrap();
        }
        if byte & 0b00001000 != 0 {
            self.p3.set_high().void_unwrap();
        } else {
            self.p3.set_low().void_unwrap();
        }
        if byte & 0b00010000 != 0 {
            self.p4.set_high().void_unwrap();
        } else {
            self.p4.set_low().void_unwrap();
        }
        if byte & 0b00100000 != 0 {
            self.p5.set_high().void_unwrap();
        } else {
            self.p5.set_low().void_unwrap();
        }
        if byte & 0b01000000 != 0 {
            self.p6.set_high().void_unwrap();
        } else {
            self.p6.set_low().void_unwrap();
        }
        if byte & 0b10000000 != 0 {
            self.p7.set_high().void_unwrap();
        } else {
            self.p7.set_low().void_unwrap();
        }
    }

    fn into_read(self) -> Self::IntoRead {
        Self::IntoRead {
            ddr: self.ddr,
            p0: self.p0.into_floating_input(self.ddr),
            p1: self.p1.into_floating_input(self.ddr),
            p2: self.p2.into_floating_input(self.ddr),
            p3: self.p3.into_floating_input(self.ddr),
            p4: self.p4.into_floating_input(self.ddr),
            p5: self.p5.into_floating_input(self.ddr),
            p6: self.p6.into_floating_input(self.ddr),
            p7: self.p7.into_floating_input(self.ddr),
        }
    }
}

pub struct Handshake {
    pub incoming_handshake: IncomingHandshake,
    pub outgoing_handshake: OutgoingHandshake,
    pub delay: Delay,
}

impl WithHandshake for Handshake {
    fn with_write_handshake<F: FnOnce()>(&mut self, f: F) {
        f();

        self.outgoing_handshake.set_low().void_unwrap();

        self.delay.delay_us(2u8); // At least 1 6502 clock cycle @ 1MHz

        self.outgoing_handshake.set_high().void_unwrap();

        while self.incoming_handshake.is_high().void_unwrap() {}
    }

    fn with_read_handshake<F: FnOnce() -> u8>(&mut self, f: F) -> u8 {
        while self.incoming_handshake.is_high().void_unwrap() {}

        let result = f();

        self.outgoing_handshake.set_low().void_unwrap();

        self.delay.delay_us(2u8); // At least 1 6502 clock cycle @ 1MHz TODO is this needed?

        self.outgoing_handshake.set_high().void_unwrap();

        while self.incoming_handshake.is_low().void_unwrap() {}

        result
    }
}
