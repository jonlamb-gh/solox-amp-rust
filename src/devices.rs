// TODO - construct a basic SVD file
#![allow(unused)]

use sel4_sys::{seL4_PageBits, seL4_Word};
//use core::ptr;

// physical memory regions (addressed from the A9)

// DDR region we're using - should be first untyped
pub const DDR_PADDR: seL4_Word = 0x8000_0000;
pub const DDR_SIZE: usize = 1 << 16;

// M4 TCM(L) - 8 pages, 32 KB
pub const M4_TCM_PADDR: seL4_Word = 0x007F_8000;
pub const M4_TCM_SIZE: usize = 8 * (1 << seL4_PageBits as usize);
pub const M4_TCM_SIZE_BITS: usize = 15;

// SRC, first page for SRC_SCR
pub const SRC_PADDR: seL4_Word = 0x020D_8000;
pub const SRC_SIZE: usize = 1 << seL4_PageBits as usize;
pub const SRC_SIZE_BITS: usize = seL4_PageBits as usize;
pub const SRC_SCR_OFFSET: seL4_Word = 0;

// CCM, first page for CCGR3
pub const CCM_PADDR: seL4_Word = 0x020C_4000;
pub const CCM_SIZE: usize = 1 << seL4_PageBits as usize;
pub const CCM_SIZE_BITS: usize = seL4_PageBits as usize;
pub const CCM_CCGR3_OFFSET: seL4_Word = 0x74;

pub fn mut_u32_ptr(paddr: seL4_Word) -> *mut u32 {
    paddr as *mut u32
}
