use std::time::Duration;
use anyhow::anyhow;
use anyhow::Result;

use boot_6502::{Frame, FrameDeserializer, get_default_serial, Payload};
use tokio_serial::SerialStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::select;
use tokio::time::sleep;

const MSG: &[u8] = b"Hello!";

#[tokio::main]
async fn main() -> Result<()> {
    println!("Opening serial port");
    let mut serial = get_default_serial()?;

    let res = echo(&mut serial).await;
    serial.flush().await?;
    res
}

async fn echo(serial: &mut SerialStream) -> Result<()> {
    let mut deserializer = FrameDeserializer::default();
    loop {
        let frame = Frame::new(Payload::Echo(MSG.to_vec()).serialize());
        serial.write(&frame.serialize()).await?;
        let sleep = sleep(Duration::from_millis(500));
        tokio::pin!(sleep);
        let mut buf = Vec::new();
        let b = select! {
            r = serial.read_u8() => r?,
            _ = &mut sleep => {
                println!("No response");
                continue
            },
        };
        buf.push(b);
        let out_frame = deserializer.push(b);
        println!("Raw buf: {:?}", &buf);
        if let Some(f) = out_frame {
            println!("Frame returned: {f:?}");
            buf.clear();
        }
    }
}
