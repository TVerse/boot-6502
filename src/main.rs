#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

use arduino_mega2560::prelude::*;
use atmega2560_hal::port;
use avr_hal_generic::void::ResultVoidExt;

use boot_6502::serial;
use boot_6502::done;
use boot_6502::serial_println;
use lib_io::impl_avr::*;
use lib_io::*;

static mut PANIC_LED: MaybeUninit<port::porta::PA1<port::mode::Output>> = MaybeUninit::uninit();

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
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
    serial_println!("PROGRAM len: {}", PROGRAM.len());

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

    let with_handshake = Handshake {
        incoming_handshake: ca2,
        outgoing_handshake: ca1,
        delay: arduino_mega2560::Delay::new(),
    };
    let write = Write {
        ddr: &pins.ddr,
        p0: pa0,
        p1: pa1,
        p2: pa2,
        p3: pa3,
        p4: pa4,
        p5: pa5,
        p6: pa6,
        p7: pa7,
    };

    let pins= Pins{
        with_handshake,
        send_byte: write,
    };

    match run(pins) {
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

static PROGRAM: &[u8] = include_bytes!("../6502/selfcontained_test.bin");

static PROGRAM_WRONG_SIZE: &str = "Program is the wrong size";

fn run<WH: WithHandshake, S: SendByte>(pins: Pins<WH, S>) -> Result<Pins<WH, impl SendByte>> {
    if PROGRAM.len() != 0x3F00 - 0x0300 {
        return Err(PROGRAM_WRONG_SIZE);
    };
    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new("S... ".as_bytes())?,
    };
    let pins = pins.execute(&mut display_string)?;

    // let mut program_buf = [0; 256];
    // let mut pins = pins;
    //
    // for load_page in 0x00..0x3C {
    //     for (i, b) in program_buf.iter_mut().enumerate() {
    //         *b = PROGRAM[load_page + i];
    //     }
    //     let mut write_data = Command::WriteData {
    //         address: (load_page as u16) + 0x0300,
    //         data: LengthLimitedSlice::new(&program_buf)?,
    //     };
    //     pins = pins.execute(&mut write_data)?;
    // }
    //
    // let mut set_ready = Command::WriteData {
    //     address: 0x3FF2,
    //     data: LengthLimitedSlice::new(&[1])?,
    // };
    //
    // let pins = pins.execute(&mut set_ready)?;

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
