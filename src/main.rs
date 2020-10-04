use std::error;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
// use boot;
use gpio_cdev;
// use lib_gpio;
// use lib_gpio::{Chip, PinValue, ReadableGpioPin, WritableGpioPin};
use gpio_cdev::{Chip, LineRequestFlags, EventRequestFlags};
// use lib_gpio_real::{RpiChip, RpiReadableGpioPin, RpiWritableGpioPin};

fn main() -> Result<()> {
    let cdev_chip: Chip = Chip::new("/dev/gpiochip0")?;
    // let mut rpi_chip = RpiChip::new(cdev_chip, "6502".to_owned());
    // let readable_line = RpiReadableGpioPin::new(&mut rpi_chip, 17)?;
    // let writeable_line = RpiWritableGpioPin::new(&mut rpi_chip, 27)?;
    // test_echo(&readable_line, &writeable_line)?;
    // unreachable!()
    test_shift(cdev_chip)
}

// fn test_echo<E, I: ReadableGpioPin<Error = E>, O: WritableGpioPin<Error = E>>(
//     i: &I,
//     o: &O,
// ) -> Result<()>
// where
//     E: error::Error + Send + Sync + 'static,
// {
//     let mut cur_value = PinValue::High;
//     loop {
//         cur_value = match cur_value {
//             PinValue::High => PinValue::Low,
//             PinValue::Low => PinValue::High,
//         };
//         o.write_pin(cur_value)?;
//         sleep(Duration::from_secs(1));
//         let read = i.read_pin()?;
//         println!("{:?}, {:?}, {}", cur_value, read, cur_value == read);
//         sleep(Duration::from_secs(1));
//     }
// }

fn test_shift(mut cdev_chip: Chip) -> Result<()> {
    let mut data: u8 = 0x41; // 'A'
    let clock_line = cdev_chip.get_line(17)?;
    let data_line = cdev_chip.get_line(27)?;

    let clock_events = clock_line.events(LineRequestFlags::INPUT, EventRequestFlags::FALLING_EDGE, "clock")?;
    let data_handle = data_line.request(LineRequestFlags::OUTPUT, 0, "data")?;

    for (event, _) in clock_events.zip(0..8) {
        let _evt = event?;
        let to_write = data & 0x1;
        data = data >> 1;
        data_handle.set_value(to_write)?
    }

    Ok(())
}