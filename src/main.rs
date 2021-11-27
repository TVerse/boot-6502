use anyhow::Result;

use rppal::gpio::Gpio;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

fn main() -> Result<()> {
    println!("Hello World!");
    let exit = Arc::new(AtomicBool::new(false));
    let e = exit.clone();
    ctrlc::set_handler(move || {
        println!("Exiting...");
        e.store(true, Ordering::SeqCst)
    })?;
    let gpio = Gpio::new()?;
    let mut pin = gpio.get(13)?.into_output();
    loop {
        pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(500));
        pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(500));
        if exit.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}
