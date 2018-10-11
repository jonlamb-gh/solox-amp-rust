//! TODO
//! - handle PMU (54/127) interrupt and others

#![no_std]
#![no_main]
#![feature(core_intrinsics)]

extern crate bare_metal;
extern crate cortex_m;
extern crate cortex_m_rt as rt;

use core::cell::Cell;
use core::intrinsics;
use core::panic::PanicInfo;
use cortex_m::interrupt::Mutex;
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

unsafe impl bare_metal::Nr for Interrupt {
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

/// TODO
const TEST_RESULTS_FAIL: u32 = 0xDEADDEAD;
const TEST_RESULTS_PASS: u32 = 0xBEEFBEEF;
static TEST_RESULTS_WORD: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

fn get_test_results() -> u32 {
    cortex_m::interrupt::free(|cs| {
        TEST_RESULTS_WORD.borrow(cs).get()
    })
}

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;
    let mut nvic = p.NVIC;
    let mut scb = p.SCB;

    // configure the system tick timer to wrap every 1 millisecond
    // M4 is set to 227 MHz by default
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(227000000 / 1000);
    syst.enable_counter();

    // configure the UART2 tx/rx mux/pins
    init_gpio_pad(MX6SLX_PAD_GPIO1_IO06_UART2_TX);
    init_gpio_pad(MX6SLX_PAD_GPIO1_IO07_UART2_RX);

    init_uart();

    serial_println!("\nM4 core is up and running\n");

    delay_ms(&mut syst, 1000);

    // enable PMU interrupts
    nvic.enable(Interrupt::PMU_REG);
    nvic.enable(Interrupt::PMU_CORE);

    // start the test
    serial_println!("starting the test");

    // TODO - use IRQ priority and a timer to stop test?
    syst.disable_counter();
    syst.clear_current();
    //syst.set_reload(227000000 / 10000); // 10 ms
    syst.set_reload(227000000 / 1000); // 1 ms
    syst.enable_interrupt();
    syst.enable_counter();

    // raise PMU regulator failure interrupt
    nvic.set_pending(Interrupt::PMU_REG);

    while syst.has_wrapped() == false {}
    syst.disable_counter();
    syst.disable_interrupt();
    // end of test

    let results = get_test_results();

    serial_print!("test results: ");
    if results == TEST_RESULTS_PASS {
        serial_println!("PASS");
    } else if results == TEST_RESULTS_FAIL {
        serial_println!("FAIL");
    } else {
        serial_println!("UNKNOWN FAILURE");
    }

    // loop forever, wait for interrupts
    scb.set_sleepdeep();
    loop {
        cortex_m::asm::wfi()
    }

    // should not get here
}

fn handle_pmu_failure_irq() {
    // TODO - clean shutdown

    cortex_m::interrupt::free(|cs| {
        let results = TEST_RESULTS_WORD.borrow(cs).get();

        if results != TEST_RESULTS_FAIL {
            TEST_RESULTS_WORD.borrow(cs).set(TEST_RESULTS_PASS);
        }

    });
}

#[exception]
fn SysTick() {
    cortex_m::interrupt::free(|cs| {
        let results = TEST_RESULTS_WORD.borrow(cs).get();

        if results != TEST_RESULTS_PASS {
            TEST_RESULTS_WORD.borrow(cs).set(TEST_RESULTS_FAIL);
        }
    });
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

/// We don't have a device crate yet, so we broker the
/// IRQ from the default handler.
#[exception]
fn DefaultHandler(irqn: i16) {
    use bare_metal::Nr;

    // irqn will be positive when the handler is servicing
    // a device specific exception (interrupt)
    let irq_was_serviced = if irqn > 0 {
        if irqn == Interrupt::PMU_REG.nr() as i16 || irqn == Interrupt::PMU_CORE.nr() as i16 {
            handle_pmu_failure_irq();
            true
        } else {
            false
        }
    } else {
        false
    };

    if !irq_was_serviced {
        panic!("Unhandled exception (IRQn = {})", irqn);
    }
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
