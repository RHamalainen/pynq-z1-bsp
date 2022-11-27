//! Interrupt request numbers.

#![allow(unused)]
#![allow(clippy::missing_docs_in_private_items)]

/// Software generated interrupts.
pub mod sgi {
    pub const IRQ_SGI_0: u32 = 0;
    pub const IRQ_SGI_1: u32 = 1;
    pub const IRQ_SGI_2: u32 = 2;
    pub const IRQ_SGI_3: u32 = 3;
    pub const IRQ_SGI_4: u32 = 4;
    pub const IRQ_SGI_5: u32 = 5;
    pub const IRQ_SGI_6: u32 = 6;
    pub const IRQ_SGI_7: u32 = 7;
    pub const IRQ_SGI_8: u32 = 8;
    pub const IRQ_SGI_9: u32 = 9;
    pub const IRQ_SGI_10: u32 = 10;
    pub const IRQ_SGI_11: u32 = 11;
    pub const IRQ_SGI_12: u32 = 12;
    pub const IRQ_SGI_13: u32 = 13;
    pub const IRQ_SGI_14: u32 = 14;
    pub const IRQ_SGI_15: u32 = 15;
}

/// Private peripheral interrupts.
pub mod ppi {
    pub const IRQ_GLOBAL_TIMER: u32 = 27;
    pub const IRQ_N_FIQ: u32 = 28;
    pub const IRQ_CPU_PRIVATE_TIMER: u32 = 29;
    pub const IRQ_AWDT: u32 = 30;
    pub const IRQ_N_IRQ: u32 = 31;
}

/// Shared peripheral interrupts.
pub mod spi {
    pub const IRQ_CPU0: u32 = 32;
    pub const IRQ_CPU1: u32 = 33;
    pub const IRQ_L2_CACHE: u32 = 34;
    pub const IRQ_OCM: u32 = 35;
    pub const IRQ_PMU0: u32 = 37;
    pub const IRQ_PMU1: u32 = 38;
    pub const IRQ_XADC: u32 = 39;
    pub const IRQ_DEV_C: u32 = 40;
    pub const IRQ_SWDT: u32 = 41;
    pub const IRQ_TTC0_0: u32 = 42;
    pub const IRQ_TTC0_1: u32 = 43;
    pub const IRQ_TTC0_2: u32 = 44;
    pub const IRQ_DMAC_ABORT: u32 = 45;
    pub const IRQ_DMAC0: u32 = 46;
    pub const IRQ_DMAC1: u32 = 47;
    pub const IRQ_DMAC2: u32 = 48;
    pub const IRQ_DMAC3: u32 = 49;
    pub const IRQ_SMC: u32 = 50;
    pub const IRQ_QUAD_SPI: u32 = 51;
    pub const IRQ_GPIO: u32 = 52;
    pub const IRQ_USB0: u32 = 53;
    pub const IRQ_ETHERNET0: u32 = 54;
    pub const IRQ_ETHERNET0_WAKEUP: u32 = 55;
    pub const IRQ_SDIO0: u32 = 56;
    pub const IRQ_I2C0: u32 = 57;
    pub const IRQ_SPI0: u32 = 58;
    pub const IRQ_UART0: u32 = 59;
    pub const IRQ_CAN0: u32 = 60;
    pub const IRQ_PL0: u32 = 61;
    pub const IRQ_PL1: u32 = 62;
    pub const IRQ_PL2: u32 = 63;
    pub const IRQ_PL3: u32 = 64;
    pub const IRQ_PL4: u32 = 65;
    pub const IRQ_PL5: u32 = 66;
    pub const IRQ_PL6: u32 = 67;
    pub const IRQ_PL7: u32 = 68;
    pub const IRQ_TTC1_0: u32 = 69;
    pub const IRQ_TTC1_1: u32 = 70;
    pub const IRQ_TTC1_2: u32 = 71;
    pub const IRQ_DMAC4: u32 = 72;
    pub const IRQ_DMAC5: u32 = 73;
    pub const IRQ_DMAC6: u32 = 74;
    pub const IRQ_DMAC7: u32 = 75;
    pub const IRQ_USB1: u32 = 76;
    pub const IRQ_ETHERNET1: u32 = 77;
    pub const IRQ_ETHERNET1_WAKEUP: u32 = 78;
    pub const IRQ_SDIO1: u32 = 79;
    pub const IRQ_I2C1: u32 = 80;
    pub const IRQ_SPI1: u32 = 81;
    pub const IRQ_UART1: u32 = 82;
    pub const IRQ_CAN1: u32 = 83;
    pub const IRQ_PL8: u32 = 84;
    pub const IRQ_PL9: u32 = 85;
    pub const IRQ_PL10: u32 = 86;
    pub const IRQ_PL11: u32 = 87;
    pub const IRQ_PL12: u32 = 88;
    pub const IRQ_PL13: u32 = 89;
    pub const IRQ_PL14: u32 = 90;
    pub const IRQ_PL15: u32 = 91;
    pub const IRQ_PARITY: u32 = 92;
}
