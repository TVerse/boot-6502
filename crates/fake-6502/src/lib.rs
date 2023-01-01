#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::marker::PhantomData;
use std::mem;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::abort;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for fake6502_cpu_state {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for fake6502_emu_state {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

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

pub struct AddressSpace {
    ram: [u8; 0x4000],
    rom: [u8; 0x8000],
    log: Vec<AccessLog>,
}

impl AddressSpace {
    pub fn new(rom: [u8; 0x8000]) -> Self {
        Self {
            ram: [0; 0x4000],
            rom,
            log: Vec::with_capacity(100),
        }
    }

    pub fn new_boxed(rom: [u8; 0x8000]) -> Box<Self> {
        Box::new(Self::new(rom))
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let byte = match address {
            0x0000..=0x3FFF => self.ram[address as usize],
            0x8000..=0xFFFF => self.rom[(address - 0x8000) as usize],
            _ => 0xFF,
        };

        self.log.push(AccessLog {
            kind: AccessKind::Read,
            address,
            byte,
        });

        byte
    }

    pub fn write_byte(&mut self, address: u16, byte: u8) {
        let r = match address {
            0x0000..=0x3FFF => &mut self.ram[address as usize],
            _ => return,
        };

        self.log.push(AccessLog {
            kind: AccessKind::Write,
            address,
            byte,
        });

        *r = byte;
    }
}

pub struct Fake6502<'a> {
    context: fake6502_context,
    _phantomdata: PhantomData<&'a mut AddressSpace>,
}

impl<'a> Fake6502<'a> {
    pub fn new(address_space: Box<AddressSpace>) -> Self {
        // Reversed in drop.
        let address_space = Box::leak(address_space);
        let context = fake6502_context {
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
            fake6502_reset(&mut self.context as *mut _);
        }
    }

    pub fn step(&mut self) {
        unsafe {
            fake6502_step(&mut self.context as *mut _);
        }
    }

    pub fn address_space(&mut self) -> &mut AddressSpace {
        unsafe { &mut *(self.context.state_host as *mut AddressSpace) }
    }
}

impl<'a> Drop for Fake6502<'a> {
    fn drop(&mut self) {
        // Grab the address space, turn it back into a box, and let drop handle it.
        let address_space = self.context.state_host as *mut AddressSpace;
        let _ = unsafe { Box::from_raw(address_space) };
    }
}

#[no_mangle]
extern "C" fn fake6502_mem_read(context: *mut fake6502_context, address: u16) -> u8 {
    unsafe {
        let mem = &mut *((*context).state_host as *mut AddressSpace);
        // TODO can't panic over FFI boundary
        match catch_unwind(AssertUnwindSafe(|| mem.read_byte(address))) {
            Ok(byte) => byte,
            Err(_) => abort(),
        }
    }
}

#[no_mangle]
extern "C" fn fake6502_mem_write(context: *mut fake6502_context, address: u16, val: u8) {
    unsafe {
        let mem = &mut *((*context).state_host as *mut AddressSpace);
        // TODO can't panic over FFI boundary
        match catch_unwind(AssertUnwindSafe(|| mem.write_byte(address, val))) {
            Ok(()) => (),
            Err(_) => abort(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{AccessKind, AccessLog, AddressSpace, Fake6502};

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
        let address_space = AddressSpace::new_boxed(rom);
        let mut fake_6502 = Fake6502::new(address_space);
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
