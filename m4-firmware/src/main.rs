//! TODO
//! - handle PMU interrupt and others

#![no_std]
#![no_main]

extern crate cortex_m;

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_abort;

use cortex_m::peripheral::syst::SystClkSource;
use rt::ExceptionFrame;

// see src/bsp.rs
mod bsp;
use bsp::*;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configure the system tick timer to wrap every 1 millisecond
    // M4 is set to 227 MHz by default
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(227000000 / 1000);
    syst.enable_counter();

    // configure the UART2 tx/rx mux/pins
    init_gpio_pad(MX6SLX_PAD_GPIO1_IO06_UART2_TX);
    init_gpio_pad(MX6SLX_PAD_GPIO1_IO07_UART2_RX);

    init_uart();

    putstr("\n\nM4 core is up and running\n\n");
    delay_ms(&mut syst, 1000);

    let string_data: &'static str = "Hello world from Rust on Cortex-M4\n";

    // loop forever, sending characters
    loop {
        for c in string_data.chars() {
            putchar(c);
            delay_ms(&mut syst, 100);
        }
    }
}

fn delay_ms(syst: &mut cortex_m::peripheral::SYST, ms: u32) {
    syst.clear_current();

    for _ in 0..ms {
        while !syst.has_wrapped() {}
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
