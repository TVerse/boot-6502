#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

use arduino_mega2560::prelude::*;
use atmega2560_hal::port;
use avr_hal_generic::void::ResultVoidExt;

use boot_6502::*;

static mut PANIC_LED: MaybeUninit<port::porta::PA1<port::mode::Output>> = MaybeUninit::uninit();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let led = unsafe { &mut *PANIC_LED.as_mut_ptr() };
    let mut delay = arduino_mega2560::Delay::new();
    loop {
        led.toggle().void_unwrap();
        delay.delay_ms(500u16);
    }
}

// TODO figure out why you get one extra interrupt when this is restarted
#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();
    let mut delay = arduino_mega2560::Delay::new();
    let pins = arduino_mega2560::Pins::new(
        dp.PORTA, dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF, dp.PORTG, dp.PORTH, dp.PORTJ,
        dp.PORTK, dp.PORTL,
    );

    unsafe {
        PANIC_LED = MaybeUninit::new(pins.d23.into_output(&pins.ddr));
    };

    let mut serial =
        arduino_mega2560::Serial::new(dp.USART0, pins.d0, pins.d1.into_output(&pins.ddr), 57600);

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

    let pins = Pins::new(
        &pins.ddr,
        &mut serial,
        ca2,
        ca1,
        pa0,
        pa1,
        pa2,
        pa3,
        pa4,
        pa5,
        pa6,
        pa7,
    );

    match execute(pins) {
        Ok(_) => loop {
            delay.delay_ms(10000u16);
        },
        Err(e) => {
            ufmt::uwriteln!(&mut serial, "ERROR: {}", e).void_unwrap();
            panic!(e);
        }
    }
}

fn execute(mut pins: Pins) -> Result<()> {
    ufmt::uwriteln!(&mut pins.serial, "Waiting for start...").void_unwrap();

    pins.delay.delay_ms(2000u16); // TODO can get a signal somehow?

    let s = "Hello world!";

    let send_len = Length::new(s.len())?;

    let mut buffer = [0; 256];

    s.bytes().zip(buffer.iter_mut()).for_each(|(b, ptr)| *ptr = b);

    let mut pins = pins.execute(Some(send_len), None, &mut buffer)?;

    ufmt::uwriteln!(&mut pins.serial, "Done!").void_unwrap();

    Ok(())
}
