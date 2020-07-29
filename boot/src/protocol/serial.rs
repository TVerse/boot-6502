use std::convert::TryFrom;
use std::pin::Pin;

use anyhow::Error;
use anyhow::Result;
use lib_gpio::*;

use crate::protocol::Protocol;

pub struct SerialProtocol<I, O>
    where I: ReadableGpioPin, O: WritableGpioPin {
    clock: I,
    input_pin: I,
    output_pin: O,
    slave_select: I,
}

impl<I: ReadableGpioPin, O: WritableGpioPin> Protocol for SerialProtocol<I, O> {
    fn send(&self, byte: u8) -> Result<(), Error> {
        unimplemented!()
    }

    fn receive(&self) -> Result<u8, Error> {
        unimplemented!()
    }
}
