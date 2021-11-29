use anyhow::Result;
use rppal::uart::Parity;
use rppal::uart::Uart;

const DEVICE: &'static str = "/dev/ttyAMA1";
const BAUD_RATE: u32 = 9600;
const PARITY: Parity = Parity::None;
const DATA_BITS: u8 = 8;
const STOP_BITS: u8 = 1;

pub fn get_default_uart() -> Result<Uart> {
    let mut uart = Uart::with_path(DEVICE, BAUD_RATE, PARITY, DATA_BITS, STOP_BITS)?;
    uart.set_hardware_flow_control(true)?; // TODO
    Ok(uart)
}

pub fn write_bytes(uart: &mut Uart, target_location: u16, data: &[u8]) -> Result<()> {
    // TODO
    assert!(data.len() < 256);
    uart.write(&[0x01])?;
    uart.write(&target_location.to_le_bytes())?;
    uart.write(data)?;

    // TODO how does read work? What about null bytes?
    // uart.read()
    Ok(())
}

pub fn read_bytes(uart: &mut Uart, source_location: usize, data: &mut [u8]) -> Result<()> {
    // TODO
    assert!(data.len() < 256);
    uart.write(&[0x01])?;
    uart.write(data)?;
    Ok(())
}

pub fn jump(uart: &mut Uart, location: usize) -> Result<()> {
    todo!()
}

pub fn jsr(uart: &mut Uart, location: usize) -> Result<()> {
    todo!()
}
