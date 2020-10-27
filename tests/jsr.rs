use boot_6502::initialize;

use lib_io::*;

#[test]
fn test_jsr() -> Result<()> {
    let pins = initialize()?;
    run(pins).map(|_| ())
}

fn run<WH: WithHandshake, S: SendByte, D: DelayMs>(pins: Pins<WH, S, D>) -> Result<Pins<WH, S, D>> {
    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("S... ".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;
    let mut buf = [1; 1];
    let mut read = Command::ReadData {
        address: 0x0300,
        out_buffer: MutableLengthLimitedSlice::new(&mut buf)?,
    };

    let pins = pins.execute(&mut read)?;

    if buf[0] != 0 {
        panic!("Got {} but expected 0", buf[0]);
    }

    let mut jsr = Command::JSR { address: 0xE000 };

    let pins = pins.execute(&mut jsr)?.execute(&mut jsr)?;

    let mut read = Command::ReadData {
        address: 0x0300,
        out_buffer: MutableLengthLimitedSlice::new(&mut buf)?,
    };
    let pins = pins.execute(&mut read)?;

    if buf[0] != 255 {
        panic!("Got {} but expected 255", buf[0]);
    }

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
