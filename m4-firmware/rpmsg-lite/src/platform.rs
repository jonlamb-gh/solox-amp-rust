// https://github.com/NXPmicro/rpmsg-lite/blob/master/lib/rpmsg_lite/porting/platform/imx6sx_m4/rpmsg_platform.c

use bare_metal::Nr;
use cortex_m::peripheral::scb::VectActive;
use cortex_m::{interrupt, peripheral};

use mu;

//#[derive(Copy, Clone, Debug)]
pub struct Platform<'a> {
    // not sure this counter mechanism is really needed?
    //disable_counter: u32,
    nvic: &'a mut peripheral::NVIC,
}

/*
impl Default for Platform {
    fn default() -> Self {
        Self {
            //disable_counter: 0
        }
    }
}
*/

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
        Platform { nvic }
    }

    pub fn init(&mut self) {
        // prepare for the MU interrupt
        // MU must be initialized before rpmsg init is called
        //mu::MU_B::init();

        //#define APP_MU_IRQ_PRIORITY (3)
        // NVIC_SetPriority(BOARD_MU_IRQ_NUM, APP_MU_IRQ_PRIORITY);

        self.nvic.enable(Interrupt::MU_M4);
    }

    //pub fn time_delay()

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

    //pub fn vatop()
    //pub fn patova()
}
