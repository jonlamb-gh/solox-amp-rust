#![no_std]

#![cfg_attr(feature = "alloc", feature(alloc))]

#[cfg(all(feature = "alloc"))]
#[macro_use]
extern crate alloc;

extern crate sel4_sys;

#[macro_use]
mod macros;

#[cfg(all(feature = "test"))]
#[macro_use]
extern crate proptest;

#[cfg(feature = "test")]
pub mod fel4_test;

pub fn run() {
    debug_println!("\nhello from a feL4 thread!\n");
}
