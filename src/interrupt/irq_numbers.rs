//! Interrupt request numbers.

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

/// Software generated interrupt.
#[derive(Clone, Copy)]
pub enum SgiIrq {
    Sgi0,
    Sgi1,
    Sgi2,
    Sgi3,
    Sgi4,
    Sgi5,
    Sgi6,
    Sgi7,
    Sgi8,
    Sgi9,
    Sgi10,
    Sgi11,
    Sgi12,
    Sgi13,
    Sgi14,
    Sgi15,
}

impl SgiIrq {
    pub fn from_u32(value: u32) -> Result<Self, u32> {
        let value = match value {
            sgi::IRQ_SGI_0 => Self::Sgi0,
            sgi::IRQ_SGI_1 => Self::Sgi1,
            sgi::IRQ_SGI_2 => Self::Sgi2,
            sgi::IRQ_SGI_3 => Self::Sgi3,
            sgi::IRQ_SGI_4 => Self::Sgi4,
            sgi::IRQ_SGI_5 => Self::Sgi5,
            sgi::IRQ_SGI_6 => Self::Sgi6,
            sgi::IRQ_SGI_7 => Self::Sgi7,
            sgi::IRQ_SGI_8 => Self::Sgi8,
            sgi::IRQ_SGI_9 => Self::Sgi9,
            sgi::IRQ_SGI_10 => Self::Sgi10,
            sgi::IRQ_SGI_11 => Self::Sgi11,
            sgi::IRQ_SGI_12 => Self::Sgi12,
            sgi::IRQ_SGI_13 => Self::Sgi13,
            sgi::IRQ_SGI_14 => Self::Sgi14,
            sgi::IRQ_SGI_15 => Self::Sgi15,
            unknown => {
                return Err(unknown);
            }
        };
        Ok(value)
    }

    pub fn as_u32(self) -> u32 {
        match self {
            Self::Sgi0 => sgi::IRQ_SGI_0,
            Self::Sgi1 => sgi::IRQ_SGI_1,
            Self::Sgi2 => sgi::IRQ_SGI_2,
            Self::Sgi3 => sgi::IRQ_SGI_3,
            Self::Sgi4 => sgi::IRQ_SGI_4,
            Self::Sgi5 => sgi::IRQ_SGI_5,
            Self::Sgi6 => sgi::IRQ_SGI_6,
            Self::Sgi7 => sgi::IRQ_SGI_7,
            Self::Sgi8 => sgi::IRQ_SGI_8,
            Self::Sgi9 => sgi::IRQ_SGI_9,
            Self::Sgi10 => sgi::IRQ_SGI_10,
            Self::Sgi11 => sgi::IRQ_SGI_11,
            Self::Sgi12 => sgi::IRQ_SGI_12,
            Self::Sgi13 => sgi::IRQ_SGI_13,
            Self::Sgi14 => sgi::IRQ_SGI_14,
            Self::Sgi15 => sgi::IRQ_SGI_15,
        }
    }
}

/// Private peripheral interrupts.
pub mod ppi {
    /// Global timer interrupt.
    pub const IRQ_GLOBAL_TIMER: u32 = 27;

    /// Legacy nFIQ interrupt which bypasses interrupt distributor.
    pub const IRQ_N_FIQ: u32 = 28;

    /// Private timer interrupt.
    pub const IRQ_CPU_PRIVATE_TIMER: u32 = 29;

    /// Watchdog timer interrupt.
    pub const IRQ_AWDT: u32 = 30;

    /// Legacy nIRQ interrupt which bypasses interrupt distributor.
    pub const IRQ_N_IRQ: u32 = 31;
}

/// Private peripheral interrupt.
#[derive(Clone, Copy)]
pub enum PpiIrq {
    GlobalTimer,
    NFiq,
    CpuPrivateTimer,
    Awdt,
    NIrq,
}

impl PpiIrq {
    pub fn from_u32(value: u32) -> Result<Self, u32> {
        let value = match value {
            ppi::IRQ_GLOBAL_TIMER => Self::GlobalTimer,
            ppi::IRQ_N_FIQ => Self::NFiq,
            ppi::IRQ_CPU_PRIVATE_TIMER => Self::CpuPrivateTimer,
            ppi::IRQ_AWDT => Self::Awdt,
            ppi::IRQ_N_IRQ => Self::NIrq,
            unknown => {
                return Err(unknown);
            }
        };
        Ok(value)
    }

