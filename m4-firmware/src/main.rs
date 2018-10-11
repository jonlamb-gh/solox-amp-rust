//! TODO
//! - handle PMU (54/127) interrupt and others

#![no_std]
#![no_main]
#![feature(core_intrinsics)]

extern crate bare_metal;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use core::intrinsics;
use core::panic::PanicInfo;
use cortex_m::peripheral::syst::SystClkSource;
use rt::{entry, exception, ExceptionFrame};

// see src/bsp.rs
mod bsp;
use bsp::*;

/// Interrupts for the M4 core
#[allow(non_camel_case_types)]
pub enum Interrupt {
    PMU_REG,  // 54
    PMU_CORE, // 127
}

unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::PMU_REG => 54,
            Interrupt::PMU_CORE => 127,
        }
    }
}

macro_rules! serial_print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        SerialOutputHandle.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

macro_rules! serial_println {
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
    let mut nvic = p.NVIC;

    // configure the system tick timer to wrap every 1 millisecond
    // M4 is set to 227 MHz by default
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(227000000 / 1000);
    syst.enable_counter();

    // configure the UART2 tx/rx mux/pins
    init_gpio_pad(MX6SLX_PAD_GPIO1_IO06_UART2_TX);
    init_gpio_pad(MX6SLX_PAD_GPIO1_IO07_UART2_RX);

    init_uart();

    // example of macro use vs putstr()
    //serial_println!("M4 core is up and running\n");
    putstr("\nM4 core is up and running\n\n");

    delay_ms(&mut syst, 1000);

    // enable PMU interrupts
    nvic.enable(Interrupt::PMU_REG);
    nvic.enable(Interrupt::PMU_CORE);

    let string_data: &'static str = "Hello world from Rust on Cortex-M4\n";

    // loop forever, sending characters
    loop {
        for c in string_data.chars() {
            putchar(c);
            delay_ms(&mut syst, 100);
        }

        nvic.set_pending(Interrupt::PMU_REG);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::free(|_cs| {
        init_uart();
        serial_println!("\n{}\n", info);
    });

    unsafe { intrinsics::abort() }
}

struct SerialOutputHandle;

impl ::core::fmt::Write for SerialOutputHandle {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for b in s.chars() {
            putchar(b);
        }
        Ok(())
    }
}
