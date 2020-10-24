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

    ufmt::uwriteln!(&mut serial, "\n\u{04}").void_unwrap();
    loop {
        delay.delay_ms(10000u16);
    }
}
