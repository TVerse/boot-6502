#![no_std]

use core::marker::PhantomData;

use arduino_mega2560::prelude::*;
use arduino_mega2560::{Delay, Serial, DDR};
use atmega2560_hal::port;
use atmega2560_hal::port::mode::{Floating, Input, Output};
use avr_hal_generic::void::ResultVoidExt;
use ufmt::derive::uDebug;

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

/*
Commands:
* Write N bytes to address A
* Read N bytes from address A
* Read from register
* JMP A
* JSR A
* Print string at address A? (can use jump for this)
 */

pub type Result<A> = core::result::Result<A, &'static str>;

static TOO_LONG_ERROR: &str = "Length should be between 1 and 256";

static RECEIVED_UNEXPECTED_BYTE_ERROR: &str = "Received unexpected byte";

#[derive(uDebug)]
pub struct AdjustedLength(u8);

impl AdjustedLength {
    fn new(len: usize) -> Result<AdjustedLength> {
        if 1 <= len && len <= 256 {
            // Wrap 256 to 0
            Ok(AdjustedLength(len as u8))
        } else {
            Err(TOO_LONG_ERROR)
        }
    }
}

#[derive(uDebug)]
pub struct LengthLimitedSlice<'a> {
    data: &'a [u8],
    data_length: AdjustedLength,
}

impl<'a> LengthLimitedSlice<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self> {
        AdjustedLength::new(data.len()).map(move |send_length| Self {
            data,
            data_length: send_length,
        })
    }
}

#[derive(uDebug)]
pub struct MutableLengthLimitedSlice<'a> {
    data: &'a mut [u8],
    data_length: AdjustedLength,
}

impl<'a> MutableLengthLimitedSlice<'a> {
    pub fn new(data: &'a mut [u8]) -> Result<Self> {
        AdjustedLength::new(data.len()).map(move |send_length| Self {
            data,
            data_length: send_length,
        })
    }
}

#[derive(uDebug)]
pub enum Command<'a> {
    DisplayString {
        data: LengthLimitedSlice<'a>,
    },
    WriteData {
        address: u16,
        data: LengthLimitedSlice<'a>,
    },
    ReadData {
        address: u16,
        out_buffer: MutableLengthLimitedSlice<'a>,
    },
}

impl Command<'_> {
    fn signature_byte(&self) -> u8 {
        match self {
            Command::DisplayString { .. } => 0x01,
            Command::WriteData { .. } => 0x02,
            Command::ReadData { .. } => 0x03,
        }
    }
}

pub struct PinsState<'a, F, A>
where
    F: FnOnce(Pins<'a>) -> (Pins<'a>, Result<A>),
{
    run: F,
    _pd: PhantomData<Pins<'a>>,
}

impl<'a, F, A> PinsState<'a, F, A>
where
    F: FnOnce(Pins<'a>) -> (Pins<'a>, Result<A>),
{
    pub fn new(run: F) -> Self {
        PinsState {
            run,
            _pd: PhantomData,
        }
    }

    pub fn run(self, pins: Pins<'a>) -> (Pins<'a>, Result<A>) {
        (self.run)(pins)
    }

    pub fn flat_map<B, G, H>(
        self,
        f: H,
    ) -> PinsState<'a, impl FnOnce(Pins<'a>) -> (Pins<'a>, Result<B>), B>
    where
        G: FnOnce(Pins<'a>) -> (Pins<'a>, Result<B>),
        H: FnOnce(A) -> PinsState<'a, G, B>,
    {
        PinsState {
            run: {
                |pins| {
                    let (p, a) = (self.run)(pins);
                    match a {
                        Ok(a) => {
                            let pins_state = f(a);
                            (pins_state.run)(p)
                        }
                        Err(e) => (p, Err(e)),
                    }
                }
            },
            _pd: PhantomData,
        }
    }
}

