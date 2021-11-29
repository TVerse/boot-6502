use anyhow::Result;

use boot_6502::get_default_uart;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

fn main() -> Result<()> {
    println!("Hello World!");
    let exit = Arc::new(AtomicBool::new(false));
    let e = exit.clone();
    ctrlc::set_handler(move || {
        println!("Exiting...");
        e.store(true, Ordering::SeqCst)
    })?;

    let mut uart = get_default_uart()?;
    uart.drain()?;
    std::thread::sleep(Duration::from_millis(500));

    println!("Sending...");
    uart.write(&[0x55])?;
    uart.drain()?;
    std::thread::sleep(Duration::from_millis(500));
    println!("Sent!");
    uart.write(&[0x55])?;
    uart.drain()?;

    std::thread::sleep(Duration::from_millis(500));

    Ok(())
}
