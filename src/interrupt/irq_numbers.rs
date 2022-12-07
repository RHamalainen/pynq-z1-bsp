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

#[derive(Clone, Copy)]
pub enum Irq {
    IrqSgi0,
    IrqSgi1,
    IrqSgi2,
    IrqSgi3,
    IrqSgi4,
    IrqSgi5,
    IrqSgi6,
    IrqSgi7,
    IrqSgi8,
    IrqSgi9,
    IrqSgi10,
    IrqSgi11,
    IrqSgi12,
    IrqSgi13,
    IrqSgi14,
    IrqSgi15,
    IrqGlobalTimer,
    IrqNFiq,
    IrqCpuPrivateTimer,
    IrqAwdt,
    IrqNIrq,
    IrqCpu0,
    IrqCpu1,
    IrqL2Cache,
    IrqOcm,
    IrqPmu0,
    IrqPmu1,
    IrqXadc,
    IrqDevC,
    IrqSwdt,
    IrqTtc00,
    IrqTtc01,
    IrqTtc02,
    IrqDmacAbort,
    IrqDmac0,
    IrqDmac1,
    IrqDmac2,
    IrqDmac3,
    IrqSmc,
    IrqQuadSpi,
    IrqGpio,
    IrqUsb0,
    IrqEthernet0,
    IrqEthernet0Wakeup,
    IrqSdio0,
    IrqI2c0,
    IrqSpi0,
    IrqUart0,
    IrqCan0,
    IrqPl0,
    IrqPl1,
    IrqPl2,
    IrqPl3,
    IrqPl4,
    IrqPl5,
    IrqPl6,
    IrqPl7,
    IrqTtc10,
    IrqTtc11,
    IrqTtc12,
    IrqDmac4,
    IrqDmac5,
    IrqDmac6,
    IrqDmac7,
    IrqUsb1,
    IrqEthernet1,
    IrqEthernet1Wakeup,
    IrqSdio1,
    IrqI2c1,
    IrqSpi1,
    IrqUart1,
    IrqCan1,
    IrqPl8,
    IrqPl9,
    IrqPl10,
    IrqPl11,
    IrqPl12,
    IrqPl13,
    IrqPl14,
    IrqPl15,
    IrqParity,
}

impl Irq {
    pub fn from_u32(value: u32) -> Self {
        match value {
            0 => Irq::IrqSgi0,
            1 => Irq::IrqSgi1,
            2 => Irq::IrqSgi2,
            3 => Irq::IrqSgi3,
            4 => Irq::IrqSgi4,
            5 => Irq::IrqSgi5,
            6 => Irq::IrqSgi6,
            7 => Irq::IrqSgi7,
            8 => Irq::IrqSgi8,
            9 => Irq::IrqSgi9,
            10 => Irq::IrqSgi10,
            11 => Irq::IrqSgi11,
            12 => Irq::IrqSgi12,
            13 => Irq::IrqSgi13,
            14 => Irq::IrqSgi14,
            15 => Irq::IrqSgi15,
            27 => Irq::IrqGlobalTimer,
            28 => Irq::IrqNFiq,
            29 => Irq::IrqCpuPrivateTimer,
            30 => Irq::IrqAwdt,
            31 => Irq::IrqNIrq,
            32 => Irq::IrqCpu0,
            33 => Irq::IrqCpu1,
            34 => Irq::IrqL2Cache,
            35 => Irq::IrqOcm,
            37 => Irq::IrqPmu0,
            38 => Irq::IrqPmu1,
            39 => Irq::IrqXadc,
            40 => Irq::IrqDevC,
            41 => Irq::IrqSwdt,
            42 => Irq::IrqTtc00,
            43 => Irq::IrqTtc01,
            44 => Irq::IrqTtc02,
            45 => Irq::IrqDmacAbort,
            46 => Irq::IrqDmac0,
            47 => Irq::IrqDmac1,
            48 => Irq::IrqDmac2,
            49 => Irq::IrqDmac3,
            50 => Irq::IrqSmc,
            51 => Irq::IrqQuadSpi,
            52 => Irq::IrqGpio,
            53 => Irq::IrqUsb0,
            54 => Irq::IrqEthernet0,
            55 => Irq::IrqEthernet0Wakeup,
            56 => Irq::IrqSdio0,
            57 => Irq::IrqI2c0,
            58 => Irq::IrqSpi0,
            59 => Irq::IrqUart0,
            60 => Irq::IrqCan0,
            61 => Irq::IrqPl0,
            62 => Irq::IrqPl1,
            63 => Irq::IrqPl2,
            64 => Irq::IrqPl3,
            65 => Irq::IrqPl4,
            66 => Irq::IrqPl5,
            67 => Irq::IrqPl6,
            68 => Irq::IrqPl7,
            69 => Irq::IrqTtc10,
            70 => Irq::IrqTtc11,
            71 => Irq::IrqTtc12,
            72 => Irq::IrqDmac4,
            73 => Irq::IrqDmac5,
            74 => Irq::IrqDmac6,
            75 => Irq::IrqDmac7,
            76 => Irq::IrqUsb1,
            77 => Irq::IrqEthernet1,
            78 => Irq::IrqEthernet1Wakeup,
            79 => Irq::IrqSdio1,
            80 => Irq::IrqI2c1,
            81 => Irq::IrqSpi1,
            82 => Irq::IrqUart1,
            83 => Irq::IrqCan1,
            84 => Irq::IrqPl8,
            85 => Irq::IrqPl9,
            86 => Irq::IrqPl10,
            87 => Irq::IrqPl11,
            88 => Irq::IrqPl12,
            89 => Irq::IrqPl13,
            90 => Irq::IrqPl14,
            91 => Irq::IrqPl15,
            92 => Irq::IrqParity,
            unknown => panic!("Unknown IRQ number: {}", unknown),
        }
    }
}
