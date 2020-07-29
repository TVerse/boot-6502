use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use boot;
use boot::protocol::parallel::ParallelProtocol;
use gpio_cdev;
use lib_gpio;
use lib_gpio_real::GpioPin;
use std::sync::{Arc, Mutex};

fn main() {
    execute().unwrap()
}

fn execute() -> Result<()> {
    let chip = gpio_cdev::Chip::new("/dev/gpiochip0")?;
    let chip = Arc::new(Mutex::new(chip));
    let handshake_incoming = GpioPin::new(chip.clone(), 11);
    let handshake_outgoing = GpioPin::new(chip.clone(), 22);
    let data_0 = GpioPin::new(chip.clone(), 0);
    let data_1 = GpioPin::new(chip.clone(), 1);
    let data_2 = GpioPin::new(chip.clone(), 2);
    let data_3 = GpioPin::new(chip.clone(), 3);
    let data = [data_0, data_1, data_2, data_3];
    let protocol = ParallelProtocol::new(handshake_incoming, handshake_outgoing, data);
    boot::boot(protocol)
}
