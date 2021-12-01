use anyhow::Result;
use serial_core::BaudRate;
use serial_core::CharSize;
use serial_core::FlowControl;
use serial_core::Parity;
use serial_core::SerialDevice;
use serial_core::SerialPortSettings;
use serial_core::StopBits;
use serial_unix::TTYPort;
use serial_unix::TTYSettings;
use std::path::Path;

pub fn get_default_serial() -> Result<TTYPort> {
    let mut tty = TTYPort::open(Path::new("/dev/ttyAMA1"))?;
    tty.set_timeout(std::time::Duration::from_millis(1000))?; // TODO, this long is required for reading
    let mut settings = tty.read_settings()?;
    settings.set_baud_rate(BaudRate::Baud9600)?;
    settings.set_parity(Parity::ParityNone);
    settings.set_char_size(CharSize::Bits8);
    settings.set_stop_bits(StopBits::Stop1);
    settings.set_flow_control(FlowControl::FlowHardware);
    tty.write_settings(&settings)?;
    Ok(tty)
}
