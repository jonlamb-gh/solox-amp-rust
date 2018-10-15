//! WIP platfrom and bare metal environment layers merged into one

// TODO
// - turn this into generic traits and provide a default IMX6SX impl
// - verify the need/behavior of the ISR/init/deinit counters used in the C impl
// - MU_IRQ_PRIORITY
// - can I seperate the interdependency of platform and environment layers
// calling into each other?
// - vq_id type with methods to replace the C macros (ie RL_GET_Q_ID())

use bare_metal::Nr;
use cortex_m::peripheral::scb::VectActive;
use cortex_m::{interrupt, peripheral};

use mu;

/// RPMSG MU channel index/register
/// As Linux suggests, use MU channel 1 for as communication channel
pub const RPMSG_MU_CHANNEL: mu::MsgRegister = mu::MsgRegister::R1;

/// Linux requires the ALIGN to 0x1000(4KB) instead of 0x80
pub const VRING_ALIGN: u32 = 0x1000;

/// contains pool of descriptors and two circular buffers
pub const VRING_SIZE: u32 = 0x8000;

/// size of shared memory + 2*VRING size
pub const VRING_OVERHEAD: u32 = (2 * VRING_SIZE);

//#define RL_PLATFORM_IMX6SX_M4_LINK_ID (0)
//#define RL_PLATFORM_HIGHEST_LINK_ID (0)

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

// TODO - do something else with these, inline, etc
//#define RL_GET_LINK_ID(id) (((id)&0xFFFFFFFE) >> 1)
//#define RL_GET_Q_ID(id) ((id)&0x1)
pub fn get_vq_id(core_id: u32, queue_id: u32) -> u32 {
    (queue_id & 0x1) | ((core_id << 1) & 0xFFFFFFFE)
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

    pub fn notify(&self, vq_id: u32) {
        // TODO
    }

    /// Called by application ISR
    pub fn rpmsg_handler(&self) {
        if let Ok(msg) = self.mu.recv_msg(RPMSG_MU_CHANNEL) {
            // notification, channel is upper 16 bits
            self.env_isr(msg >> 16)
        }
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
    //

    //---------
    // bm env stuff begins here
    // --------

    /// Called by application ISR
    fn env_isr(&self, vector: u32) {
        // TODO
    }
}
