#![no_std]

pub type Result<A> = core::result::Result<A, &'static str>;

const TOO_LONG_ERROR: &str = "Length should be between 1 and 256";

const RECEIVED_UNEXPECTED_BYTE_ERROR: &str = "Received unexpected byte";

pub trait ReadByte {
    type IntoSend: SendByte<IntoRead = Self>;

    fn read(&self) -> u8;

    fn into_send(self) -> Self::IntoSend;
}

pub trait SendByte {
    type IntoRead: ReadByte<IntoSend = Self>;

    fn send(&mut self, byte: u8);

    fn into_read(self) -> Self::IntoRead;
}

pub trait DelayMs<A> {
    fn delay_ms(&mut self, ms: A);
}

pub trait WithHandshake {
    fn with_write_handshake<F: FnOnce()>(&mut self, f: F);

    fn with_read_handshake<F: FnOnce() -> u8>(&mut self, f: F) -> u8;
}

pub struct AdjustedLength(u8);

impl AdjustedLength {
    fn new(len: usize) -> Result<AdjustedLength> {
        if 1 <= len && len <= 256 {
            // Wrap 256 to 0
            Ok(AdjustedLength(len as u8))
        } else {
            Err(TOO_LONG_ERROR)
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

    fn address(&self) -> Option<u16> {
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
    D: DelayMs<u8>,
{
    pub with_handshake: WH,
    pub send_byte: S,
    pub delay: D,
}

impl<WH, S, D> Pins<WH, S, D>
where
    WH: WithHandshake,
    S: SendByte,
    D: DelayMs<u8>,
{
    pub fn execute(mut self, command: &mut Command) -> Result<Pins<WH, S, D>> {
        self.send_signature(command);
        command.address().iter().for_each(|a| self.send_address(*a));
        command.length().iter().for_each(|l| self.send_length(*l));
        command
            .sendable_data()
            .iter()
            .for_each(|lls| self.send_data(*lls));

        let read_pins = ReadPins {
            with_handshake: self.with_handshake,
            read_byte: self.send_byte.into_read(),
            delay: self.delay,
        };

        let read_pins = read_pins.execute(command)?;

        Ok(Pins {
            with_handshake: read_pins.with_handshake,
            send_byte: read_pins.read_byte.into_send(),
            delay: read_pins.delay,
        })
    }

    fn send_signature(&mut self, command: &Command) {
        self.send_byte(command.signature_byte())
    }

    fn send_length(&mut self, length: u8) {
        self.send_byte(length)
    }

    fn send_address(&mut self, address: u16) {
        let address = address.to_le_bytes();

        for b in address.iter() {
            self.send_byte(*b);
        }
    }

    fn send_data(&mut self, lls: &LengthLimitedSlice) {
        let LengthLimitedSlice { data, .. } = lls;
        for d in data.iter() {
            self.send_byte(*d);
        }
    }

    fn send_byte(&mut self, data: u8) {
        //serial_println!("Sending: {}", data);
        let Self {
            with_handshake,
            send_byte,
            ..
        } = self;

        with_handshake.with_write_handshake(|| send_byte.send(data));
    }
}

struct ReadPins<WH, R, D>
where
    WH: WithHandshake,
    R: ReadByte,
    D: DelayMs<u8>,
{
    with_handshake: WH,
    read_byte: R,
    delay: D,
}

impl<WH, R, D> ReadPins<WH, R, D>
where
    WH: WithHandshake,
    R: ReadByte,
    D: DelayMs<u8>,
{
    fn execute(mut self, command: &mut Command) -> Result<Self> {
        // Need a certain delay here for handshakes to switch properly?
        // At least 2ms? Is there an extra WAI somewhere?
        self.delay.delay_ms(10u8);
        if self.receive_byte() != command.ack_byte() {
            Err(RECEIVED_UNEXPECTED_BYTE_ERROR)
        } else {
            command
                .receivable_data()
                .iter_mut()
                .for_each(|mlls| self.receive_data(*mlls));
            Ok(self)
        }
    }

    fn receive_data(&mut self, mlls: &mut MutableLengthLimitedSlice) {
        let MutableLengthLimitedSlice { data, .. } = mlls;
        for d in data.iter_mut() {
            *d = self.receive_byte();
        }
    }

    fn receive_byte(&mut self) -> u8 {
        let Self {
            with_handshake,
            read_byte,
            ..
        } = self;

        with_handshake.with_read_handshake(|| read_byte.read())
    }
}
