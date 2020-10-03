use gpio_cdev;
use gpio_cdev::{LineHandle, LineRequestFlags};
use lib_gpio;
use lib_gpio::{PinValue, ReadableGpioPin, WritableGpioPin};
use thiserror::Error;

use crate::GpioError::PinAlreadyInUseError;

pub struct RpiChip {
    inner: gpio_cdev::Chip,
    in_use_pins: Vec<u32>,
    consumer: String,
}

impl RpiChip {
    pub fn new(cdev_chip: gpio_cdev::Chip, consumer: String) -> Self {
        Self {
            inner: cdev_chip,
            in_use_pins: Vec::with_capacity(8),
            consumer,
        }
    }
}

#[derive(Error, Debug)]
pub enum GpioError {
    #[error("Pin {} already in use", .pin)]
    PinAlreadyInUseError {
        pin: u32
    },
    #[error(transparent)]
    GpioCdevError {
        #[from]
        source: gpio_cdev::errors::Error,
    },
}

pub struct RpiReadableGpioPin {
    handle: LineHandle,
}

impl RpiReadableGpioPin {
    pub fn new(chip: &mut RpiChip, pin: u32) -> Result<Self, GpioError> {
        Ok(Self { handle: get_handle(chip, pin, LineRequestFlags::INPUT)? })
    }
}

impl ReadableGpioPin for RpiReadableGpioPin {
    type Error = GpioError;

    fn read_pin(&self) -> Result<PinValue, Self::Error> {
        if self.handle.get_value()? == 0 {
            Ok(PinValue::Low)
        } else {
            Ok(PinValue::High)
        }
    }
}

pub struct RpiWritableGpioPin {
    handle: LineHandle,
}

impl RpiWritableGpioPin {
    pub fn new(chip: &mut RpiChip, pin: u32) -> Result<Self, GpioError> {
        Ok(Self { handle: get_handle(chip, pin, LineRequestFlags::OUTPUT)? })
    }
}

impl WritableGpioPin for RpiWritableGpioPin {
    type Error = GpioError;

    fn write_pin(&self, value: PinValue) -> Result<(), Self::Error> {
        let val = if value == PinValue::Low {
            0
        } else {
            1
        };
        Ok(self.handle.set_value(val)?)
    }
}

fn get_handle(chip: &mut RpiChip, pin: u32, flags: LineRequestFlags) -> Result<LineHandle, GpioError> {
    if chip.in_use_pins.contains(&pin) {
        Err(PinAlreadyInUseError { pin })
    } else {
        let handle = chip.inner.get_line(pin)?.request(
            flags,
            0,
            chip.consumer.as_str(),
        )?;
        chip.in_use_pins.push(pin);
        Ok(handle)
    }
}
