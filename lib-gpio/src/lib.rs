use anyhow::Result;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PinValue {
    High,
    Low,
}

impl PinValue {
    pub fn as_u8(&self) -> u8 {
        if *self == PinValue::High {
            1
        } else {
            0
        }
    }

    pub fn from_u8(u: u8) -> PinValue {
        if u == 0 {
            PinValue::Low
        } else {
            PinValue::High
        }
    }
}

pub trait WritableGpioPin {
    fn write_pin(&self, value: PinValue) -> Result<()>;
}

pub trait ReadableGpioPin {
    fn read_pin(&self) -> Result<PinValue>;
}
