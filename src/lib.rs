#![no_std]
#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(all(feature = "alloc"))]
#[macro_use]
extern crate alloc;

extern crate rlibc;
extern crate sel4_sys;
extern crate sel4twinkle_alloc;

#[cfg(all(feature = "test"))]
#[macro_use]
extern crate proptest;

#[cfg(feature = "test")]
pub mod fel4_test;

#[macro_use]
mod macros;
mod cpio;
mod devices;

use core::mem;
use core::ptr;
use devices::*;
use sel4_sys::*;
use sel4twinkle_alloc::Allocator;

// CPIO archive in our ELF file
#[link(name = "m4archive")]
#[no_mangle]
extern "C" {
    // TODO - is there a proper way to do this?
    static _cpio_archive: u8;
    static _binary_archive_cpio_size: u32;
}

const FAULT_EP_BADGE: seL4_Word = 0xBEEF;

const CHILD_STACK_SIZE: usize = 4096;
static mut CHILD_STACK: *const [u64; CHILD_STACK_SIZE] = &[0; CHILD_STACK_SIZE];

pub fn init(allocator: &mut Allocator, fault_ep_cap: seL4_CPtr) {
    debug_println!("feL4 app init");

    let tcb_obj = allocator.vka_alloc_tcb().unwrap();
    let tcb_cap = tcb_obj.cptr;

    let pd_cap = seL4_CapInitThreadVSpace;
    let cspace_cap = seL4_CapInitThreadCNode;

    let tcb_err: seL4_Error = unsafe {
        seL4_TCB_Configure(
            tcb_cap,
            seL4_CapNull.into(),
            cspace_cap.into(),
            seL4_NilData.into(),
            pd_cap.into(),
            seL4_NilData.into(),
            0,
            0,
        )
    };

    assert!(tcb_err == 0, "Failed to configure TCB");

    let stack_base = unsafe { CHILD_STACK as usize };
    let stack_top = stack_base + CHILD_STACK_SIZE;
    let mut regs: seL4_UserContext = unsafe { mem::zeroed() };
    regs.pc = thread_run as seL4_Word;
    regs.sp = stack_top as seL4_Word;

    let _: u32 = unsafe { seL4_TCB_WriteRegisters(tcb_cap, 0, 0, 2, &mut regs) };
    let _: u32 = unsafe { seL4_TCB_SetPriority(tcb_cap, seL4_CapInitThreadTCB.into(), 255) };
    let _: u32 = unsafe { seL4_TCB_Resume(tcb_cap) };
}

pub fn thread_run() {
    debug_println!("\nhello from a feL4 thread!\n");

    // construct CPIO pointers, symbols are from our ELF file
    let cpio_archive: *const u8 = unsafe { &_cpio_archive };
    let cpio_archive_size: usize = unsafe { &_binary_archive_cpio_size as *const u32 } as usize;

    let cpio_reader = cpio::Reader::new(cpio_archive, cpio_archive_size);

    debug_println!("created new CPIO reader\n{:#?}", cpio_reader);

    // get first CPIO entry, should be our M4 binary file
    let m4_bin_fw_cpio_file = cpio_reader.parse_entry();

    debug_println!(
        "parsed CPIO entry '{}'\n{:#?}",
        m4_bin_fw_cpio_file.file_name(),
        m4_bin_fw_cpio_file
    );

    // TODO - this will fault, need to map in the device frames to back the vaddr's

    // upload the M4 binary from the CPIO file and start the M4 core
    upload_and_run_m4_binary(
        &m4_bin_fw_cpio_file,
        mut_u32_ptr(SRC_VADDR + SRC_SCR_OFFSET),
        mut_u32_ptr(CCM_VADDR + CCM_CCGR3_OFFSET),
        mut_u32_ptr(M4_TCM_VADDR),
    );

    debug_println!("\nthread work all done, sitting on loop");

    loop {}
}

fn upload_and_run_m4_binary(
    cpio_file: &cpio::FileEntry,
    src_scr_ptr: *mut u32,
    ccm_ccgr3_ptr: *mut u32,
    m4_tcm_ptr: *mut u32,
) {
    debug_println!("enabling M4 core clock");

    // enable M4 clock
    unsafe { ptr::write_volatile(ccm_ccgr3_ptr, ptr::read_volatile(ccm_ccgr3_ptr) | (3 << 2)) };

    debug_println!("copying M4 binary to TCM");

    // copy the binary to the M4 memory region
    unsafe {
        let _ = rlibc::memcpy(
            m4_tcm_ptr as *mut u8,
            cpio_file.data_ptr(),
            cpio_file.file_size(),
        );
    }

    debug_println!("enabling and starting the M4 core");

    // enable M4 and assert soft reset
    unsafe {
        ptr::write_volatile(
            src_scr_ptr,
            ptr::read_volatile(src_scr_ptr) | (1 << 22) | (1 << 4),
        )
    };

    // release the reset, starting the M4
    unsafe { ptr::write_volatile(src_scr_ptr, ptr::read_volatile(src_scr_ptr) & !(1 << 4)) };

    debug_println!("waiting for SRC_SCR reset auto-clear (bit 3) to clear");

    // wait for self-clearing SW reset to clear
    unsafe { while (ptr::read_volatile(src_scr_ptr) & (1 << 3)) != 0 {} };
}
