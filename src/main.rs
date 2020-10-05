use std::error;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;
use gpio_cdev;
use gpio_cdev::{Chip, EventRequestFlags, EventType, LineEvent, LineHandle, LineRequestFlags};

fn main() -> Result<()> {
    let mut cdev_chip: Chip = Chip::new("/dev/gpiochip0")?;
    // test_send(&mut cdev_chip)
    test_shift_poll(&mut cdev_chip)
}

fn test_send(cdev_chip: &mut Chip) -> Result<()> {
    let clock_line = cdev_chip.get_line(17)?;
    let data_line = cdev_chip.get_line(27)?;

    let data_handle = data_line.request(LineRequestFlags::OUTPUT, 0, "data")?;
    let clock_handle = clock_line.request(LineRequestFlags::OUTPUT, 1, "clock")?;

    let mut data: [u8; 15] = b"Hello from RPi!".clone();

    for d in data.iter_mut() {
        for _ in 0..8 {
            clock_handle.set_value(0)?;
            sleep(Duration::from_millis(1));
            let to_write = *d & 0x80;
            *d = *d << 1;
            sleep(Duration::from_millis(1));
            data_handle.set_value(to_write)?;
            sleep(Duration::from_millis(1));
            clock_handle.set_value(1)?;
            sleep(Duration::from_millis(1));
        }
    }

    Ok(())
}

fn test_shift(cdev_chip: &mut Chip) -> Result<()> {
    let clock_line = cdev_chip.get_line(17)?;
    let data_line = cdev_chip.get_line(27)?;

    let data_handle = data_line.request(LineRequestFlags::OUTPUT, 0, "data")?;

    let mut count = 0;
    let mut last_ts = None;
    let mut last_type = None;

    loop {
        let mut data: u8 = 0b10101010;
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
        let mut byte_counter = 0;
        for event in clock_events {
            let evt = event?;
            let eq = last_type.as_ref().map(|t| *t == evt.event_type());
            let diff = last_ts.map(|ts| evt.timestamp() - ts);
            last_ts = Some(evt.timestamp());
            println!("Got event {:?}, diff: {:?}", evt, diff);
            if let Some(true) = eq {
                return Err(anyhow!("Got {:?} twice!", evt.event_type()));
            };
            last_type = Some(evt.event_type());
            //  if let Some(d) = diff {
            //      if d > 700000 {
            //          return Err(anyhow!("Too high diff, missed a bit: {}", d));
            //      };
            //  };
            if evt.event_type() == EventType::FallingEdge {
                let to_write = data & 0x80;
                data = data << 1;
                println!("Writing {}, left over: {:#b}", to_write, data);
                data_handle.set_value(to_write)?;
                byte_counter += 1;
                if byte_counter == 8 {
                    break;
                }
            }
        }

        println!("Shifted byte {}", count);

        count += 1;
    }
}

fn test_shift_poll(cdev_chip: &mut Chip) -> Result<()> {
    let clock_line = cdev_chip.get_line(17)?;
    let data_line = cdev_chip.get_line(27)?;

    let data_handle = data_line.request(LineRequestFlags::OUTPUT, 0, "data")?;
    let clock_handle = clock_line.request(LineRequestFlags::INPUT, 0, "clock")?;

    let mut count = 0;

    loop {
        let mut data: u8 = 0b10101010;
        println!("Waiting...");

        let mut bit_counter = 0;
        while bit_counter != 8 {
            wait_for_falling_edge(&clock_handle)?;
            let to_write = data & 0x80;
            data = data << 1;
            println!("Writing {}, left over: {:#b}", to_write, data);
            data_handle.set_value(to_write)?;
            bit_counter += 1;
        }

        println!("Shifted byte {}", count);

        count += 1;
    }
}

fn wait_for_falling_edge(line_handle: &LineHandle) -> Result<()> {
    let mut cur = line_handle.get_value()?;
    while cur == 0 {
        cur = line_handle.get_value()?;
    }

    while cur == 1 {
        cur = line_handle.get_value()?;
    }

    Ok(())
}
