use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use gpio_cdev::*;
use lib_gpio::*;
use std::sync::{Arc, Mutex};

pub struct GpioPin {
    chip: Arc<Mutex<Chip>>,
    pin: u32,
    handle: RefCell<Option<(PinState, LineHandle)>>,
}

#[derive(Clone, Copy, Debug)]
enum PinState {
    INPUT,
    OUTPUT,
}

impl GpioPin {
    pub fn new(chip: Arc<Mutex<Chip>>, pin: u32) -> GpioPin {
        GpioPin { chip, pin, handle: RefCell::new(None) }
    }

    fn ensure_readable(&self) -> Result<()> {
        let mut h = self.handle.borrow_mut();
        if let Some(PinState::INPUT) = h.as_ref().map(|x| x.0) {
            return Ok(());
        }
        let mut chip = self.chip.lock().unwrap();
        let handle = chip
            .get_line(self.pin)?
            .request(LineRequestFlags::INPUT, 0, format!("read_pin_{}", self.pin).as_str())?;
        *h = Some((PinState::INPUT, handle));
        Ok(())
    }

    fn ensure_writeable(&self) -> Result<()> {
        let mut h = self.handle.borrow_mut();
        if let Some(PinState::OUTPUT) = h.as_ref().map(|x| x.0) {
            return Ok(());
        }
        let mut chip = self.chip.lock().unwrap();
        let handle = chip
            .get_line(self.pin)?
            .request(LineRequestFlags::OUTPUT, 0, format!("read_pin_{}", self.pin).as_str())?;
        *h = Some((PinState::OUTPUT, handle));
        Ok(())
    }
}

impl ReadableGpioPin for GpioPin {
    fn read_pin(&self) -> Result<PinValue> {
        self.ensure_readable()?;
        let result = self.handle.borrow().as_ref().unwrap().1.get_value()?;
        if result == 0 {
            Ok(PinValue::Low)
        } else {
            Ok(PinValue::High)
        }
    }
}

impl WritableGpioPin for GpioPin {
    fn write_pin(&self, value: PinValue) -> Result<()> {
        self.ensure_writeable()?;
        let value = match value {
            PinValue::High => 1,
            PinValue::Low => 0,
        };
        Ok(self.handle.borrow().as_ref().unwrap().1.set_value(value)?)
    }
}