use anyhow::anyhow;
use anyhow::Result;

use boot_6502::get_default_serial;
use rand::prelude::*;
use std::io::Read;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

const MSG: &'static [u8] = b"Hello!\0";

fn main() -> Result<()> {
    println!("Opening serial port");
    let mut serial = get_default_serial()?;
    let mut rng = rand::thread_rng();

    println!("Generating and sending");
    serial.write_all(MSG)?;
    println!("Flushing");
    serial.flush()?;
    let mut rcvd = [0; MSG.len()];
    println!("Reading");
    serial.read_exact(&mut rcvd)?;
    if rcvd == MSG {
        println!(
            "Success! Got {:?}, {}",
            &rcvd,
            String::from_utf8_lossy(&rcvd)
        );
    } else {
        println!("Got {:?}, {}", &rcvd, String::from_utf8_lossy(&rcvd));
    }

    Ok(())
}
