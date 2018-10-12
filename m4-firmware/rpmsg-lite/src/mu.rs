//! Messaging unit
//!

#![allow(non_camel_case_types, non_upper_case_globals)]

// TODO
// - macro for generating A and B peripherals
// - use an svd file
// MU B 0x4229_C000

use core::marker::PhantomData;

pub struct MU_B {
    _marker: PhantomData<*const ()>,
}

impl MU_B {
    pub fn ptr() -> *const RegisterBlock {
        0x4229_C000 as *const _
    }

    pub fn init() {
        let rb = unsafe { &*Self::ptr() };
        // Clear GIEn, RIEn, TIEn, GIRn and ABFn
        let bits = rb.cr.register.get();
        rb.cr.register.set(
            bits & !(MU_CR_GIEn_MASK
                | MU_CR_RIEn_MASK
                | MU_CR_TIEn_MASK
                | MU_CR_GIRn_MASK
                | MU_CR_Fn_MASK),
        );
    }
}

pub const MU_CR_GIEn_MASK: u32 = 0xF000_0000;
pub const MU_CR_RIEn_MASK: u32 = 0x0F00_0000;
pub const MU_CR_TIEn_MASK: u32 = 0x00F0_0000;
pub const MU_CR_GIRn_MASK: u32 = 0x000F_0000;
pub const MU_CR_Fn_MASK: u32 = 0x000_0007;

#[repr(C)]
pub struct RegisterBlock {
    pub tr0: TR,
    pub tr1: TR,
    pub tr2: TR,
    pub tr3: TR,
    pub rr0: RR,
    pub rr1: RR,
    pub rr2: RR,
    pub rr3: RR,
    pub sr: SR,
    pub cr: CR,
}

pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}

pub struct RR {
    register: ::vcell::VolatileCell<u32>,
}

// reset == 0x00F0_0080
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}

pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
