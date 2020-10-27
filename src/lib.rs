use gpio_cdev::{Chip, Line, LineHandle, LineRequestFlags};
use lib_io::*;
use lib_io_rpi::pins::*;
use lib_io_rpi::*;
use std::marker::PhantomData;

fn get_pin(
    chip: &mut Chip,
    flags: LineRequestFlags,
    number: u32,
    description: &'static str,
) -> Result<(Line, LineHandle)> {
    let line = chip
        .get_line(number)
        .map_err(|e| IoError::Other(Box::new(e)))?;
    let line_handle = line
        .request(flags, 0, description)
        .map_err(|e| IoError::Other(Box::new(e)))?;
    Ok((line, line_handle))
}

pub fn initialize() -> Result<Pins<Handshake, Write, Delay>> {
    let mut delay = Delay;
    let mut chip = Chip::new("/dev/gpiochip0").map_err(|e| IoError::Other(Box::new(e)))?;
    let c = &mut chip;

    let reset = 13;
    let mut reset = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, reset, "reset")?;
        Reset { line, handle }
    };

    let ca1 = 26;
    let mut outgoing_handshake = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, ca1, "ca1")?;
        OutgoingHandshake { line, handle }
    };
    let ca2 = 19;
    let incoming_handshake = {
        let (line, handle) = get_pin(c, LineRequestFlags::INPUT, ca2, "ca2")?;
        IncomingHandshake { line, handle }
    };
    let pa0 = 21;
    let p0 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa0, "p0")?;
        P0 {
            line,
            handle,
            _pd: PhantomData,
        }
    };
    let pa1 = 20;
    let p1 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa1, "p1")?;
        P1 {
            line,
            handle,
            _pd: PhantomData,
        }
    };
    let pa2 = 6;
    let p2 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa2, "p2")?;
        P2 {
            line,
            handle,
            _pd: PhantomData,
        }
    };
    let pa3 = 5;
    let p3 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa3, "p3")?;
        P3 {
            line,
            handle,
            _pd: PhantomData,
        }
    };
    let pa4 = 22;
    let p4 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa4, "p4")?;
        P4 {
            line,
            handle,
            _pd: PhantomData,
        }
    };
    let pa5 = 27;
    let p5 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa5, "p5")?;
        P5 {
            line,
            handle,
            _pd: PhantomData,
        }
    };
    let pa6 = 17;
    let p6 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa6, "p6")?;
        P6 {
            line,
            handle,
            _pd: PhantomData,
        }
    };
    let pa7 = 4;
    let p7 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa7, "p7")?;
        P7 {
            line,
            handle,
            _pd: PhantomData,
        }
    };

    outgoing_handshake.set_high()?;

    println!("Resetting...");
    reset.set_low()?;
    delay.delay_us(5);
    reset.set_high()?;

    println!("Waiting...");
    delay.delay_ms(2000);
    while incoming_handshake.is_low()? {}

    let with_handshake = Handshake {
        incoming_handshake,
        outgoing_handshake,
        delay,
    };
    let write = Write {
        p0,
        p1,
        p2,
        p3,
        p4,
        p5,
        p6,
        p7,
    };

    Ok(Pins {
        with_handshake,
        send_byte: write,
        delay,
    })
}
