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

use core::ptr;
use devices::*;

// CPIO archive in our ELF file
#[link(name = "m4archive")]
#[no_mangle]
extern "C" {
    // TODO - is there a proper way to do this?
    static _cpio_archive: u8;
    static _binary_archive_cpio_size: u32;
}

pub fn run() {
    debug_println!("\nhello from a feL4 thread!\n");

    // construct CPIO pointers, symbols are from our ELF file
    let cpio_archive: *const u8 = unsafe { &_cpio_archive };
    let cpio_archive_size: usize = unsafe { &_binary_archive_cpio_size as *const u32 } as usize;

    let cpio_reader = cpio::Reader::new(cpio_archive, cpio_archive_size);

    debug_println!("created new CPIO reader\n{:#?}", cpio_reader);

    // get first CPIO entry, should be our M4 binary file
    let m4_bin_fw_cpio_file = cpio_reader.parse_entry();

    debug_println!("parsed CPIO entry\n{:#?}", m4_bin_fw_cpio_file);

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