pub struct Pins<'a> {
    handshake_pins: HandshakePins,
    data_pins: OutputDataPins<'a>,
    delay: Delay,
    pub serial: &'a mut Serial<Floating>,
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
            handshake_pins: HandshakePins {
                incoming_handshake,
                outgoing_handshake,
            },
            data_pins: OutputDataPins {
                ddr,
                p0,
                p1,
                p2,
                p3,
                p4,
                p5,
                p6,
                p7,
            },
            delay: Delay::new(),
            serial,
        }
    }

    // TODO pass signature byte? Handle this another way?
    // TODO put entire ACK+read phase in InputPins for better delay handling when switching modes
    pub fn execute(self, command: &mut Command) -> (Self, Result<()>) {
        ufmt::uwriteln!(self.serial, "Sending!").void_unwrap();
        match command {
            Command::DisplayString { data: lls } => self.handle_display_string(&lls),
            Command::WriteData { address, data: lls } => self.handle_write_data(*address, &lls),
            Command::ReadData {
                address,
                out_buffer,
            } => self.handle_read_data(*address, out_buffer),
        }
    }

    fn handle_display_string(mut self, lls: &LengthLimitedSlice) -> (Self, Result<()>) {
        ufmt::uwrite!(self.serial, "Displaying string...").void_unwrap();
        let LengthLimitedSlice { data, data_length } = lls;
        self.send_byte(0x00);

        // TODO could also grab length out of the slice here
        self.send_byte(data_length.0);

        for d in data.iter() {
            self.send_byte(*d);
        }

        let mut inputpins = InputPins::from(self);

        let ack = inputpins.receive_byte();

        let pins = Self::from(inputpins);

        match ack {
            0x01 => {
                ufmt::uwriteln!(pins.serial, "Done!").void_unwrap();

                (pins, Ok(()))
            }
            _ => (pins, Err(RECEIVED_UNEXPECTED_BYTE_ERROR)),
        }
    }

    fn handle_write_data(mut self, address: u16, lls: &LengthLimitedSlice) -> (Self, Result<()>) {
        ufmt::uwrite!(self.serial, "Writing data...").void_unwrap();
        let LengthLimitedSlice { data, data_length } = lls;
        self.send_byte(0x01);

        let address = address.to_le_bytes();

        for b in address.iter() {
            self.send_byte(*b);
        }

        // TODO could also grab length out of the slice here
        self.send_byte(data_length.0);

        for d in data.iter() {
            self.send_byte(*d);
        }

        let mut inputpins = InputPins::from(self);

        let ack = inputpins.receive_byte();

        let pins = Self::from(inputpins);
        match ack {
            0x01 => {
                ufmt::uwriteln!(pins.serial, "Done!").void_unwrap();

                (pins, Ok(()))
            }
            _ => (pins, Err(RECEIVED_UNEXPECTED_BYTE_ERROR)),
        }
    }

    fn handle_read_data(
        mut self,
        address: u16,
        mlls: &mut MutableLengthLimitedSlice,
    ) -> (Self, Result<()>) {
        ufmt::uwrite!(self.serial, "Requesting data...").void_unwrap();
        let MutableLengthLimitedSlice { data, data_length } = mlls;
        self.send_byte(0x02);

        let address = address.to_le_bytes();
        for b in address.iter() {
            self.send_byte(*b);
        }

        self.send_byte(data_length.0);
        let mut inputpins = InputPins::from(self);
        let ack = inputpins.receive_byte();

        match ack {
            0x02 => {
                for d in data.iter_mut() {
                    *d = inputpins.receive_byte();
                }

                let pins = Self::from(inputpins);

                (pins, Ok(()))
            }
            _ => (Self::from(inputpins), Err(RECEIVED_UNEXPECTED_BYTE_ERROR)),
        }
    }

    fn send_byte(&mut self, data: u8) {
        ufmt::uwriteln!(self.serial, "Sending {}", data).void_unwrap();

        let Self {
            handshake_pins,
            data_pins,
            delay,
            serial,
            ..
        } = self;

        handshake_pins
            .with_write_handshake(delay, serial, || data_pins.prepare_data_for_send(data));
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
        let Self {
            handshake_pins,
            data_pins,
            delay,
            serial,
            ..
        } = self;

        let result = handshake_pins.with_read_handshake(delay, serial, || data_pins.read_data());

        ufmt::uwriteln!(self.serial, "Received {}", result).void_unwrap();

        result
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
    fn with_write_handshake<F: FnOnce()>(
        &mut self,
        delay: &mut Delay,
        _serial: &mut Serial<Floating>,
        f: F,
    ) {
        f();

        self.outgoing_handshake.set_low().void_unwrap();
        delay.delay_us(2u8); // At least 1 6502 clock cycle @ 1MHz

        self.outgoing_handshake.set_high().void_unwrap();

        while self.incoming_handshake.is_high().void_unwrap() {}
    }

    fn with_read_handshake<F: FnOnce() -> u8>(
        &mut self,
        delay: &mut Delay,
        _serial: &mut Serial<Floating>,
        f: F,
    ) -> u8 {
        // Need a certain delay here for handshakes to switch properly?
        // At least 2ms? Is there an extra WAI somewhere?
        delay.delay_ms(2u8);
        while self.incoming_handshake.is_high().void_unwrap() {}

        let result = f();

        self.outgoing_handshake.set_low().void_unwrap();

        delay.delay_us(2u8); // At least 1 6502 clock cycle @ 1MHz

        self.outgoing_handshake.set_high().void_unwrap();

        result
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
