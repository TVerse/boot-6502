use boot_6502::*;

use lib_io::*;

static PROGRAM: &[u8] = include_bytes!("../6502/selfcontained_test.bin");

fn main() -> Result<()> {
    let pins = initialize()?;
    run(pins).map(|_| ())
}

fn run<WH: WithHandshake, S: SendByte, D: DelayMs>(pins: Pins<WH, S, D>) -> Result<Pins<WH, S, D>> {
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
