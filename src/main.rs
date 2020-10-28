use boot_6502::*;

use lib_io::*;

fn main() -> Result<()> {
    let pins = initialize()?;
    run(pins).map(|_| ())
}

fn run<WH: WithHandshake, S: SendByte, D: DelayMs>(pins: Pins<WH, S, D>) -> Result<Pins<WH, S, D>> {
    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("S... ".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;

    let mut commands = prepare_program();
    let pins = commands.iter_mut().try_fold(pins, |p, c|p.execute(c))?;

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
