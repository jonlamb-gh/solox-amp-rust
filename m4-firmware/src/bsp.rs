use core::ptr;

pub const UART2_BAUD: u32 = 115200;
pub const UART2_CLOCK_FREQ: u32 = 80000000;

// see 65.15 UART Memory Map/Register Definition in the TRM
pub const M4_OFFSET: u32 = 0x40000000;
pub const AIPS_TZ2_BASE_ADDR: u32 = (0x02100000 + M4_OFFSET);
pub const AIPS2_OFF_BASE_ADDR: u32 = (AIPS_TZ2_BASE_ADDR + 0x80000);
pub const UART2_IPS_BASE_ADDR: u32 = (AIPS2_OFF_BASE_ADDR + 0x68000);

pub const IOMUX_BASE_ADDR: u32 = 0x420E0000;
pub const IOMUX_NO_PAD_CTRL: u32 = (1 << 17);

pub const MX6SLX_PAD_GPIO1_IO06_UART2_TX: u64 =
    (0x002C << 0) | (0 << 36) | (0x0374 << 12) | ((1 << 17) << 41) | (0x0838 << 24) | (0 << 59);

pub const MX6SLX_PAD_GPIO1_IO07_UART2_RX: u64 =
    (0x0030 << 0) | (0 << 36) | (0x0378 << 12) | ((1 << 17) << 41) | (0x0838 << 24) | (1 << 59);

pub const UART2_UTXD: *mut u32 = (UART2_IPS_BASE_ADDR + 0x40) as *mut u32;
pub const UART2_UCR1: *mut u32 = (UART2_IPS_BASE_ADDR + 0x80) as *mut u32;
pub const UART2_UCR2: *mut u32 = (UART2_IPS_BASE_ADDR + 0x84) as *mut u32;
pub const UART2_UCR3: *mut u32 = (UART2_IPS_BASE_ADDR + 0x88) as *mut u32;
pub const UART2_UCR4: *mut u32 = (UART2_IPS_BASE_ADDR + 0x8C) as *mut u32;
pub const UART2_UFCR: *mut u32 = (UART2_IPS_BASE_ADDR + 0x90) as *mut u32;
pub const UART2_USR1: *mut u32 = (UART2_IPS_BASE_ADDR + 0x94) as *mut u32;
pub const UART2_USR2: *mut u32 = (UART2_IPS_BASE_ADDR + 0x98) as *mut u32;
pub const UART2_UESC: *mut u32 = (UART2_IPS_BASE_ADDR + 0x9C) as *mut u32;
pub const UART2_UTIM: *mut u32 = (UART2_IPS_BASE_ADDR + 0xA0) as *mut u32;
pub const UART2_UBIR: *mut u32 = (UART2_IPS_BASE_ADDR + 0xA4) as *mut u32;
pub const UART2_UBMR: *mut u32 = (UART2_IPS_BASE_ADDR + 0xA8) as *mut u32;
pub const UART2_ONEMS: *mut u32 = (UART2_IPS_BASE_ADDR + 0xB0) as *mut u32;
pub const UART2_UTS: *mut u32 = (UART2_IPS_BASE_ADDR + 0xB4) as *mut u32;

pub const UART_UTS_TXEMPTY: u32 = (1 << 6);
pub const UART_UCR1_UARTEN: u32 = (1 << 0);
pub const UART_UCR2_SRST: u32 = (1 << 0);
pub const UART_UFCR_RXTL_SHF: u32 = 0;
pub const UART_UFCR_TXTL_SHF: u32 = 10;
pub const UART_UFCR_RFDIV_2: u32 = (4 << 7);
pub const UART_UCR2_PREN: u32 = (1 << 8);
pub const UART_UCR2_WS: u32 = (1 << 5);
pub const UART_UCR2_IRTS: u32 = (1 << 14);
pub const UART_UCR2_STPB: u32 = (1 << 6);
pub const UART_UCR2_RXEN: u32 = (1 << 1);
pub const UART_UCR2_TXEN: u32 = (1 << 2);
pub const UART_USR2_ADET: u32 = (1 << 15);
pub const UART_USR2_IDLE: u32 = (1 << 12);
pub const UART_USR2_IRINT: u32 = (1 << 8);
pub const UART_USR2_WAKE: u32 = (1 << 7);
pub const UART_USR2_RTSF: u32 = (1 << 4);
pub const UART_USR2_BRCD: u32 = (1 << 2);
pub const UART_USR2_ORE: u32 = (1 << 1);
pub const UART_USR2_RDR: u32 = (1 << 0);
pub const UART_USR1_PARITYERR: u32 = (1 << 15);
pub const UART_USR1_RTSD: u32 = (1 << 12);
pub const UART_USR1_ESCF: u32 = (1 << 11);
pub const UART_USR1_FRAMERR: u32 = (1 << 10);
pub const UART_USR1_AIRINT: u32 = (1 << 5);
pub const UART_USR1_AWAKE: u32 = (1 << 4);
pub const UART_UTS_TXFULL: u32 = (1 << 4);

