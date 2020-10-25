#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

use arduino_mega2560::prelude::*;
use atmega2560_hal::port;
use avr_hal_generic::void::ResultVoidExt;

use boot_6502::serial;
use boot_6502::*;

static mut PANIC_LED: MaybeUninit<port::porta::PA1<port::mode::Output>> = MaybeUninit::uninit();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };
    let mut delay = arduino_mega2560::Delay::new();
    serial_println!("Panic!");
    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500u16);
    }
}

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap_or_else(|| panic!());
    let mut delay = arduino_mega2560::Delay::new();
    let pins = arduino_mega2560::Pins::new(
        dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH, dp.PORTJ,
        dp.PORTK, dp.PORTL,
    );

    let serial =
        arduino_mega2560::Serial::new(dp.USART0, pins.d0, pins.d1.into_output(&pins.ddr), 57600);

    unsafe {
        PANIC_LED = MaybeUninit::new(pins.d23.into_output(&pins.ddr));
        serial::init(serial);
    };

    let mut reset = pins.d22.into_output(&pins.ddr);

    let mut ca1 = pins.d51.into_output(&pins.ddr);
    let ca2 = pins.d53;
    let pa0 = pins.d43.into_output(&pins.ddr);
    let pa1 = pins.d41.into_output(&pins.ddr);
    let pa2 = pins.d39.into_output(&pins.ddr);
    let pa3 = pins.d37.into_output(&pins.ddr);
    let pa4 = pins.d35.into_output(&pins.ddr);
    let pa5 = pins.d33.into_output(&pins.ddr);
    let pa6 = pins.d31.into_output(&pins.ddr);
    let pa7 = pins.d29.into_output(&pins.ddr);

    ca1.set_high().void_unwrap();

    reset.set_low().void_unwrap();
    delay.delay_us(5u8);
    reset.set_high().void_unwrap();

    serial_println!("Waiting for start...");

    while ca2.is_low().void_unwrap() {}

    let pins = Pins::new(&pins.ddr, ca2, ca1, pa0, pa1, pa2, pa3, pa4, pa5, pa6, pa7);

    match execute(pins) {
        Ok(_) => {
            serial_println!("Success!");
            done();
        }
        Err(e) => {
            serial_println!("ERROR: {}", e);
            panic!(e);
        }
    }
}

fn execute(pins: Pins) -> Result<Pins> {
    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("Starting.".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;

    let addresses = (0x200..0x3e00).step_by(0xED);
    let mut data: [u8; 256] = [0; 256];
    for (i, d) in data.iter_mut().enumerate() {
        *d = i as u8;
    }
    let mut misses: usize = 0;
    let mut pins = pins;
    for address in addresses {
        let sizes = (0x1..=0x256).step_by(23);
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
            pins = pins
                .execute(&mut write_command)?
                .execute(&mut read_command)?;
            for (i, (written, read)) in input_data.iter().zip(output_buf.iter()).enumerate() {
                if *written != *read {
                    serial_println!(
                        "Got a miss at base address {}, byte {}: wrote {}, read {}",
                        address,
                        i,
                        written,
                        read
                    );
                    misses += 1;
                }
            }
        }
    }

    if misses != 0 {
        serial_println!("Had {} misses", misses);
    } else {
        serial_println!("Success!");
    }

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
