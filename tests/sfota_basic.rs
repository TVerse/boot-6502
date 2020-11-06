use boot_6502::*;

use lib_io::*;

#[test]
fn test_sfota() -> Result<()> {
    let pins = initialize()?;
    run(pins).map(|_| ())
}

static PROGRAM: &str = "  STZ #$0300\n  RTS\n";

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

    let compiled = sfota::assemble(PROGRAM);

    let mut commands = prepare_program(&compiled);
    let pins = commands.iter_mut().try_fold(pins, |p, c| p.execute(c))?;

    let pins = pins.execute(&mut read)?;

    if buf[0] != 0x9C {
        panic!("Got {} but expected 0x9C", buf[0]);
    }

    let mut jsr = Command::JSR { address: 0x0300 };

    let pins = pins.execute(&mut jsr)?.execute(&mut jsr)?;

    let mut read = Command::ReadData {
        address: 0x0300,
        out_buffer: MutableLengthLimitedSlice::new(&mut buf)?,
    };
    let pins = pins.execute(&mut read)?;

    if buf[0] != 0x00 {
        panic!("Got {} but expected 0x00", buf[0]);
    }

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
