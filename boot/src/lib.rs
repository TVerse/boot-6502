use std::convert::TryFrom;
use std::net::Shutdown::Read;
use std::ops::Range;
use std::pin::Pin;

use anyhow::Error;
use anyhow::Result;
use lib_gpio::*;

pub struct PinConfig<I, O, B>
    where I: ReadableGpioPin, O: WritableGpioPin, B: WritableGpioPin + ReadableGpioPin
{
    incoming_handshake: I,
    outgoing_handshake: O,
    data: [B; 4],
}

impl<I: ReadableGpioPin, O: WritableGpioPin, B: WritableGpioPin + ReadableGpioPin> PinConfig<I, O, B> {
    pub fn new(incoming_handshake: I, outgoing_handshake: O, data: [B; 4]) -> PinConfig<I, O, B> {
        PinConfig { incoming_handshake, outgoing_handshake, data }
    }
}

pub fn boot<I: ReadableGpioPin, O: WritableGpioPin, B: ReadableGpioPin + WritableGpioPin>(pin_config: PinConfig<I, O, B>) -> Result<()> {
    Ok(())
}

struct Nibble(u8);

impl TryFrom<u8> for Nibble {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value & 0xF0 != 0 {
            Err(Error::msg(format!("Invalid nibble: {}", value)))
        } else {
            Ok(Nibble(value))
        }
    }
}

impl From<&[PinValue; 4]> for Nibble {
    fn from(value: &[PinValue; 4]) -> Self {
        Nibble(value[0].as_u8() | (value[1].as_u8() << 1) | (value[2].as_u8() << 2) | (value[3].as_u8() << 3))
    }
}

impl From<&Nibble> for [PinValue; 4] {
    fn from(n: &Nibble) -> Self {
        [PinValue::from_u8(n.0 & (1 << 0)),
            PinValue::from_u8(n.0 & (1 << 1)),
            PinValue::from_u8(n.0 & (1 << 2)),
            PinValue::from_u8(n.0 & (1 << 3))]
    }
}

fn read_nibble<I: ReadableGpioPin>(input_pins: &[I; 4]) -> Result<Nibble> {
    // let res = input_pins.iter().map(ReadableGpioPin::read_pin).collect()?;
    // Ok(Nibble::from(&res))
    todo!()
}

fn write_nibble<O: WritableGpioPin>(nibble: &Nibble, output_pins: &[O; 4]) -> Result<()> {
    let pin_values: [PinValue; 4] = nibble.into();
    for (&value, pin) in pin_values.into_iter().zip(output_pins.into_iter()) {
        pin.write_pin(value)?;
    }
    Ok(())
}