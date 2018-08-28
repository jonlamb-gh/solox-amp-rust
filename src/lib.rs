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

// thread stack size in bytes or u64's?
const THREAD_STACK_SIZE: usize = 4096;
static mut THREAD_STACK: *const [u64; THREAD_STACK_SIZE] = &[0; THREAD_STACK_SIZE];

pub fn init(allocator: &mut Allocator, global_fault_ep_cap: seL4_CPtr) {
    debug_println!("feL4 app init");

    let tcb_obj = allocator.vka_alloc_tcb().unwrap();
    let tcb_cap = tcb_obj.cptr;

    let pd_cap = seL4_CapInitThreadVSpace;
    let cspace_cap = seL4_CapInitThreadCNode;

    // create a IPC buffer and capability for it
    let mut ipc_frame_cap: seL4_CPtr = 0;
    let ipc_buffer_vaddr = allocator
        .vspace_new_ipc_buffer(Some(&mut ipc_frame_cap))
        .unwrap();

    // set the IPC buffer's virtual address in a field of the IPC buffer
    let ipc_buffer: *mut seL4_IPCBuffer = ipc_buffer_vaddr as _;
    unsafe { (*ipc_buffer).userData = ipc_buffer_vaddr };

    // allocate a cspace slot for the fault endpoint
    let fault_ep_cap = allocator.vka_cspace_alloc().unwrap();

    // create a badged fault endpoint for the thread
    let err: seL4_Error = unsafe {
        seL4_CNode_Mint(
            cspace_cap,
            fault_ep_cap,
            seL4_WordBits as _,
            cspace_cap,
            global_fault_ep_cap,
            seL4_WordBits as _,
            seL4_CapRights_new(1, 1, 1),
            FAULT_EP_BADGE,
        )
    };
    assert!(err == 0, "Failed to mint a copy of the fault endpoint");

    // TODO - finish up
    let tcb_err: seL4_Error = unsafe {
        seL4_TCB_Configure(
            tcb_cap,
            fault_ep_cap,
            cspace_cap.into(),
            seL4_NilData.into(),
            pd_cap.into(),
            seL4_NilData.into(),
            ipc_buffer_vaddr,
            ipc_frame_cap,
        )
    };

    assert!(tcb_err == 0, "Failed to configure TCB");

    let stack_alignment_requirement: usize = (seL4_WordBits as usize / 8) * 2;

    assert!(THREAD_STACK_SIZE >= 512, "Thread stack size is too small");
    assert!(
        THREAD_STACK_SIZE % stack_alignment_requirement == 0,
        "Thread stack is not properly aligned to a {} byte boundary",
        stack_alignment_requirement
    );

    let stack_base = unsafe { THREAD_STACK as usize };
    let stack_top = stack_base + THREAD_STACK_SIZE;

    assert!(
        stack_top % stack_alignment_requirement == 0,
        "Thread stack is not properly aligned to a {} byte boundary",
        stack_alignment_requirement
    );

    let mut regs: seL4_UserContext = unsafe { mem::zeroed() };

    // only 2 registers
    regs.pc = thread_run as seL4_Word;
    regs.sp = stack_top as seL4_Word;

    let err: u32 = unsafe { seL4_TCB_WriteRegisters(tcb_cap, 0, 0, 2, &mut regs) };
    assert!(err == 0, "Failed to write TCB registers");

    let err: u32 = unsafe { seL4_TCB_SetPriority(tcb_cap, seL4_CapInitThreadTCB.into(), 255) };
    assert!(err == 0, "Failed to set TCB priority");

    let err: u32 = unsafe { seL4_TCB_Resume(tcb_cap) };
    assert!(err == 0, "Failed to start thread");
}

pub fn handle_fault(badge: seL4_Word) {
    debug_println!("!!! Fault from badge 0x{:X}", badge);
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
