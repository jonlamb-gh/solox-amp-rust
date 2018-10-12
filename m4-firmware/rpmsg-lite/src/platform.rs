#[derive(Copy, Clone, Debug)]
pub struct Platform {
    //
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            //
        }
    }
}

/// Interrupts from the M4's (core B) perspective
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// MU interrupt to the A9 core, 90
    MU_A9,
    /// MU interrupt to the M4 core, 99
    MU_M4,
}

unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::MU_A9 => 90,
            Interrupt::MU_M4 => 99,
        }
    }
}

impl Platform {
    pub fn init(&mut self) {
        // prepare for the MU interrupt
        // MU must be initialized before rpmsg init is called


        //#define APP_MU_IRQ_PRIORITY (3)
        //let p = cortex_m::Peripherals::take().unwrap();
        // let mut nvic = p.NVIC;
        // NVIC_SetPriority(BOARD_MU_IRQ_NUM, APP_MU_IRQ_PRIORITY);
        //nvic.enable(Interrupt::MU_M4);
    }
}
