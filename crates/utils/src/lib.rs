mod vasm;

pub use vasm::compile;

use emu_6502::{Component, InterruptPending};

struct Rom {
    rom: [u8; 0x8000],
}

impl Component for Rom {
    fn read_byte(&mut self, address: u16) -> Option<u8> {
        (address >= 0x8000).then(self.rom[address - 0x8000])
    }

    fn write_byte(&mut self, _address: u16, _byte: u8) -> Option<()> {
        None
    }
}

struct Ram {
    ram: [u8; 0x4000],
}

impl Component for Ram {
    fn read_byte(&mut self, address: u16) -> Option<u8> {
        (address < 0x4000).then(self.ram[address])
    }

    fn write_byte(&mut self, address: u16, byte: u8) -> Option<()> {
        (address < 0x4000).then(self.ram[address] = byte)
    }
}

#[derive(Default)]
struct VIA {
    rb: u8,
    ra: u8,
    ddrb: u8,
    ddra: u8,
    t1_counter: u16,
    t1_latch: u16,
    t2c_l: u8,
    t2c_h: u8,
    sr: u8,
    acr: u8,
    pcr: u8,
    ifr: u8,
    ier: u8,
    ra_no_handshake: u8,
}

impl VIA {
    fn set_ifr(&mut self, bit: u8) {
        self.ifr |= 1 << bit
    }

    fn reset_ifr(&mut self, bit: u8) {
        self.ifr &= !(1 << bit)
    }

    fn t1_zero(&mut self) -> Option<InterruptPending> {
        self.set_ifr(6);
        if self.acr & 0b01000000 > 0 {
            self.t1_counter = self.t1_latch
        }
        ((self.ier | self.ifr) > 0).then(InterruptPending::IRQ)
    }
}

impl Component for VIA {
    fn read_byte(&mut self, address: u16) -> Option<u8> {
        match address {
            0x6000 => Some(self.rb),
            0x6001 => Some(self.ra),
            0x6002 => Some(self.ddrb),
            0x6003 => Some(self.ddra),
            0x6004 => {
                self.reset_ifr(6);
                Some(self.t1c_l)
            }
            0x6005 => Some(self.t1c_h),
            0x6006 => Some(self.t1l_l),
            0x6007 => Some(self.t1l_h),
            0x6008 => Some(self.t2c_l),
            0x6009 => Some(self.t2c_h),
            0x600A => Some(self.sr),
            0x600B => Some(self.acr),
            0x600C => Some(self.pcr),
            0x600D => Some(self.ifr),
            0x600E => Some(self.ier),
            0x600F => Some(self.ra_no_handshake),
            _ => None
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) -> Option<()> {
        match address {
            0x6000 => Some(self.rb = byte),
            0x6001 => Some(self.ra = byte),
            0x6002 => Some(self.ddrb = byte),
            0x6003 => Some(self.ddra = byte),
            0x6004 => {
                Some(self.t1l_l = byte)
            }
            0x6005 => {
                self.t1l_h = byte;
                self.t1c_l = self.t1l_l;
                self.t1c_h = self.t1l_h;
                self.reset_ifr(6);
                Some(())
            }
            0x6006 => Some(self.t1l_l = byte),
            0x6007 => {
                self.reset_ifr(6);
                Some(self.t1l_h = byte)
            }
            0x6008 => Some(self.t2c_l = byte),
            0x6009 => Some(self.t2c_h = byte),
            0x600A => Some(self.sr = byte),
            0x600B => Some(self.acr = byte),
            0x600C => Some(self.pcr = byte),
            0x600D => Some(self.ifr = byte),
            0x600E => Some(self.ier = byte),
            0x600F => Some(self.ra_no_handshake = byte),
            _ => None
        }
    }

    fn tick(&mut self, cycles: usize) -> Option<InterruptPending> {
        let t1 = (self.t1c_l as u16) | ((self.t1c_h as u16) << 8);
        if t1 as usize > cycles {

        }

        None
    }
}
