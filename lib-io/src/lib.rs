use crate::IoError::{ReceivedUnexpectedbyte, TooLong};
use thiserror::Error;

pub type Result<A> = std::result::Result<A, IoError>;

#[derive(Error, Debug)]
pub enum IoError {
    #[error("Length should be between 1 and 256, found {length}")]
    TooLong { length: usize },
    #[error("Received unexpected byte: expected {expected}, found {found}")]
    ReceivedUnexpectedbyte { expected: u8, found: u8 },
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub trait ReadByte {
    type IntoSend: SendByte<IntoRead = Self>;

    fn read(&self) -> Result<u8>;

    fn into_send(self) -> Result<Self::IntoSend>;
}

pub trait SendByte {
    type IntoRead: ReadByte<IntoSend = Self>;

    fn send(&mut self, byte: u8) -> Result<()>;

    fn into_read(self) -> Result<Self::IntoRead>;
}

pub trait DelayMs {
    fn delay_ms(&mut self, ms: u64);
}

pub trait DelayUs {
    fn delay_us(&mut self, us: u64);
}

pub trait WithHandshake {
    fn with_write_handshake<F: FnOnce() -> Result<()>>(&mut self, f: F) -> Result<()>;

    fn with_read_handshake<F: FnOnce() -> Result<u8>>(&mut self, f: F) -> Result<u8>;
}

pub struct AdjustedLength(u8);

impl AdjustedLength {
    fn new(len: usize) -> Result<AdjustedLength> {
        if 1 <= len && len <= 256 {
            // Wrap 256 to 0
            Ok(AdjustedLength(len as u8))
        } else {
            Err(TooLong { length: len })
        }
    }
}

pub struct LengthLimitedSlice<'a> {
    data: &'a [u8],
    data_length: AdjustedLength,
}

impl<'a> LengthLimitedSlice<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self> {
        AdjustedLength::new(data.len()).map(move |send_length| Self {
            data,
            data_length: send_length,
        })
    }
}

pub struct MutableLengthLimitedSlice<'a> {
    data: &'a mut [u8],
    data_length: AdjustedLength,
}

impl<'a> MutableLengthLimitedSlice<'a> {
    pub fn new(data: &'a mut [u8]) -> Result<Self> {
        AdjustedLength::new(data.len()).map(move |send_length| Self {
            data,
            data_length: send_length,
        })
    }
}

pub enum Command<'a> {
    DisplayString {
        data: LengthLimitedSlice<'a>, // TODO max length is really 128
    },
    WriteData {
        address: u16,
        data: LengthLimitedSlice<'a>,
    },
    ReadData {
        address: u16,
        out_buffer: MutableLengthLimitedSlice<'a>,
    },
    JSR {
        address: u16,
    },
}

impl<'a> Command<'a> {
    const DISPLAY_STRING_SIGNATURE: u8 = 0x00;
    const WRITE_DATA_SIGNATURE: u8 = 0x01;
    const READ_DATA_SIGNATURE: u8 = 0x02;
    const JSR_SIGNATURE: u8 = 0x03;

    const PLAIN_ACK: u8 = 0x01;
    const DATA_FOLLOWING_ACK: u8 = 0x02;

    fn signature_byte(&self) -> u8 {
        match self {
            Command::DisplayString { .. } => Command::DISPLAY_STRING_SIGNATURE,
            Command::WriteData { .. } => Command::WRITE_DATA_SIGNATURE,
            Command::ReadData { .. } => Command::READ_DATA_SIGNATURE,
            Command::JSR { .. } => Command::JSR_SIGNATURE,
        }
    }

    fn ack_byte(&self) -> u8 {
        match self {
            Command::DisplayString { .. } => Command::PLAIN_ACK,
            Command::WriteData { .. } => Command::PLAIN_ACK,
            Command::ReadData { .. } => Command::DATA_FOLLOWING_ACK,
            Command::JSR { .. } => Command::PLAIN_ACK,
        }
    }

    fn length(&self) -> Option<u8> {
        match self {
            Command::DisplayString { data, .. } => Some(data.data_length.0),
            Command::WriteData { data, .. } => Some(data.data_length.0),
            Command::ReadData { out_buffer, .. } => Some(out_buffer.data_length.0),
            Command::JSR { .. } => None,
        }
    }

