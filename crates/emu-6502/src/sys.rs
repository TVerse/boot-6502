#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;
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
