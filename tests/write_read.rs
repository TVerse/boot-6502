use boot_6502::initialize;

use lib_io::*;

#[test]
fn test_write_read() -> Result<()> {
    let pins = initialize()?;
    run(pins).map(|_| ())
}

fn run<WH: WithHandshake, S: SendByte, D: DelayMs>(pins: Pins<WH, S, D>) -> Result<Pins<WH, S, D>> {
    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("Starting.".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;

    let addresses = (0x200u16..0x3d00).step_by(0xED);
    let mut data: [u8; 256] = [0; 256];
    for (i, d) in data.iter_mut().enumerate() {
        *d = i as u8;
    }
    let mut misses: usize = 0;
    let mut pins = pins;
    for address in addresses {
        println!("Address: {:#X}", address);
        let sizes = (1..257).step_by(27);
        for size in sizes {
            let input_data = &data[0..size];
            let mut write_command = Command::WriteData {
                data: LengthLimitedSlice::new(input_data)?,
                address,
            };
            let mut buf = [0; 256];
            let output_buf = &mut buf[0..size];
            let mut read_command = Command::ReadData {
                out_buffer: MutableLengthLimitedSlice::new(output_buf)?,
                address,
            };
            pins = pins.execute(&mut write_command)?;
            pins = pins.execute(&mut read_command)?;
            for (i, (written, read)) in input_data.iter().zip(output_buf.iter()).enumerate() {
                if *written != *read {
                    println!(
                        "Got a miss at base address {}, byte {}: wrote {}, read {}",
                        address, i, written, read
                    );
                    misses += 1;
                }
            }
        }
    }

    if misses != 0 {
        panic!("Had {} misses", misses);
    } else {
        println!("No misses!");
    }

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