    pub fn address(&self) -> Option<u16> {
        match self {
            Command::DisplayString { .. } => None,
            Command::WriteData { address, .. } => Some(*address),
            Command::ReadData { address, .. } => Some(*address),
            Command::JSR { address, .. } => Some(*address),
        }
    }

    fn sendable_data(&self) -> Option<&LengthLimitedSlice> {
        match self {
            Command::DisplayString { data, .. } => Some(data),
            Command::WriteData { data, .. } => Some(data),
            Command::ReadData { .. } => None,
            Command::JSR { .. } => None,
        }
    }

    fn receivable_data(&mut self) -> Option<&mut MutableLengthLimitedSlice<'a>> {
        match self {
            Command::DisplayString { .. } => None,
            Command::WriteData { .. } => None,
            Command::ReadData { out_buffer, .. } => Some(out_buffer),
            Command::JSR { .. } => None,
        }
    }
}

pub struct Pins<WH, S, D>
where
    WH: WithHandshake,
    S: SendByte,
    D: DelayMs,
{
    with_handshake: WH,
    send_byte: S,
    delay: D,
}

impl<WH, S, D> Pins<WH, S, D>
where
    WH: WithHandshake,
    S: SendByte,
    D: DelayMs,
{
    pub fn new(with_handshake: WH, send_byte: S, delay: D) -> Self {
        Self {
            with_handshake,
            send_byte,
            delay,
        }
    }

    pub fn execute(mut self, command: &mut Command) -> Result<Pins<WH, S, D>> {
        self.send_signature(command)?;
        command
            .address()
            .iter()
            .try_for_each(|a| self.send_address(*a))?;
        command
            .length()
            .iter()
            .try_for_each(|l| self.send_length(*l))?;
        command
            .sendable_data()
            .iter()
            .try_for_each(|lls| self.send_data(*lls))?;

        let read_pins = ReadPins {
            with_handshake: self.with_handshake,
            read_byte: self.send_byte.into_read()?,
            delay: self.delay,
        };

        let read_pins = read_pins.execute(command)?;

        Ok(Pins {
            with_handshake: read_pins.with_handshake,
            send_byte: read_pins.read_byte.into_send()?,
            delay: read_pins.delay,
        })
    }

    fn send_signature(&mut self, command: &Command) -> Result<()> {
        self.send_byte(command.signature_byte())
    }

    fn send_length(&mut self, length: u8) -> Result<()> {
        self.send_byte(length)
    }

    fn send_address(&mut self, address: u16) -> Result<()> {
        let address = address.to_le_bytes();

        for b in address.iter() {
            self.send_byte(*b)?;
        }

        Ok(())
    }

    fn send_data(&mut self, lls: &LengthLimitedSlice) -> Result<()> {
        let LengthLimitedSlice { data, .. } = lls;
        for d in data.iter() {
            self.send_byte(*d)?;
        }
        Ok(())
    }

    fn send_byte(&mut self, data: u8) -> Result<()> {
        let Self {
            with_handshake,
            send_byte,
            ..
        } = self;

        with_handshake.with_write_handshake(|| send_byte.send(data))
    }
}

struct ReadPins<WH, R, D>
where
    WH: WithHandshake,
    R: ReadByte,
    D: DelayMs,
{
    with_handshake: WH,
    read_byte: R,
    delay: D,
}

impl<WH, R, D> ReadPins<WH, R, D>
where
    WH: WithHandshake,
    R: ReadByte,
    D: DelayMs,
{
    fn execute(mut self, command: &mut Command) -> Result<Self> {
        // Need a certain delay here for handshakes to switch properly?
        // At least 2ms? Is there an extra WAI somewhere?
        self.delay.delay_ms(2);
        let b = self.receive_byte()?;
        if b != command.ack_byte() {
            Err(ReceivedUnexpectedbyte {
                expected: command.ack_byte(),
                found: b,
            })
        } else {
            command
                .receivable_data()
                .iter_mut()
                .try_for_each(|mlls| self.receive_data(*mlls))?;
            Ok(self)
        }
    }

    fn receive_data(&mut self, mlls: &mut MutableLengthLimitedSlice) -> Result<()> {
        let MutableLengthLimitedSlice { data, .. } = mlls;
        for d in data.iter_mut() {
            *d = self.receive_byte()?;
        }
        Ok(())
    }

    fn receive_byte(&mut self) -> Result<u8> {
        let Self {
            with_handshake,
            read_byte,
            ..
        } = self;

        with_handshake.with_read_handshake(|| read_byte.read())
    }
}