pub fn init_gpio_pad(pad: u64) {
    let mux_ctrl_ofs: u32 = ((pad & (0xFFF << 0)) >> 0) as u32;
    let mux_mode: u32 = ((pad & (0x1F << 36)) >> 36) as u32;
    let sel_input_ofs: u32 = ((pad & (0xFFF << 24)) >> 24) as u32;
    let sel_input: u32 = ((pad & (0xF << 59)) >> 59) as u32;
    let pad_ctrl_ofs: u32 = ((pad & (0xFFF << 12)) >> 12) as u32;
    let pad_ctrl: u32 = ((pad & (0x3FFFF << 41)) >> 41) as u32;

    unsafe {
        if mux_ctrl_ofs != 0 {
            ptr::write_volatile((IOMUX_BASE_ADDR + mux_ctrl_ofs) as *mut u32, mux_mode);
        }

        if sel_input_ofs != 0 {
            ptr::write_volatile((IOMUX_BASE_ADDR + sel_input_ofs) as *mut u32, sel_input);
        }

        if ((pad_ctrl & IOMUX_NO_PAD_CTRL) == 0) && (pad_ctrl_ofs != 0) {
            ptr::write_volatile((IOMUX_BASE_ADDR + pad_ctrl_ofs) as *mut u32, pad_ctrl);
        }
    }
}

pub fn init_uart() {
    unsafe {
        // wait for UART to finish transmitting
        while (ptr::read_volatile(UART2_UTS) & UART_UTS_TXEMPTY) == 0 {}

        // set to default POR state
        ptr::write_volatile(UART2_UCR1, 0x00000000);
        ptr::write_volatile(UART2_UCR2, 0x00000000);

        // disable UART
        ptr::write_volatile(
            UART2_UCR1,
            ptr::read_volatile(UART2_UCR1) & !UART_UCR1_UARTEN,
        );

        // wait for reset
        while (ptr::read_volatile(UART2_UCR2) & UART_UCR2_SRST) == 0 {}

        ptr::write_volatile(UART2_UCR3, 0x00000704);
        ptr::write_volatile(UART2_UCR4, 0x00008000);
        ptr::write_volatile(UART2_UFCR, 0x00000801);
        ptr::write_volatile(UART2_UESC, 0x0000002B);
        ptr::write_volatile(UART2_UTIM, 0x00000000);
        ptr::write_volatile(UART2_UBIR, 0x00000000);
        ptr::write_volatile(UART2_UBMR, 0x00000000);
        ptr::write_volatile(UART2_ONEMS, 0x00000000);
        ptr::write_volatile(UART2_UTS, 0x00000000);

        // configure the FIFOs
        ptr::write_volatile(
            UART2_UFCR,
            (1 << UART_UFCR_RXTL_SHF) | UART_UFCR_RFDIV_2 | (2 << UART_UFCR_TXTL_SHF),
        );

        // setup one ms timer
        ptr::write_volatile(UART2_ONEMS, (UART2_CLOCK_FREQ / 2) / 1000);

        // configure 8N1
        ptr::write_volatile(UART2_UCR2, ptr::read_volatile(UART2_UCR2) & !UART_UCR2_PREN);
        ptr::write_volatile(UART2_UCR2, ptr::read_volatile(UART2_UCR2) | UART_UCR2_WS);
        ptr::write_volatile(UART2_UCR2, ptr::read_volatile(UART2_UCR2) & !UART_UCR2_STPB);

        // ignore RTS
        ptr::write_volatile(UART2_UCR2, ptr::read_volatile(UART2_UCR2) | UART_UCR2_IRTS);

        // enable UART
        ptr::write_volatile(
            UART2_UCR1,
            ptr::read_volatile(UART2_UCR1) | UART_UCR1_UARTEN,
        );

        // enable FIFOs
        ptr::write_volatile(
            UART2_UCR2,
            ptr::read_volatile(UART2_UCR2) | UART_UCR2_SRST | UART_UCR2_RXEN | UART_UCR2_TXEN,
        );

        // clear USR2 status flags
        ptr::write_volatile(
            UART2_USR2,
            ptr::read_volatile(UART2_USR2)
                | UART_USR2_ADET
                | UART_USR2_IDLE
                | UART_USR2_IRINT
                | UART_USR2_WAKE
                | UART_USR2_RTSF
                | UART_USR2_BRCD
                | UART_USR2_ORE
                | UART_USR2_RDR,
        );

        // clear USR1 status flags
        ptr::write_volatile(
            UART2_USR1,
            ptr::read_volatile(UART2_USR1)
                | UART_USR1_PARITYERR
                | UART_USR1_RTSD
                | UART_USR1_ESCF
                | UART_USR1_FRAMERR
                | UART_USR1_AIRINT
                | UART_USR1_AWAKE,
        );

        // set the numerator value minus one of the BRM ratio
        ptr::write_volatile(UART2_UBIR, (UART2_BAUD / 100) - 1);

        // set the denominator value minus one of the BRM ratio
        ptr::write_volatile(UART2_UBMR, ((UART2_CLOCK_FREQ / 2) / 1600) - 1);
    }
}

pub fn putchar(data: char) {
    unsafe {
        // wait for Tx FIFO to be ready
        while (ptr::read_volatile(UART2_UTS) & UART_UTS_TXFULL) != 0 {}

        // add caraige return
        if data == '\n' {
            ptr::write_volatile(UART2_UTXD, '\r' as u32);

            while (ptr::read_volatile(UART2_UTS) & UART_UTS_TXFULL) != 0 {}
        }

        ptr::write_volatile(UART2_UTXD, data as u32);
    }
}

pub fn putstr(string: &'static str) {
    for c in string.chars() {
        putchar(c);
    }
}

pub fn delay_ms(syst: &mut cortex_m::peripheral::SYST, ms: u32) {
    syst.clear_current();

    for _ in 0..ms {
        while !syst.has_wrapped() {}
    }
}
