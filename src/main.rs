use anyhow::Result;

use boot_6502::get_default_uart;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Queue, Uart};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use rand::prelude::*;

fn main() -> Result<()> {
    println!("Hello World!");
    let exit = Arc::new(AtomicBool::new(false));
    let e = exit.clone();
    //ctrlc::set_handler(move || {
    //    println!("Exiting...");
    //    e.store(true, Ordering::SeqCst)
    //})?;

    let mut uart = get_default_uart()?;
    sleep();
    let mut rng = rand::thread_rng();

    loop {
        uart.flush(Queue::Both)?;
        let b: u8 = rng.gen();
        uart.write(&[b])?;
        uart.drain()?;
        let mut rcv = [0; 16];
        let read = uart.read(&mut rcv)?;
        if read != 1 {
            println!("Read a strange number of bytes. Sent: {:#04X?}, got len: {:?}, {:#04X?}", b, read, &rcv[..read]);
        } else if rcv[0] != b {
            println!("Got the wrong byte: expected {:#04X?}, got {:#04X?}", b, rcv[0]);
        } else {
            println!("Success! Expected and got {:#04X?}", b);
        }
        sleep()
    }
}

fn sleep() {
    std::thread::sleep(Duration::from_millis(500));
}
