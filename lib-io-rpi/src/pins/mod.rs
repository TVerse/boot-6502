use gpio_cdev::LineRequestFlags;
use lib_io::Result;

#[macro_use]
mod generator_macros;

pub struct Input;
pub struct Output;

pub trait Selector {
    fn flag() -> LineRequestFlags;
}

impl Selector for Input {
    fn flag() -> LineRequestFlags {
        LineRequestFlags::INPUT
    }
}

impl Selector for Output {
    fn flag() -> LineRequestFlags {
        LineRequestFlags::OUTPUT
    }
}

pub trait InputPin {
    fn is_high(&self) -> Result<bool>;

    fn is_low(&self) -> Result<bool> {
        self.is_high().map(|b| !b)
    }
}

pub trait OutputPin {
    fn set_high(&mut self) -> Result<()>;

    fn set_low(&mut self) -> Result<()>;
}

switchable_pin!(P0);
switchable_pin!(P1);
switchable_pin!(P2);
switchable_pin!(P3);
switchable_pin!(P4);
switchable_pin!(P5);
switchable_pin!(P6);
switchable_pin!(P7);
input_pin!(IncomingHandshake);
output_pin!(OutgoingHandshake);
output_pin!(Reset);
