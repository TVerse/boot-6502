use std::convert::TryFrom;
use std::io::{self, BufRead, Read};
use std::ops::Range;
use std::pin::Pin;

use anyhow::Error;
use anyhow::Result;
use lib_gpio::*;

use crate::protocol::Protocol;

pub mod protocol;

pub fn boot<T: Protocol>(protocol: T) -> Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        let mut buffer = String::new();
        handle.read_line(&mut buffer)?;
        protocol.send(0xFF)?;
        for &b in buffer.as_bytes() {
            protocol.send(b)?;
        }
    }
}
