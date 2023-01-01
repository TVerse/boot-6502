use anyhow::Result;

use boot_6502::get_default_serial;
use rand::prelude::*;
use std::io::Read;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Opening serial port");
    let mut serial = get_default_serial()?;
    let mut rng = rand::thread_rng();

    loop {
        println!("Generating and sending");
        let b: u8 = rng.gen();
        serial.write_all(&[b])?;
        println!("Flushing");
        serial.flush()?;
        let mut rcvd = [0; 1];
        println!("Reading");
        serial.read_exact(&mut rcvd)?;
        if rcvd[0] != b {
            println!(
                "Got the wrong byte: expected {:#04X?}, got {:#04X?}",
                b, rcvd[0]
            );
        } else {
            println!("Success! Expected and got {:#04X?}", b);
        }
    }
}
