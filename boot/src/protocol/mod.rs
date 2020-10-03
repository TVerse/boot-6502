use anyhow::Result;

pub mod serial;

pub trait Protocol {
    fn send(&self, byte: u8) -> Result<()>;
    fn receive(&self) -> Result<u8>;
}
