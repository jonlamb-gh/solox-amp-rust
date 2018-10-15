// TODO
// - turn this into generic traits and provide a default IMX6SX impl
// - verify the need/behavior of the ISR/init/deinit counters used in the C impl
// - MU_IRQ_PRIORITY
// - can I seperate the interdependency of platform and environment layers calling into
// each other?

use bare_metal::Nr;
use cortex_m::peripheral::scb::VectActive;
use cortex_m::{interrupt, peripheral};

use mu;

pub const SYSTEM_CORE_CLOCK: u32 = 227_000_000;

//#[derive(Copy, Clone, Debug)]
pub struct Platform<'a> {
    nvic: &'a mut peripheral::NVIC,
    mu: mu::Mu<mu::MU_B>,
}

/// Interrupts from the M4's (core B) perspective
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// MU interrupt to the A9 core, 90
    MU_A9,
    /// MU interrupt to the M4 core, 99
    MU_M4,
}

unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::MU_A9 => 90,
            Interrupt::MU_M4 => 99,
        }
    }
}

// TODO - figure out some traits after initial port
impl<'a> Platform<'a> {
    pub fn new(nvic: &'a mut peripheral::NVIC) -> Self {
        Platform {
            nvic,
            mu: mu::Mu::new(mu::MU_B::new()),
        }
    }

    pub fn init(&mut self) {
        // prepare for the MU interrupt
        // MU must be initialized before rpmsg init is called
        self.mu.init();

        //#define APP_MU_IRQ_PRIORITY (3)
        // NVIC_SetPriority(BOARD_MU_IRQ_NUM, APP_MU_IRQ_PRIORITY);

        self.nvic.enable(Interrupt::MU_M4);
    }

    pub fn time_delay(&self, ms: u32) {
        // calculate the CPU loops to delay, each loop has 3 cycles
        let mut loop_val = SYSTEM_CORE_CLOCK / 3 / (1000 * ms);

        // there's some difference among toolchains, 3 or 4 cycles each loop
        while loop_val != 0 {
            cortex_m::asm::nop();
            loop_val -= 1;
        }
    }

    /// Return whether CPU is processing IRQ
    pub fn in_isr() -> bool {
        peripheral::SCB::vect_active() != VectActive::ThreadMode
    }

    pub fn interrupt_enable(&mut self, _vq_id: u8) {
        self.nvic.enable(Interrupt::MU_M4);
    }

    pub fn interrupt_disable(&mut self, _vq_id: u8) {
        // virtqueues use the same NVIC vector
        self.nvic.disable(Interrupt::MU_M4);
    }

    // memory mapping

    // cache ops

    // pub fn vatop()
    // pub fn patova()
}