    pub fn as_u32(self) -> u32 {
        match self {
            Self::GlobalTimer => ppi::IRQ_GLOBAL_TIMER,
            Self::NFiq => ppi::IRQ_N_FIQ,
            Self::CpuPrivateTimer => ppi::IRQ_CPU_PRIVATE_TIMER,
            Self::Awdt => ppi::IRQ_AWDT,
            Self::NIrq => ppi::IRQ_N_IRQ,
        }
    }
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

/// Shared peripheral interrupt.
#[derive(Clone, Copy)]
pub enum SpiIrq {
    Cpu0,
    Cpu1,
    L2Cache,
    Ocm,
    Pmu0,
    Pmu1,
    Xadc,
    DevC,
    Swdt,
    Ttc00,
    Ttc01,
    Ttc02,
    DmacAbort,
    Dmac0,
    Dmac1,
    Dmac2,
    Dmac3,
    Smc,
    QuadSpi,
    Gpio,
    Usb0,
    Ethernet0,
    Ethernet0Wakeup,
    Sdio0,
    I2c0,
    Spi0,
    Uart0,
    Can0,
    Pl0,
    Pl1,
    Pl2,
    Pl3,
    Pl4,
    Pl5,
    Pl6,
    Pl7,
    Ttc10,
    Ttc11,
    Ttc12,
    Dmac4,
    Dmac5,
    Dmac6,
    Dmac7,
    Usb1,
    Ethernet1,
    Ethernet1Wakeup,
    Sdio1,
    I2c1,
    Spi1,
    Uart1,
    Can1,
    Pl8,
    Pl9,
    Pl10,
    Pl11,
    Pl12,
    Pl13,
    Pl14,
    Pl15,
    Parity,
}

impl SpiIrq {
    pub fn from_u32(value: u32) -> Result<Self, u32> {
        let value = match value {
            spi::IRQ_CPU0 => Self::Cpu0,
            spi::IRQ_CPU1 => Self::Cpu1,
            spi::IRQ_L2_CACHE => Self::L2Cache,
            spi::IRQ_OCM => Self::Ocm,
            spi::IRQ_PMU0 => Self::Pmu0,
            spi::IRQ_PMU1 => Self::Pmu1,
            spi::IRQ_XADC => Self::Xadc,
            spi::IRQ_DEV_C => Self::DevC,
            spi::IRQ_SWDT => Self::Swdt,
            spi::IRQ_TTC0_0 => Self::Ttc00,
            spi::IRQ_TTC0_1 => Self::Ttc01,
            spi::IRQ_TTC0_2 => Self::Ttc02,
            spi::IRQ_DMAC_ABORT => Self::DmacAbort,
            spi::IRQ_DMAC0 => Self::Dmac0,
            spi::IRQ_DMAC1 => Self::Dmac1,
            spi::IRQ_DMAC2 => Self::Dmac2,
            spi::IRQ_DMAC3 => Self::Dmac3,
            spi::IRQ_SMC => Self::Smc,
            spi::IRQ_QUAD_SPI => Self::QuadSpi,
            spi::IRQ_GPIO => Self::Gpio,
            spi::IRQ_USB0 => Self::Usb0,
            spi::IRQ_ETHERNET0 => Self::Ethernet0,
            spi::IRQ_ETHERNET0_WAKEUP => Self::Ethernet0Wakeup,
            spi::IRQ_SDIO0 => Self::Sdio0,
            spi::IRQ_I2C0 => Self::I2c0,
            spi::IRQ_SPI0 => Self::Spi0,
            spi::IRQ_UART0 => Self::Uart0,
            spi::IRQ_CAN0 => Self::Can0,
            spi::IRQ_PL0 => Self::Pl0,
            spi::IRQ_PL1 => Self::Pl1,
            spi::IRQ_PL2 => Self::Pl2,
            spi::IRQ_PL3 => Self::Pl3,
            spi::IRQ_PL4 => Self::Pl4,
            spi::IRQ_PL5 => Self::Pl5,
            spi::IRQ_PL6 => Self::Pl6,
            spi::IRQ_PL7 => Self::Pl7,
            spi::IRQ_TTC1_0 => Self::Ttc10,
            spi::IRQ_TTC1_1 => Self::Ttc11,
            spi::IRQ_TTC1_2 => Self::Ttc12,
            spi::IRQ_DMAC4 => Self::Dmac4,
            spi::IRQ_DMAC5 => Self::Dmac5,
            spi::IRQ_DMAC6 => Self::Dmac6,
            spi::IRQ_DMAC7 => Self::Dmac7,
            spi::IRQ_USB1 => Self::Usb1,
            spi::IRQ_ETHERNET1 => Self::Ethernet1,
            spi::IRQ_ETHERNET1_WAKEUP => Self::Ethernet1Wakeup,
            spi::IRQ_SDIO1 => Self::Sdio1,
            spi::IRQ_I2C1 => Self::I2c1,
            spi::IRQ_SPI1 => Self::Spi1,
            spi::IRQ_UART1 => Self::Uart1,
            spi::IRQ_CAN1 => Self::Can1,
            spi::IRQ_PL8 => Self::Pl8,
            spi::IRQ_PL9 => Self::Pl9,
            spi::IRQ_PL10 => Self::Pl10,
            spi::IRQ_PL11 => Self::Pl11,
            spi::IRQ_PL12 => Self::Pl12,
            spi::IRQ_PL13 => Self::Pl13,
            spi::IRQ_PL14 => Self::Pl14,
            spi::IRQ_PL15 => Self::Pl15,
            spi::IRQ_PARITY => Self::Parity,
            unknown => {
                return Err(unknown);
            }
        };
        Ok(value)
    }

