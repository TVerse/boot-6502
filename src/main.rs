#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use core::panic::PanicInfo;

use arduino_mega2560::prelude::*;
use atmega2560_hal::port;
use avr_hal_generic::void::ResultVoidExt;

use boot_6502::*;

static mut PANIC_LED: MaybeUninit<port::portb::PB1<port::mode::Output>> = MaybeUninit::uninit();

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
        PANIC_LED = MaybeUninit::new(pins.d52.into_output(&pins.ddr));
    };

    let mut serial =
        arduino_mega2560::Serial::new(dp.USART0, pins.d0, pins.d1.into_output(&pins.ddr), 57600);

    let mut reset = pins.d22.into_output(&pins.ddr);

    let mut ca1 = pins.d51.into_output(&pins.ddr);
    let ca2 = pins.d53;
    let mut pa0 = pins.d43.into_output(&pins.ddr);
    let mut pa1 = pins.d41.into_output(&pins.ddr);
    let mut pa2 = pins.d39.into_output(&pins.ddr);
    let mut pa3 = pins.d37.into_output(&pins.ddr);
    let mut pa4 = pins.d35.into_output(&pins.ddr);
    let mut pa5 = pins.d33.into_output(&pins.ddr);
    let mut pa6 = pins.d31.into_output(&pins.ddr);
    let mut pa7 = pins.d29.into_output(&pins.ddr);

    ca1.set_high().void_unwrap();

    reset.set_low().void_unwrap();
    delay.delay_us(5u8);
    reset.set_high().void_unwrap();

    ufmt::uwriteln!(&mut serial, "Waiting for start...").void_unwrap();

    delay.delay_ms(2000u16); // TODO can get a signal somehow?

    let mut handshake_pins = HandshakePins::new(&ca2, &mut ca1);

    let mut send_data_pins = SendDataPins::new(
        &mut pa0, &mut pa1, &mut pa2, &mut pa3, &mut pa4, &mut pa5, &mut pa6, &mut pa7,
    );

    ufmt::uwriteln!(&mut serial, "Sending!").void_unwrap();

    let mut send_data = SendData::new(&mut handshake_pins, &mut send_data_pins, &mut serial);

    let s = "Hello world!";

    let command = Command::DisplayString { string: s };

    send_data.send(command);

    ufmt::uwriteln!(&mut serial, "Done!").void_unwrap();

    loop {
        delay.delay_ms(10000u16);
    }
}
