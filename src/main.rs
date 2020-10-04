use std::error;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
// use boot;
use gpio_cdev;
// use lib_gpio;
// use lib_gpio::{Chip, PinValue, ReadableGpioPin, WritableGpioPin};
use gpio_cdev::{Chip, EventRequestFlags, LineRequestFlags, LineEvent, EventType};
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
    let clock_line = cdev_chip.get_line(17)?;
    let data_line = cdev_chip.get_line(27)?;

    let data_handle = data_line.request(LineRequestFlags::OUTPUT, 0, "data")?;

    let mut count = 0;
    let mut last_ts = None;
    let mut last_type = None;

    loop {
        let mut data: u8 = 0x41; // 'A'
        println!("Waiting...");

        let clock_events = clock_line.events(
            LineRequestFlags::INPUT,
            EventRequestFlags::BOTH_EDGES,
            "clock",
        )?;
        use std::sync::Mutex;
        let c = Mutex::new(0);
        let clock_events = clock_events.map(|x| {
            let mut d = c.lock().unwrap();
            println!("d: {}", &d);
            *d += 1;
            x
        });
        for event in clock_events.take(8) {
            let evt = event?;
            let eq = last_type.map(|t| t == evt.event_type());
            last_type = Some(evt.event_type());
            let diff = last_ts.map(|ts| evt.timestamp() - ts);
            last_ts = Some(evt.timestamp());
            println!("Got event {:?}, diff: {:?}", evt, diff);
            if let Some(false) = eq {
                return Err(anyhow!("Expected {:?}, got {:?}", last_type.unwrap(), evt.event_type()))
            };
            //  if let Some(d) = diff {
            //      if d > 700000 {
            //          return Err(anyhow!("Too high diff, missed a bit: {}", d));
            //      };
            //  };
            if evt.event_type() == EventType::FallingEdge {
                let to_write = data & 0x80;
                data = data << 1;
                println!("Writing {}, left over: {:#b}", to_write, data);
                data_handle.set_value(to_write)?
            }
        }

        println!("Shifted byte {}", count);

        count += 1;
    }
}
