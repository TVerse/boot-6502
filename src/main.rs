use boot_6502::*;

use lib_io::*;
use lib_io_rpi::*;

fn main() -> Result<()> {
    let pins = initialize()?;
    run(pins).map(|_| ())?;
    println!("Done!");
    Ok(())
}

fn run<WH: WithHandshake, S: SendByte, D: DelayMs>(pins: Pins<WH, S, D>) -> Result<Pins<WH, S, D>> {
    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("S... ".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;

    let mut commands = prepare_program();
    commands.truncate(59);
    assert!(commands.len() == 59);
    let pins = commands.iter_mut().try_fold(pins, |p, c| {
        println!("{:X}", c.address().unwrap());
        let res = p.execute(c);
        println!("done");
        res
    })?;

    // let mut set_ready = Command::WriteData {
    //     address: 0x3FF2,
    //     data: LengthLimitedSlice::new(&[1])?,
    // };

    // let pins = pins.execute(&mut set_ready)?;

    // println!("Set ready done");

    let mut jsr = Command::JSR { address: 0x0300 };

    let pins = pins.execute(&mut jsr)?;

    println!("Running");

    Delay.delay_ms(2000);

    let mut buf = [1; 1];
    let mut read_data = Command::ReadData {
        address: 0x0300,
        out_buffer: MutableLengthLimitedSlice::new(&mut buf)?,
    };

    let pins = pins.execute(&mut read_data)?;

    println!("Read");

    dbg!(buf[0]);
    assert!(buf[0] == 0xAA);

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
