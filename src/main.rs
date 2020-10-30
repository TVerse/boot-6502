use boot_6502::*;

use lib_io::{
    Command, DelayMs, LengthLimitedSlice, MutableLengthLimitedSlice, Pins, SendByte, WithHandshake,
};

use std::fs;
use std::process;

use anyhow::anyhow;
use anyhow::Result;

fn main() -> Result<()> {
    let pins = initialize()?;
    run(pins).map(|_| ())?;
    println!("Done!");
    Ok(())
}

fn run<WH: WithHandshake, S: SendByte, D: DelayMs>(pins: Pins<WH, S, D>) -> Result<Pins<WH, S, D>> {
    let program_name = "selfcontained_test";
    let tmpdir = tempfile::tempdir()?;
    println!("Copy source to {:#?}", tmpdir.path());
    fs::copy(
        format!("/home/pi/projects/boot-6502/6502/{}.s", program_name),
        tmpdir.path().join(format!("{}.s", program_name)),
    )?;
    for dir in fs::read_dir(tmpdir.path())? {
        dbg!(dir?);
    }
    println!("Compile");
    if !process::Command::new("/home/pi/compilers/vasm6502_oldstyle")
        .args(&["-wdc02", "-Fbin", "-dotdir"])
        .arg(tmpdir.path().join(format!("{}.s", program_name)))
        .arg("-o")
        .arg(tmpdir.path().join(format!("{}.bin", program_name)))
        .current_dir(tmpdir.path())
        .status()?
        .success()
    {
        return Err(anyhow!("Got a nonzero status from compiler"));
    }
    println!("Reading program");
    let program = fs::read(tmpdir.path().join(format!("{}.bin", program_name)))?;
    tmpdir.close()?;
    assert!(program.len() > 0);

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("S... ".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;

    let mut commands = prepare_program(&program);
    let pins = commands.iter_mut().try_fold(pins, |p, c| {
        println!("{:#04X}", c.address().unwrap());
        let res = p.execute(c);
        println!("done");
        res
    })?;
    println!("Jumping");
    let mut jsr = Command::JSR { address: 0x0301 };
    let pins = pins.execute(&mut jsr)?;

    let mut buf = [1; 1];
    let mut read_data = Command::ReadData {
        address: 0x0300,
        out_buffer: MutableLengthLimitedSlice::new(&mut buf)?,
    };

    let pins = pins.execute(&mut read_data)?;

    println!("Read");

    dbg!(buf[0]);
    assert_eq!(buf[0], 0xAA);

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    let pins = pins.execute(&mut display_string)?;

    Ok(pins)
}
