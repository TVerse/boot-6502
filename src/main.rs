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
    //ctrlc::set_handler(move || {
    //    println!("Exiting...");
    //    e.store(true, Ordering::SeqCst)
    //})?;

    let mut uart = get_default_uart()?;
    std::thread::sleep(Duration::from_millis(500));
    let data: Vec<_> = (0_u8..=255).collect();

    println!("Sending...");
    uart.write(&data)?;
    uart.drain()?;
    println!("Sent!");

    std::thread::sleep(Duration::from_millis(500));

    let mut recv = [0_u8; 256];
    println!("Reading...");
    //uart.set_read_mode(2, Duration::from_millis(500))?;
    uart.read(&mut recv)?;
    if &recv != data.as_slice() {
        println!("Got a mistake! Got: {:#04X?}", &recv);
    } else {
        println!("Good!");
    }

    std::thread::sleep(Duration::from_millis(500));

    Ok(())
}
