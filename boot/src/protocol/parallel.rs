use std::convert::TryFrom;
use std::pin::Pin;

use anyhow::Error;
use anyhow::Result;
use lib_gpio::*;

use crate::protocol::Protocol;

pub struct ParallelProtocol<I, O, B>
    where I: ReadableGpioPin, O: WritableGpioPin, B: WritableGpioPin + ReadableGpioPin
{
    handshakes: Handshakes<I, O>,
    data: [B; 4],
}

impl<I: ReadableGpioPin, O: WritableGpioPin, B: WritableGpioPin + ReadableGpioPin> ParallelProtocol<I, O, B> {
    pub fn new(incoming_handshake: I, outgoing_handshake: O, data: [B; 4]) -> ParallelProtocol<I, O, B> {
        ParallelProtocol { handshakes: Handshakes { incoming_handshake, outgoing_handshake }, data }
    }
}

impl<I: ReadableGpioPin, O: WritableGpioPin, B: WritableGpioPin + ReadableGpioPin> Protocol for ParallelProtocol<I, O, B> {
    fn send(&self, byte: u8) -> Result<()> {
        let pins = WritePins::new(&self.handshakes, &self.data);
        pins.write_byte(byte)
    }

    fn receive(&self) -> Result<u8> {
        let pins = ReadPins {
            handshakes: &self.handshakes,
            data: &self.data,
        };
        pins.read_byte()
    }
}

struct Handshakes<I, O>
    where I: ReadableGpioPin, O: WritableGpioPin {
    incoming_handshake: I,
    outgoing_handshake: O,
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

struct ReadPins<'a, I, O, D>
    where I: ReadableGpioPin, O: WritableGpioPin, D: ReadableGpioPin {
    handshakes: &'a Handshakes<I, O>,
    data: &'a [D; 4],
}

impl<'a, I: ReadableGpioPin, O: WritableGpioPin, D: ReadableGpioPin> ReadPins<'a, I, O, D> {
    pub fn new(handshakes: &'a Handshakes<I, O>, data: &'a [D; 4]) -> ReadPins<'a, I, O, D> {
        ReadPins { handshakes, data }
    }

    pub fn read_byte(&self) -> Result<u8> {
        let n1 = self.read_nibble()?;
        let n2 = self.read_nibble()?;
        Ok((n1.0 << 4) | n2.0)
    }

    fn read_nibble(&self) -> Result<Nibble> {
        self.handshakes.outgoing_handshake.write_pin(PinValue::High)?;
        loop {
            if let Ok(PinValue::Low) = self.handshakes.incoming_handshake.read_pin() {
                break;
            }
        }
        let res = read_nibble_data(self.data)?;
        self.handshakes.outgoing_handshake.write_pin(PinValue::Low)?;
        Ok(res)
    }
}

fn read_nibble_data<I: ReadableGpioPin>(input_pins: &[I; 4]) -> Result<Nibble> {
    let mut res = [PinValue::Low; 4];
    for i in 0..4 {
        res[i] = ReadableGpioPin::read_pin(&input_pins[i])?
    }
    Ok(Nibble::from(&res))
}

struct WritePins<'a, I, O, D>
    where I: ReadableGpioPin, O: WritableGpioPin, D: WritableGpioPin {
    handshakes: &'a Handshakes<I, O>,
    data: &'a [D; 4],
}

impl<'a, I: ReadableGpioPin, O: WritableGpioPin, D: WritableGpioPin> WritePins<'a, I, O, D> {
    pub fn new(handshakes: &'a Handshakes<I, O>, data: &'a [D; 4]) -> WritePins<'a, I, O, D> {
        WritePins { handshakes, data }
    }

    pub fn write_byte(&self, byte: u8) -> Result<()> {
        let n1 = Nibble::try_from(byte >> 4)?;
        let n2 = Nibble::try_from(byte & 0xF)?;
        self.write_nibble(&n1)?;
        self.write_nibble(&n2)?;
        Ok(())
    }

    fn write_nibble(&self, nibble: &Nibble) -> Result<()> {
        write_nibble_data(nibble, self.data)?;
        self.handshakes.outgoing_handshake.write_pin(PinValue::Low)?;
        loop {
            if let Ok(PinValue::Low) = self.handshakes.incoming_handshake.read_pin() {
                break;
            }
        }
        self.handshakes.outgoing_handshake.write_pin(PinValue::High)?;
        Ok(())
    }
}

fn write_nibble_data<O: WritableGpioPin>(nibble: &Nibble, output_pins: &[O; 4]) -> Result<()> {
    let pin_values: [PinValue; 4] = nibble.into();
    for (&value, pin) in pin_values.iter().zip(output_pins.into_iter()) {
        pin.write_pin(value)?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_success() {}

    #[test]
    fn test_fail() {
        panic!("Expected")
    }
}