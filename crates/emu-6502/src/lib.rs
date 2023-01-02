mod sys;

use std::marker::PhantomData;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::abort;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AccessKind {
    Read,
    Write,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AccessLog {
    kind: AccessKind,
    address: u16,
    byte: u8,
}

pub trait Addressable {
    fn read_byte(&mut self, address: u16) -> Option<u8>;
    fn write_byte(&mut self, address: u16, byte: u8) -> Option<()>;
}

pub struct AddressSpace<'a> {
    addressables: Vec<&'a mut dyn Addressable>,
    log: Vec<AccessLog>,
}

impl<'a> AddressSpace<'a> {
    pub fn new(addressables: Vec<&'a mut dyn Addressable>) -> Self {
        Self {
            addressables,
            log: Vec::with_capacity(100),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let byte = self
            .addressables
            .iter_mut()
            .find_map(|a| a.read_byte(address))
            .unwrap_or(0xFF);

        self.log.push(AccessLog {
            kind: AccessKind::Read,
            address,
            byte,
        });

        byte
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        if self
            .addressables
            .iter_mut()
            .find_map(|a| a.write_byte(address, byte))
            .is_some()
        {
            self.log.push(AccessLog {
                kind: AccessKind::Write,
                address,
                byte,
            })
        }
    }
}

#[no_mangle]
extern "C" fn fake6502_mem_read(context: *mut sys::fake6502_context, address: u16) -> u8 {
    unsafe {
        let mem = &mut *((*context).state_host as *mut AddressSpace);
        // TODO can't panic over FFI boundary
        match catch_unwind(AssertUnwindSafe(|| mem.read_byte(address))) {
            Ok(byte) => byte,
            Err(_) => {
                println!("Didn't read byte at address {address}, aborting process");
                abort()
            }
        }
    }
}

#[no_mangle]
extern "C" fn fake6502_mem_write(context: *mut sys::fake6502_context, address: u16, val: u8) {
    unsafe {
        let mem = &mut *((*context).state_host as *mut AddressSpace);
        // TODO can't panic over FFI boundary
        match catch_unwind(AssertUnwindSafe(|| mem.write_byte(address, val))) {
            Ok(()) => (),
            Err(_) => {
                println!("Didn't write byte {val} at address {address}, aborting process");
                abort()
            }
        }
    }
}

#[derive(Debug)]
pub struct Fake6502<'a> {
    context: sys::fake6502_context,
    _phantomdata: PhantomData<&'a mut AddressSpace<'a>>,
}

impl<'a> Fake6502<'a> {
    pub fn new(address_space: &'a mut AddressSpace) -> Self {
        // Reversed in drop.
        let context = sys::fake6502_context {
            cpu: Default::default(),
            emu: Default::default(),
            state_host: address_space as *mut AddressSpace as *mut _,
        };
        Self {
            context,
            _phantomdata: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            sys::fake6502_reset(&mut self.context as *mut _);
        }
    }

    pub fn step(&mut self) {
        unsafe {
            sys::fake6502_step(&mut self.context as *mut _);
        }
    }

    pub fn address_space(&mut self) -> &mut AddressSpace {
        unsafe { &mut *(self.context.state_host as *mut AddressSpace) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rom {
        rom: Vec<u8>,
    }

    impl Addressable for Rom {
        fn read_byte(&mut self, address: u16) -> Option<u8> {
            if address as usize >= 0x8000 {
                self.rom.get((address as usize) - 0x8000).copied()
            } else {
                None
            }
        }

        fn write_byte(&mut self, _address: u16, _byte: u8) -> Option<()> {
            None
        }
    }

    struct Ram {
        ram: Vec<u8>,
    }

    impl Addressable for Ram {
        fn read_byte(&mut self, address: u16) -> Option<u8> {
            self.ram.get(address as usize).copied()
        }

        fn write_byte(&mut self, address: u16, byte: u8) -> Option<()> {
            if let Some(b) = self.ram.get_mut(address as usize) {
                *b = byte;
                Some(())
            } else {
                None
            }
        }
    }

    #[test]
    fn reset_and_write_byte() {
        let mut rom = [0x55; 0x8000];
        // Reset vector
        rom[0x7FFC] = 0x00;
        rom[0x7FFD] = 0x80;
        // Set to-be-written address to known value
        rom[0x0001] = 0x00;
        // LDA #EE
        rom[0x0000] = 0xA9;
        rom[0x0001] = 0xEE;
        // STA #01
        rom[0x0002] = 0x85;
        rom[0x0003] = 0x01;
        let mut rom = Rom { rom: rom.to_vec() };
        let ram = [0x55; 0x4000];
        let mut ram = Ram { ram: ram.to_vec() };
        let addressables: Vec<&mut dyn Addressable> = vec![&mut rom, &mut ram];
        let mut address_space = AddressSpace::new(addressables);
        let mut fake_6502 = Fake6502::new(&mut address_space);
        fake_6502.reset();
        fake_6502.step();
        fake_6502.step();
        let b = fake_6502.address_space().read_byte(0x0001);
        assert_eq!(b, 0xEE);
        let expected = AccessLog {
            kind: AccessKind::Read,
            address: 0x0001,
            byte: 0xEE,
        };
        assert_eq!(fake_6502.address_space().log.last().unwrap(), &expected)
    }
}
