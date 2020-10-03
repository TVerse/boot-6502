#![feature(associated_type_bounds)]

use std::cell::RefCell;
use std::error;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use boot;
use lib_gpio;
use lib_gpio::{PinValue, ReadableGpioPin, WritableGpioPin};
use lib_gpio_real::{RpiChip, RpiReadableGpioPin, RpiWritableGpioPin};

use gpio_cdev;

fn main() -> Result<()> {
    let cdev_chip: gpio_cdev::Chip = gpio_cdev::Chip::new("/dev/gpiochip0")?;
    let mut rpi_chip = RpiChip::new(cdev_chip, "6502".to_owned());
    let readable_line = RpiReadableGpioPin::new(&mut rpi_chip, 17)?;
    let writeable_line = RpiWritableGpioPin::new(&mut rpi_chip, 27)?;
    test_echo(&readable_line, &writeable_line)?;
    unreachable!()
}

fn test_echo<
    I: ReadableGpioPin<Error: error::Error + Send + Sync + 'static>,
    O: WritableGpioPin<Error: error::Error + Send + Sync + 'static>,
>(
    i: &I,
    o: &O,
) -> Result<()> {
    let mut cur_value = PinValue::High;
    loop {
        cur_value = match cur_value {
            PinValue::High => PinValue::Low,
            PinValue::Low => PinValue::High,
        };
        o.write_pin(cur_value)?;
        sleep(Duration::from_secs(2));
        let read = i.read_pin()?;
        println!("{}", cur_value == read);
        sleep(Duration::from_secs(2));
    }
}
