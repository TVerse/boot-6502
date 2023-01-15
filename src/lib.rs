use anyhow::Result;
use std::mem;
use tokio_serial::{DataBits, FlowControl, Parity, SerialStream, StopBits};

pub fn get_default_serial() -> Result<SerialStream> {
    // Note that HW flow control only works for transmitting to 6502, not receiving
    // Assume our receive buffer is large enough (which it will be)
    let builder = tokio_serial::new("/dev/ttyAMA1", 600)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .flow_control(FlowControl::Hardware);
    let stream = SerialStream::open(&builder)?;
    Ok(stream)
}

const FRAME_START: u8 = b'(';
const FRAME_END: u8 = b')';
const ESCAPE_CHAR: u8 = b'\\';
const ESCAPED_XOR: u8 = 0x20;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    payload: Vec<u8>,
}

impl Frame {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.payload.len() + 3 + 10);
        out.push(FRAME_START);
        for &b in self.payload.iter() {
            match b {
                FRAME_START | FRAME_END | ESCAPE_CHAR => {
                    out.push(ESCAPE_CHAR);
                    out.push(b ^ ESCAPED_XOR)
                }
                _ => out.push(b),
            }
        }
        out.push(FRAME_END);

        out
    }
}

#[derive(Debug, Copy, Clone)]
enum DeserializationState {
    Waiting,
    Accepted,
    Escaped,
}

#[derive(Debug)]
pub struct FrameDeserializer {
    deserialization_state: DeserializationState,
    current_frame_buffer: Vec<u8>,
}

impl Default for FrameDeserializer {
    fn default() -> Self {
        Self {
            deserialization_state: DeserializationState::Waiting,
            current_frame_buffer: Vec::new(),
        }
    }
}

impl FrameDeserializer {
    pub fn push(&mut self, byte: u8) -> Option<Frame> {
        match self.deserialization_state {
            DeserializationState::Waiting => {
                if byte == FRAME_START {
                    self.deserialization_state = DeserializationState::Accepted
                }
                None
            }
            DeserializationState::Accepted => {
                match byte {
                    FRAME_END => {
                        self.deserialization_state = DeserializationState::Waiting;
                        Some(Frame::new(mem::take(&mut self.current_frame_buffer)))
                    }
                    ESCAPE_CHAR => {
                        self.deserialization_state = DeserializationState::Escaped;
                        None
                    }
                    FRAME_START => {
                        // Unescaped start byte: we missed an end, clear and restart
                        println!(
                            "Got an unexpected start byte, throwing away buffer {:?}",
                            self.current_frame_buffer
                        );
                        self.current_frame_buffer.clear();
                        None
                    }
                    _ => {
                        self.current_frame_buffer.push(byte);
                        None
                    }
                }
            }
            DeserializationState::Escaped => {
                self.current_frame_buffer.push(byte ^ ESCAPED_XOR);
                self.deserialization_state = DeserializationState::Accepted;
                None
            }
        }
    }
}

pub enum Payload {
    Echo(Vec<u8>),
    Echoed(Vec<u8>),
}

impl Payload {
    pub fn serialize(&self) -> Vec<u8> {
        let type_byte = self.type_byte();
        let mut out = Vec::new();
        out.push(type_byte);
        match self {
            Payload::Echo(payload) => {
                out.extend(payload);
            }
            Payload::Echoed(payload) => {
                out.extend(payload);
            }
        }
        out
    }

    fn type_byte(&self) -> u8 {
        match self {
            Payload::Echo(_) => 0x01,
            Payload::Echoed(_) => 0x02,
        }
    }

    pub fn deserialize(type_byte: u8, data: Vec<u8>) -> Option<Self> {
        match type_byte {
            0x01 => Some(Self::Echo(data)),
            0x02 => Some(Self::Echo(data)),
            _ => {
                println!("Got unknown payload type byte 0x{type_byte:02x}");
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn basic_serde(bytes: Vec<u8>) {
            let frame = Frame::new(bytes);
            let serialized = frame.serialize();
            let mut deserializer = FrameDeserializer::default();
            let mut res = None;
            for b in serialized {
                let r = deserializer.push(b);
                if r.is_some() {
                    res = r;
                    break
                }
            }
            prop_assert_eq!(frame, res.unwrap())
        }

        #[test]
        fn serde_preceding_garbage(mut garbage: Vec<u8>, bytes: Vec<u8>) {
            let frame = Frame::new(bytes);
            let serialized = frame.serialize();
            let mut deserializer = FrameDeserializer::default();
            let mut res = None;
            if let Some(&end) = garbage.last() {
                // If we end with escape, it won't recognize the next frame
                if end == ESCAPE_CHAR {
                    garbage.pop();
                }
            }
            deserializer.push(FRAME_START);
            for b in garbage {
                deserializer.push(b);
            }
            // Byte before frame_start should not be ESCAPE or we lose another frame
            for b in serialized {
                let r = deserializer.push(b);
                if r.is_some() {
                    res = r;
                    break
                }
            }
            prop_assert_eq!(frame, res.unwrap())
        }
    }
}
