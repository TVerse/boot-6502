use anyhow::anyhow;
use anyhow::Result;

use boot_6502::get_default_serial;
use tokio_serial::SerialStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const MSG: &[u8] = b"xHello!\0";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Opening serial port");
    let mut serial = get_default_serial()?;

    let res = ping(&mut serial).await;
    serial.flush().await?;
    res
}

async fn ping(serial: &mut SerialStream) -> Result<()> {
    loop {
        println!("Waiting for signal");
        serial.flush().await?;
        let mut rcvd = [0; 1];
        serial.read_exact(&mut rcvd).await?;
        if rcvd != [0x55] {
            return Err(anyhow!("Wrong init byte, got {rcvd:?}"));
        }
        println!("Generating and sending");
        serial.write_all(MSG).await?;
        println!("Flushing");
        serial.flush().await?;
        let mut rcvd = [0; MSG.len()];
        println!("Reading");
        serial.read_exact(&mut rcvd).await?;
        if rcvd == MSG {
            println!(
                "Success! Got {:?}, {}",
                &rcvd,
                String::from_utf8_lossy(&rcvd)
            );
        } else {
            println!("Got {:?}, {}", &rcvd, String::from_utf8_lossy(&rcvd));
        }
    }
}
