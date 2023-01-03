mod sys;

use std::marker::PhantomData;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InterruptPending {
    IRQ,
    NMI,
}

pub trait Component {
    fn read_byte(&mut self, address: u16) -> Option<u8>;
    fn write_byte(&mut self, address: u16, byte: u8) -> Option<()>;
    fn tick(&mut self, _cycles: usize) -> Option<InterruptPending> {
        None
    }
}

pub struct Components {
    components: Vec<Arc<Mutex<dyn Component>>>,
    log: Vec<AccessLog>,
}

impl Components {
    pub fn new(components: Vec<Arc<Mutex<dyn Component>>>) -> Self {
        Self {
            components,
            log: Vec::with_capacity(100),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let byte = self
            .components
            .iter_mut()
            .find_map(|a| a.lock().unwrap().read_byte(address))
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
            .components
            .iter_mut()
            .find_map(|a| a.lock().unwrap().write_byte(address, byte))
            .is_some()
        {
            self.log.push(AccessLog {
                kind: AccessKind::Write,
                address,
                byte,
            })
        }
    }

    pub fn tick(&mut self, cycles: usize) -> Option<InterruptPending> {
        let interrupts: Vec<_> = self
            .components
            .iter_mut()
            .map(|c| c.lock().unwrap().tick(cycles))
            .filter_map(|x| x)
            .collect();
        if interrupts.is_empty() {
            None
        } else {
            if interrupts.contains(&InterruptPending::NMI) {
                Some(InterruptPending::NMI)
            } else {
                Some(InterruptPending::IRQ)
            }
        }
    }
}

#[no_mangle]
extern "C" fn fake6502_mem_read(context: *mut sys::fake6502_context, address: u16) -> u8 {
    unsafe {
        let mem = &mut *((*context).state_host as *mut Components);
        // can't panic over FFI boundary
        match catch_unwind(AssertUnwindSafe(|| mem.read_byte(address))) {
            Ok(byte) => byte,
            Err(_) => {
                println!("Didn't read byte at address {address}, returning 0xFF");
                0xFF
            }
        }
    }
}

#[no_mangle]
extern "C" fn fake6502_mem_write(context: *mut sys::fake6502_context, address: u16, val: u8) {
    unsafe {
        let mem = &mut *((*context).state_host as *mut Components);
        // can't panic over FFI boundary
        match catch_unwind(AssertUnwindSafe(|| mem.write_byte(address, val))) {
            Ok(()) => (),
            Err(_) => {
                println!("Didn't write byte {val} at address {address}");
            }
        }
    }
}

#[derive(Debug)]
pub struct Fake6502 {
    context: sys::fake6502_context,
    _phantomdata: PhantomData<Box<Components>>,
}

impl Fake6502 {
    pub fn new(components: Components) -> Self {
        let components = Box::new(components);
        let components = Box::leak(components);
        let context = sys::fake6502_context {
            cpu: Default::default(),
            emu: Default::default(),
            state_host: components as *mut Components as *mut _,
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

    pub fn step(&mut self) -> Option<InterruptPending> {
        let cycles_before = self.context.emu.clockticks;
        unsafe {
            sys::fake6502_step(&mut self.context as *mut _);
        }
        let cycles_taken = self.context.emu.clockticks - cycles_before;
        let cycles_taken = cycles_taken as usize;
        self.address_space().tick(cycles_taken)
    }

    pub fn start_interrupt_routine(
        &mut self,
        interrupt_pending: InterruptPending,
    ) -> Option<InterruptPending> {
        let cycles_before = self.context.emu.clockticks;
        match interrupt_pending {
            InterruptPending::IRQ => unsafe { sys::fake6502_irq(&mut self.context as *mut _) },
            InterruptPending::NMI => unsafe { sys::fake6502_nmi(&mut self.context as *mut _) },
        };
        let cycles_taken = self.context.emu.clockticks - cycles_before;
        let cycles_taken = cycles_taken as usize;
        self.address_space().tick(cycles_taken)
    }

    pub fn address_space(&mut self) -> &mut Components {
        unsafe { &mut *(self.context.state_host as *mut Components) }
    }
}

impl Drop for Fake6502 {
    fn drop(&mut self) {
        let components = self.context.state_host as *mut Components;
        let _ = unsafe {Box::from_raw(components)};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Rom {
        rom: Vec<u8>,
    }

    impl Component for Rom {
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

    impl Component for Ram {
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
        let rom = Rom { rom: rom.to_vec() };
        let ram = [0x55; 0x4000];
        let ram = Ram { ram: ram.to_vec() };
        let addressables: Vec<Arc<Mutex<dyn Component>>> = vec![Arc::new(Mutex::new(rom)), Arc::new(Mutex::new(ram))];
        let address_space = Components::new(addressables);
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
