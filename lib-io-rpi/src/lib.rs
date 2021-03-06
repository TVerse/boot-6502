use std::convert::TryFrom;
use std::thread;
use std::time;

use lib_io::Result;
use lib_io::{DelayMs, DelayUs, IoError, ReadByte, SendByte, WithHandshake};
use pins::*;

pub mod pins;

#[derive(Copy, Clone)]
pub struct Delay;

impl DelayMs for Delay {
    fn delay_ms(&mut self, ms: u64) {
        let duration = time::Duration::from_millis(ms);
        thread::sleep(duration)
    }
}

impl DelayUs for Delay {
    fn delay_us(&mut self, us: u64) {
        let duration = time::Duration::from_micros(us);
        thread::sleep(duration)
    }
}

pub struct Read {
    p0: P0<Input>,
    p1: P1<Input>,
    p2: P2<Input>,
    p3: P3<Input>,
    p4: P4<Input>,
    p5: P5<Input>,
    p6: P6<Input>,
    p7: P7<Input>,
}

impl ReadByte for Read {
    type IntoSend = Write;

    fn read(&self) -> Result<u8> {
        let mut out = 0u8;
        if self.p0.is_high()? {
            out |= 0b00000001;
        }
        if self.p1.is_high()? {
            out |= 0b00000010;
        }
        if self.p2.is_high()? {
            out |= 0b00000100;
        }
        if self.p3.is_high()? {
            out |= 0b00001000;
        }
        if self.p4.is_high()? {
            out |= 0b00010000;
        }
        if self.p5.is_high()? {
            out |= 0b00100000;
        }
        if self.p6.is_high()? {
            out |= 0b01000000;
        }
        if self.p7.is_high()? {
            out |= 0b10000000;
        }
        Ok(out)
    }

    fn into_send(self) -> Result<Self::IntoSend> {
        Ok(Self::IntoSend {
            p0: P0::try_from(self.p0).map_err(|e| IoError::Other(Box::new(e)))?,
            p1: P1::try_from(self.p1).map_err(|e| IoError::Other(Box::new(e)))?,
            p2: P2::try_from(self.p2).map_err(|e| IoError::Other(Box::new(e)))?,
            p3: P3::try_from(self.p3).map_err(|e| IoError::Other(Box::new(e)))?,
            p4: P4::try_from(self.p4).map_err(|e| IoError::Other(Box::new(e)))?,
            p5: P5::try_from(self.p5).map_err(|e| IoError::Other(Box::new(e)))?,
            p6: P6::try_from(self.p6).map_err(|e| IoError::Other(Box::new(e)))?,
            p7: P7::try_from(self.p7).map_err(|e| IoError::Other(Box::new(e)))?,
        })
    }
}

pub struct Write {
    p0: P0<Output>,
    p1: P1<Output>,
    p2: P2<Output>,
    p3: P3<Output>,
    p4: P4<Output>,
    p5: P5<Output>,
    p6: P6<Output>,
    p7: P7<Output>,
}

impl Write {
    pub fn new(
        p0: P0<Output>,
        p1: P1<Output>,
        p2: P2<Output>,
        p3: P3<Output>,
        p4: P4<Output>,
        p5: P5<Output>,
        p6: P6<Output>,
        p7: P7<Output>,
    ) -> Self {
        Self {
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
        }
    }
}

impl SendByte for Write {
    type IntoRead = Read;

    fn send(&mut self, byte: u8) -> Result<()> {
        if byte & 0b00000001 != 0 {
            self.p0.set_high()?;
        } else {
            self.p0.set_low()?;
        }
        if byte & 0b00000010 != 0 {
            self.p1.set_high()?;
        } else {
            self.p1.set_low()?;
        }
        if byte & 0b00000100 != 0 {
            self.p2.set_high()?;
        } else {
            self.p2.set_low()?;
        }
        if byte & 0b00001000 != 0 {
            self.p3.set_high()?;
        } else {
            self.p3.set_low()?;
        }
        if byte & 0b00010000 != 0 {
            self.p4.set_high()?;
        } else {
            self.p4.set_low()?;
        }
        if byte & 0b00100000 != 0 {
            self.p5.set_high()?;
        } else {
            self.p5.set_low()?;
        }
        if byte & 0b01000000 != 0 {
            self.p6.set_high()?;
        } else {
            self.p6.set_low()?;
        }
        if byte & 0b10000000 != 0 {
            self.p7.set_high()?;
        } else {
            self.p7.set_low()?;
        }
        Ok(())
    }

    fn into_read(self) -> Result<Self::IntoRead> {
        Ok(Self::IntoRead {
            p0: P0::try_from(self.p0).map_err(|e| IoError::Other(Box::new(e)))?,
            p1: P1::try_from(self.p1).map_err(|e| IoError::Other(Box::new(e)))?,
            p2: P2::try_from(self.p2).map_err(|e| IoError::Other(Box::new(e)))?,
            p3: P3::try_from(self.p3).map_err(|e| IoError::Other(Box::new(e)))?,
            p4: P4::try_from(self.p4).map_err(|e| IoError::Other(Box::new(e)))?,
            p5: P5::try_from(self.p5).map_err(|e| IoError::Other(Box::new(e)))?,
            p6: P6::try_from(self.p6).map_err(|e| IoError::Other(Box::new(e)))?,
            p7: P7::try_from(self.p7).map_err(|e| IoError::Other(Box::new(e)))?,
        })
    }
}

pub struct Handshake {
    incoming_handshake: IncomingHandshake,
    outgoing_handshake: OutgoingHandshake,
    delay: Delay,
}

impl Handshake {
    pub fn new(
        incoming_handshake: IncomingHandshake,
        outgoing_handshake: OutgoingHandshake,
        delay: Delay,
    ) -> Self {
        Self {
            incoming_handshake,
            outgoing_handshake,
            delay,
        }
    }
}

impl WithHandshake for Handshake {
    fn with_write_handshake<F: FnOnce() -> Result<()>>(&mut self, f: F) -> Result<()> {
        f()?;

        self.outgoing_handshake.set_low()?;

        self.delay.delay_us(2); // At least 1 6502 clock cycle @ 1MHz

        self.outgoing_handshake.set_high()?;

        while self.incoming_handshake.is_high()? {}

        Ok(())
    }

    fn with_read_handshake<F: FnOnce() -> Result<u8>>(&mut self, f: F) -> Result<u8> {
        while self.incoming_handshake.is_high()? {}

        let result = f()?;

        self.outgoing_handshake.set_low()?;

        self.delay.delay_us(2); // At least 1 6502 clock cycle @ 1MHz TODO is this needed?

        self.outgoing_handshake.set_high()?;

        // TODO do I not need this?
        //println!("while low");
        //while self.incoming_handshake.is_low()? {}

        Ok(result)
    }
}
