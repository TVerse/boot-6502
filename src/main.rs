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
        serial_println!("Got {} but expected 0", buf[0]);
        return Err("!");
    }

    let mut jsr = Command::JSR { address: 0xE000 };

    let pins = pins.execute(&mut jsr)?.execute(&mut jsr)?;

    let mut read = Command::ReadData {
        address: 0x0300,
        out_buffer: MutableLengthLimitedSlice::new(&mut buf)?,
    };
    let pins = pins.execute(&mut read)?;

    if buf[0] != 255 {
        serial_println!("Got {} but expected 255", buf[0]);
        return Err("!");
    }

    let mut display_string = Command::DisplayString {
        data: LengthLimitedSlice::new(" Done!".as_bytes())?,
    };

    pins.execute(&mut display_string)
}
