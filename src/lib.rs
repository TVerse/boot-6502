use gpio_cdev::Chip;
use lib_io::*;
use lib_io_rpi::pins::*;
use lib_io_rpi::*;

pub fn initialize() -> Result<Pins<Handshake, Write, Delay>> {
    let mut delay = Delay;
    let mut chip = Chip::new("/dev/gpiochip0").map_err(|e| IoError::Other(Box::new(e)))?;
    let c = &mut chip;

    let reset = 13;
    let mut reset = Reset::new(c, reset)?;

    let ca1 = 26;
    let mut outgoing_handshake = OutgoingHandshake::new(c, ca1)?;
    let ca2 = 19;
    let incoming_handshake = IncomingHandshake::new(c, ca2)?;
    let pa0 = 21;
    let p0 = P0::new(c, pa0)?;
    let pa1 = 20;
    let p1 = P1::new(c, pa1)?;
    let pa2 = 6;
    let p2 = P2::new(c, pa2)?;
    let pa3 = 5;
    let p3 = P3::new(c, pa3)?;
    let pa4 = 22;
    let p4 = P4::new(c, pa4)?;
    let pa5 = 27;
    let p5 = P5::new(c, pa5)?;
    let pa6 = 17;
    let p6 = P6::new(c, pa6)?;
    let pa7 = 4;
    let p7 = P7::new(c, pa7)?;

    outgoing_handshake.set_high()?;

    println!("Resetting...");
    reset.set_low()?;
    delay.delay_us(5);
    reset.set_high()?;

    println!("Waiting...");
    delay.delay_ms(2000);
    while incoming_handshake.is_low()? {}

    let with_handshake = Handshake {
        incoming_handshake,
        outgoing_handshake,
        delay,
    };
    let write = Write {
        p0,
        p1,
        p2,
        p3,
        p4,
        p5,
        p6,
        p7,
    };

    Ok(Pins {
        with_handshake,
        send_byte: write,
        delay,
    })
}
