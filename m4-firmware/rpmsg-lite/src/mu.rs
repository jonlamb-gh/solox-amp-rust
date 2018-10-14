//! Messaging unit

#![allow(non_camel_case_types, non_upper_case_globals)]

// TODO
// - macro for generating A and B peripherals
// - use an svd file
// - inlining functions that are just register manipulations

use core::marker::PhantomData;
use void::Void;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MsgRegister {
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

    pub fn send_msg(&self, msg_register: MsgRegister, msg: u32) -> nb::Result<(), Void> {
        if self.is_tx_empty(msg_register) {
            let rb = unsafe { &*Self::ptr() };
            Ok(rb.tr(msg_register, msg))
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn recv_msg(&self, msg_register: MsgRegister) -> nb::Result<u32, Void> {
        if self.is_rx_full(msg_register) {
            let rb = unsafe { &*Self::ptr() };
            Ok(rb.rr(msg_register))
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn is_rx_full(&self, msg_register: MsgRegister) -> bool {
        let rb = unsafe { &*Self::ptr() };
        rb.sr.register.get() & (MU_SR_RF0_MASK >> u32::from(msg_register)) != 0
    }

    pub fn is_tx_empty(&self, msg_register: MsgRegister) -> bool {
        let rb = unsafe { &*Self::ptr() };
        rb.sr.register.get() & (MU_SR_TE0_MASK >> u32::from(msg_register)) != 0
    }

    pub fn is_general_int_accepted(&self, msg_register: MsgRegister) -> bool {
        let rb = unsafe { &*Self::ptr() };
        rb.sr.register.get() & (MU_CR_GIR0_MASK >> u32::from(msg_register)) == 0
    }

    /// Trigger a general purpose interrupt to the other core
    ///
    /// Returns Ok if the interrupt was triggered
    /// and nb::Error::WouldBlock if the interrupt is currently pending
    pub fn trigger_general_int(&self, msg_register: MsgRegister) -> nb::Result<(), Void> {
        if self.is_general_int_accepted(msg_register) {
            // all interrupts have been accepted, trigger now
            let rb = unsafe { &*Self::ptr() };
            let cr = rb.cr.register.get();
            rb.cr.register.set(
                // clear GIRn
                (cr & !MU_CR_GIRn_MASK)
                // set GIRn
                | (MU_CR_GIR0_MASK >> u32::from(msg_register)),
            );
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn is_flag_pending(&self) -> bool {
        let rb = unsafe { &*Self::ptr() };
        rb.sr.register.get() & MU_SR_FUP_MASK != 0
    }

    /// Try to set some of the 3-bit flags
    pub fn set_flags(&self, flags: u32) -> nb::Result<(), Void> {
        if self.is_flag_pending() {
            let rb = unsafe { &*Self::ptr() };
            let cr = rb.cr.register.get();
            rb.cr
                .register
                .set((cr & !(MU_CR_GIRn_MASK | MU_CR_Fn_MASK)) | flags);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /// Enable TX empty interrupt
    pub fn enable_tx_empty_int(&self, msg_register: MsgRegister) {
        let rb = unsafe { &*Self::ptr() };
        let cr = rb.cr.register.get();
        rb.cr.register.set(
            // clear GIRn
            (cr & !MU_CR_GIRn_MASK)
            // set TIEn
            | (MU_CR_TIE0_MASK >> u32::from(msg_register)),
        );
    }

    /// Disable TX empty interrupt
    pub fn disable_tx_empty_int(&self, msg_register: MsgRegister) {
        let rb = unsafe { &*Self::ptr() };
        let cr = rb.cr.register.get();
        rb.cr.register.set(
            cr
            // clear GIRn, TIEn
            & !(MU_CR_GIRn_MASK | (MU_CR_TIE0_MASK >> u32::from(msg_register))),
        );
    }
}

impl From<MsgRegister> for u32 {
    fn from(r: MsgRegister) -> u32 {
        match r {
            MsgRegister::R0 => 0,
            MsgRegister::R1 => 1,
            MsgRegister::R2 => 2,
            MsgRegister::R3 => 3,
        }
    }
}

pub const NUM_RX_REGISTERS: u32 = 4;
pub const NUM_TX_REGISTERS: u32 = 4;

pub const MU_SR_RF0_MASK: u32 = 1 << 27;
pub const MU_SR_TE0_MASK: u32 = 1 << 23;
pub const MU_SR_FUP_MASK: u32 = 0x0000_0100;

pub const MU_CR_GIR0_MASK: u32 = 1 << 19;
pub const MU_CR_TIE0_MASK: u32 = 1 << 23;
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
    fn rr(&self, msg_register: MsgRegister) -> u32 {
        match msg_register {
            MsgRegister::R0 => self.rr0.register.get(),
            MsgRegister::R1 => self.rr1.register.get(),
            MsgRegister::R2 => self.rr2.register.get(),
            MsgRegister::R3 => self.rr3.register.get(),
        }
    }

    fn tr(&self, msg_register: MsgRegister, msg: u32) {
        match msg_register {
            MsgRegister::R0 => self.tr0.register.set(msg),
            MsgRegister::R1 => self.tr1.register.set(msg),
            MsgRegister::R2 => self.tr2.register.set(msg),
            MsgRegister::R3 => self.tr3.register.set(msg),
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
