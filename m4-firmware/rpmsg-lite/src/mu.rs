//! Messaging unit

#![allow(non_camel_case_types, non_upper_case_globals)]

// TODO
// - macro for generating A and B peripherals
// - use an svd file
// MU B 0x4229_C000

use core::marker::PhantomData;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum RxRegister {
    R0,
    R1,
    R2,
    R3,
}

pub struct MU_B {
    _marker: PhantomData<*const ()>,
}

impl MU_B {
    fn ptr() -> *const RegisterBlock {
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

    pub fn is_rx_full(&self, msg_register: RxRegister) -> bool {
        let rb = unsafe { &*Self::ptr() };
        rb.sr.register.get() & (MU_SR_RF0_MASK >> u32::from(msg_register)) != 0
    }

    // use nb::Error::WouldBlock?
    pub fn try_recv_msg(&self, msg_register: RxRegister) -> Result<u32, ()> {
        if self.is_rx_full(msg_register) {
            let rb = unsafe { &*Self::ptr() };
            Ok(rb.rr(msg_register))
        } else {
            Err(())
        }
    }
}

impl From<RxRegister> for u32 {
    fn from(r: RxRegister) -> u32 {
        match r {
            RxRegister::R0 => 0,
            RxRegister::R1 => 1,
            RxRegister::R2 => 2,
            RxRegister::R3 => 3,
        }
    }
}

pub const NUM_RX_REGISTERS: u32 = 4;
pub const NUM_TX_REGISTERS: u32 = 4;

pub const MU_SR_RF0_MASK: u32 = 1 << 27;

pub const MU_CR_GIEn_MASK: u32 = 0xF000_0000;
pub const MU_CR_RIEn_MASK: u32 = 0x0F00_0000;
pub const MU_CR_TIEn_MASK: u32 = 0x00F0_0000;
pub const MU_CR_GIRn_MASK: u32 = 0x000F_0000;
pub const MU_CR_Fn_MASK: u32 = 0x000_0007;

#[repr(C)]
struct RegisterBlock {
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

impl RegisterBlock {
    fn rr(&self, msg_register: RxRegister) -> u32 {
        match msg_register {
            RxRegister::R0 => self.rr0.register.get(),
            RxRegister::R1 => self.rr1.register.get(),
            RxRegister::R2 => self.rr2.register.get(),
            RxRegister::R3 => self.rr3.register.get(),
        }
    }
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