    pub fn as_u32(self) -> u32 {
        match self {
            Self::Cpu0 => spi::IRQ_CPU0,
            Self::Cpu1 => spi::IRQ_CPU1,
            Self::L2Cache => spi::IRQ_L2_CACHE,
            Self::Ocm => spi::IRQ_OCM,
            Self::Pmu0 => spi::IRQ_PMU0,
            Self::Pmu1 => spi::IRQ_PMU1,
            Self::Xadc => spi::IRQ_XADC,
            Self::DevC => spi::IRQ_DEV_C,
            Self::Swdt => spi::IRQ_SWDT,
            Self::Ttc00 => spi::IRQ_TTC0_0,
            Self::Ttc01 => spi::IRQ_TTC0_1,
            Self::Ttc02 => spi::IRQ_TTC0_2,
            Self::DmacAbort => spi::IRQ_DMAC_ABORT,
            Self::Dmac0 => spi::IRQ_DMAC0,
            Self::Dmac1 => spi::IRQ_DMAC1,
            Self::Dmac2 => spi::IRQ_DMAC2,
            Self::Dmac3 => spi::IRQ_DMAC3,
            Self::Smc => spi::IRQ_SMC,
            Self::QuadSpi => spi::IRQ_QUAD_SPI,
            Self::Gpio => spi::IRQ_GPIO,
            Self::Usb0 => spi::IRQ_USB0,
            Self::Ethernet0 => spi::IRQ_ETHERNET0,
            Self::Ethernet0Wakeup => spi::IRQ_ETHERNET0_WAKEUP,
            Self::Sdio0 => spi::IRQ_SDIO0,
            Self::I2c0 => spi::IRQ_I2C0,
            Self::Spi0 => spi::IRQ_SPI0,
            Self::Uart0 => spi::IRQ_UART0,
            Self::Can0 => spi::IRQ_CAN0,
            Self::Pl0 => spi::IRQ_PL0,
            Self::Pl1 => spi::IRQ_PL1,
            Self::Pl2 => spi::IRQ_PL2,
            Self::Pl3 => spi::IRQ_PL3,
            Self::Pl4 => spi::IRQ_PL4,
            Self::Pl5 => spi::IRQ_PL5,
            Self::Pl6 => spi::IRQ_PL6,
            Self::Pl7 => spi::IRQ_PL7,
            Self::Ttc10 => spi::IRQ_TTC1_0,
            Self::Ttc11 => spi::IRQ_TTC1_1,
            Self::Ttc12 => spi::IRQ_TTC1_2,
            Self::Dmac4 => spi::IRQ_DMAC4,
            Self::Dmac5 => spi::IRQ_DMAC5,
            Self::Dmac6 => spi::IRQ_DMAC6,
            Self::Dmac7 => spi::IRQ_DMAC7,
            Self::Usb1 => spi::IRQ_USB1,
            Self::Ethernet1 => spi::IRQ_ETHERNET1,
            Self::Ethernet1Wakeup => spi::IRQ_ETHERNET1_WAKEUP,
            Self::Sdio1 => spi::IRQ_SDIO1,
            Self::I2c1 => spi::IRQ_I2C1,
            Self::Spi1 => spi::IRQ_SPI1,
            Self::Uart1 => spi::IRQ_UART1,
            Self::Can1 => spi::IRQ_CAN1,
            Self::Pl8 => spi::IRQ_PL8,
            Self::Pl9 => spi::IRQ_PL9,
            Self::Pl10 => spi::IRQ_PL10,
            Self::Pl11 => spi::IRQ_PL11,
            Self::Pl12 => spi::IRQ_PL12,
            Self::Pl13 => spi::IRQ_PL13,
            Self::Pl14 => spi::IRQ_PL14,
            Self::Pl15 => spi::IRQ_PL15,
            Self::Parity => spi::IRQ_PARITY,
        }
    }
}

/// Interrupt request.
#[derive(Clone, Copy)]
pub enum Irq {
    /// Software generated interrupt.
    Sgi(SgiIrq),

    /// Private peripheral interrupt.
    Ppi(PpiIrq),

    /// Shared peripheral interrupt.
    Spi(SpiIrq),
}

impl Irq {
    pub fn from_u32(value: u32) -> Self {
        if let Ok(sgi) = SgiIrq::from_u32(value) {
            Self::Sgi(sgi)
        } else if let Ok(ppi) = PpiIrq::from_u32(value) {
            Self::Ppi(ppi)
        } else if let Ok(spi) = SpiIrq::from_u32(value) {
            Self::Spi(spi)
        } else {
            panic!("Unknown IRQ number: {value}");
        }
    }

    pub fn as_u32(self) -> u32 {
        match self {
            Self::Sgi(sgi) => sgi.as_u32(),
            Self::Ppi(ppi) => ppi.as_u32(),
            Self::Spi(spi) => spi.as_u32(),
        }
    }
}
