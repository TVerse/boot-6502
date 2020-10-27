use gpio_cdev::{Chip, Line, LineHandle, LineRequestFlags};
use lib_io::*;
use lib_io_rpi::*;
use lib_io_rpi::pins::*;
use std::marker::PhantomData;

static PROGRAM: &[u8] = include_bytes!("../6502/selfcontained_test.bin");

fn get_pin(chip: &mut Chip, flags: LineRequestFlags, number: u32, description: &'static str) -> Result<(Line, LineHandle)> {
    let line = chip.get_line(number).map_err(|e| IoError::Other(Box::new(e)))?;
    let line_handle = line.request(flags, 0, description).map_err(|e| IoError::Other(Box::new(e)))?;
    Ok((line, line_handle))
}

fn main() -> Result<()> {
    let mut delay = Delay;
    let mut chip = Chip::new("/dev/gpiochip0").map_err(|e| IoError::Other(Box::new(e)))?;
    let c = &mut chip;

    let reset = 1;
    let mut reset ={
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, reset, "reset")?;
        Reset {
            line,
            handle,
        }
    };

    let ca1 = 0;
    let mut outgoing_handshake = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, ca1, "ca1")?;
        OutgoingHandshake {
            line,
            handle,
        }
    };
    let ca2 = 0;
    let incoming_handshake = {
        let (line, handle) = get_pin(c, LineRequestFlags::INPUT, ca2, "ca2")?;
        IncomingHandshake {
            line,
            handle,
        }
    };
    let pa0 = 0;
    let p0 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa0, "p0")?;
        P0{
            line,
            handle,
            _pd: PhantomData
        }
    };
    let pa1 = 0;
    let p1 ={
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa1, "p1")?;
        P1{
            line,
            handle,
            _pd: PhantomData
        }
    };
    let pa2 = 0;
    let p2 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa2, "p2")?;
        P2{
            line,
            handle,
            _pd: PhantomData
        }
    };
    let pa3 =0;
    let p3 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa3, "p3")?;
        P3{
            line,
            handle,
            _pd: PhantomData
        }
    };
    let pa4 = 0;
    let p4 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa4, "p4")?;
        P4{
            line,
            handle,
            _pd: PhantomData
        }
    };
    let pa5 = 0;
    let p5 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa5, "p5")?;
        P5{
            line,
            handle,
            _pd: PhantomData
        }
    };
    let pa6 = 0;
    let p6 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa6, "p6")?;
        P6{
            line,
            handle,
            _pd: PhantomData
        }
    };
    let pa7 =0;
    let p7 = {
        let (line, handle) = get_pin(c, LineRequestFlags::OUTPUT, pa7, "p7")?;
        P7{
            line,
            handle,
            _pd: PhantomData
        }
    };

    outgoing_handshake.set_high()?;

    reset.set_low()?;
    delay.delay_us(5);
    reset.set_high()?;

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

    let pins = Pins {
        with_handshake,
        send_byte: write,
        delay,
    };

    run(pins).map(|_| ())
}

fn run<WH: WithHandshake, S: SendByte, D: DelayMs>(
    pins: Pins<WH, S, D>,
) -> Result<Pins<WH, S, D>> {
    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("S... ".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;

    let mut program_buf = [0; 256];
    let mut pins = pins;

    for load_page in 0x00..0x3C {
        for (i, b) in program_buf.iter_mut().enumerate() {
            *b = PROGRAM[load_page + i];
        }
        let mut write_data = Command::WriteData {
            address: (load_page as u16) + 0x0300,
            data: LengthLimitedSlice::new(&program_buf)?,
        };
        pins = pins.execute(&mut write_data)?;
    }

    let mut set_ready = Command::WriteData {
        address: 0x3FF2,
        data: LengthLimitedSlice::new(&[1])?,
    };

    let pins = pins.execute(&mut set_ready)?;

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
