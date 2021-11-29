use anyhow::Result;

use rppal::uart::{Parity, Uart};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use boot_6502::get_default_uart;

const DEVICE: &'static str = "/dev/???";
const BAUD_RATE: u32 = 9600;
const PARITY: Parity = Parity::None;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

fn main() -> Result<()> {
    println!("Hello World!");
    let exit = Arc::new(AtomicBool::new(false));
    let e = exit.clone();
    ctrlc::set_handler(move || {
        println!("Exiting...");
        e.store(true, Ordering::SeqCst)
    })?;
    let mut uart = get_default_uart();

    println!("Sending...");
    uart.write(b"Hi!")?;
    println!("Sent!");

    std::thread::sleep(Duration::from_millis(500));

    Ok(())
}
