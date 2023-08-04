//! General interrupt controller.
//!
//! Board uses 5 bits to indicate interrupt's priority.
//! The lower the value, the higher the priority.
//!
//! # How to use?
//!
//! ```ignore
//! GIC.toggle(false);
//! let interrupt = Irq::Spi(SpiIrq::Pl0);
//! GIC.toggle_interrupt(interrupt, true);
//! GIC.set_shared_peripheral_interrupt_sensitivity(interrupt, InterruptSensitivity::Edge);
//! GIC.set_shared_peripheral_interrupt_targets(interrupt, InterruptTargets::Cpu0);
//! GIC.set_interrupt_priority(interrupt, InterruptPriority::Priority0);
//! GIC.toggle(true);
//! ```
//!
//! You must configure [`ICC`](super::icc) to enable detection of interrupts by the processor core.

#![allow(unused)]

use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;

use super::irq_numbers::Irq;
use super::irq_numbers::SgiIrq;
use super::irq_numbers::SpiIrq;
use super::InterruptPriority;

#[derive(Copy, Clone)]
pub enum InterruptSecurity {
    /// Interrupt is secure.
    Secure,

    /// Interrupt is non-secure.
    NonSecure,
}

impl InterruptSecurity {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Secure => false,
            Self::NonSecure => true,
        }
    }
}

// TODO: use this
pub enum InterruptState {
    Inactive,
    Pending,
    Active,
    PendingActive,
}

/// Which CPU handles the interrupt.
#[derive(Clone, Copy)]
pub enum InterruptTargets {
    /// Interrupt is not signaled to any CPU.
    None,

    /// Interrupt is signaled to CPU 0.
    Cpu0,

    /// Interrupt is signaled to CPU 1.
    Cpu1,

    /// Interrupt is signaled to both CPU 0 and CPU 1.
    Both,
}

impl InterruptTargets {
    pub fn as_u32(self) -> u32 {
        match self {
            Self::None => 0b00,
            Self::Cpu0 => 0b01,
            Self::Cpu1 => 0b10,
            Self::Both => 0b11,
        }
    }

    pub fn from_u32(value: u32) -> Self {
        match value {
            0b00 => Self::None,
            0b01 => Self::Cpu0,
            0b10 => Self::Cpu1,
            0b11 => Self::Both,
            other => panic!("Invalid interrupt targets: {other}"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum InterruptSensitivity {
    /// Interrupt is level-sensitive.
    Level,

    /// Interrupt is edge-triggered.
    Edge,
}

impl InterruptSensitivity {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Level => false,
            Self::Edge => true,
        }
    }
}

/// Configures the side-effects of acknowledgement by one target.
///
/// TODO: is this deprecated after v1.0 GIC?
#[derive(Copy, Clone)]
pub enum InterruptHandlingModel {
    /// If interrupt is acknowledged by one target, then the interrupt is still pending for other targets.
    NToN,

    /// If interrupt is acknowledged by one target, then the interrupt will change from pending to inactive for other targets.
    OneToN,
}

impl InterruptHandlingModel {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::NToN => false,
            Self::OneToN => true,
        }
    }
}

// TODO
#[derive(Copy, Clone)]
pub enum TargetListFilter {
    /// Send the interrupt to the CPU interfaces specified in the CpuTargetList field.
    Option1,

    /// Send the interrupt to all CPU interfaces except the CPU interface that requested the interrupt.
    Option2,

    /// Send the interrupt only to the CPU interface that requested the interrupt.
    Option3,
}

impl TargetListFilter {
    /// Transform to unsigned 32-bit integer.
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::Option1 => 0b00,
            Self::Option2 => 0b01,
            Self::Option3 => 0b10,
        }
    }
}

/// How many interrupts are configured by single address.
enum InterruptsPerAddress {
    /// Register configures 4 interrupts.
    Count4,

    /// Register configures 16 interrupts.
    Count16,

    /// Register configures 32 interrupts.
    Count32,
}

struct SolvedAddressOffset {
    offset_register: usize,
    offset_bit: u32,
}

impl SolvedAddressOffset {
    fn solve(interrupt_id: u32, interrupts_per_address: InterruptsPerAddress) -> Self {
        let offset_register = match interrupts_per_address {
            InterruptsPerAddress::Count4 => match interrupt_id {
                0..=3 => 0,
                4..=7 => 1,
                8..=11 => 2,
                12..=15 => 3,
                16..=19 => 4,
                20..=23 => 5,
                24..=27 => 6,
                28..=31 => 7,
                32..=35 => 8,
                36..=39 => 9,
                40..=43 => 10,
                44..=47 => 11,
                48..=51 => 12,
                52..=55 => 13,
                56..=59 => 14,
                60..=63 => 15,
                64..=67 => 16,
                68..=71 => 17,
                72..=75 => 18,
                76..=79 => 19,
                80..=83 => 20,
                84..=87 => 21,
                88..=91 => 22,
                92..=95 => 23,
                unknown => panic!("Unknown IRQ ID: {unknown}"),
            },
            InterruptsPerAddress::Count16 => match interrupt_id {
                0..=15 => 0,
                16..=31 => 1,
                32..=47 => 2,
                48..=63 => 3,
                64..=79 => 4,
                80..=95 => 5,
                unknown => panic!("Unknown IRQ ID: {unknown}"),
            },
            InterruptsPerAddress::Count32 => match interrupt_id {
                0..=31 => 0,
                32..=63 => 1,
                64..=95 => 2,
                unknown => panic!("Unknown IRQ ID: {unknown}"),
            },
        };
        // TODO: maybe replace with modulus calculation
        let offset_bit = match interrupts_per_address {
            InterruptsPerAddress::Count4 => match interrupt_id {
                0 => 0,
                1 => 8,
                2 => 16,
                3 => 24,
                4 => 0,
                5 => 8,
                6 => 16,
                7 => 24,
                8 => 0,
                9 => 8,
                10 => 16,
                11 => 24,
                12 => 0,
                13 => 8,
                14 => 16,
                15 => 24,
                16 => 0,
                17 => 8,
                18 => 16,
                19 => 24,
                20 => 0,
                21 => 8,
                22 => 16,
                23 => 24,
                24 => 0,
                25 => 8,
                26 => 16,
                27 => 24,
                28 => 0,
                29 => 8,
                30 => 16,
                31 => 24,
                32 => 0,
                33 => 8,
                34 => 16,
                35 => 24,
                36 => 0,
                37 => 8,
                38 => 16,
                39 => 24,
                40 => 0,
                41 => 8,
                42 => 16,
                43 => 24,
                44 => 0,
                45 => 8,
                46 => 16,
                47 => 24,
                48 => 0,
                49 => 8,
                50 => 16,
                51 => 24,
                52 => 0,
                53 => 8,
                54 => 16,
                55 => 24,
                56 => 0,
                57 => 8,
                58 => 16,
                59 => 24,
                60 => 0,
                61 => 8,
                62 => 16,
                63 => 24,
                64 => 0,
                65 => 8,
                66 => 16,
                67 => 24,
                68 => 0,
                69 => 8,
                70 => 16,
                71 => 24,
                72 => 0,
                73 => 8,
                74 => 16,
                75 => 24,
                76 => 0,
                77 => 8,
                78 => 16,
                79 => 24,
                80 => 0,
                81 => 8,
                82 => 16,
                83 => 24,
                84 => 0,
                85 => 8,
                86 => 16,
                87 => 24,
                88 => 0,
                89 => 8,
                90 => 16,
                91 => 24,
                92 => 0,
                93 => 8,
                94 => 16,
                95 => 24,
                unknown => panic!("Unknown IRQ ID: {unknown}"),
            },
            InterruptsPerAddress::Count16 => match interrupt_id {
                0 => 0,
                1 => 2,
                2 => 4,
                3 => 6,
                4 => 8,
                5 => 10,
                6 => 12,
                7 => 14,
                8 => 16,
                9 => 18,
                10 => 20,
                11 => 22,
                12 => 24,
                13 => 26,
                14 => 28,
                15 => 30,
                16 => 0,
                17 => 2,
                18 => 4,
                19 => 6,
                20 => 8,
                21 => 10,
                22 => 12,
                23 => 14,
                24 => 16,
                25 => 18,
                26 => 20,
                27 => 22,
                28 => 24,
                29 => 26,
                30 => 28,
                31 => 30,
                32 => 0,
                33 => 2,
                34 => 4,
                35 => 6,
                36 => 8,
                37 => 10,
                38 => 12,
                39 => 14,
                40 => 16,
                41 => 18,
                42 => 20,
                43 => 22,
                44 => 24,
                45 => 26,
                46 => 28,
                47 => 30,
                48 => 0,
                49 => 2,
                50 => 4,
                51 => 6,
                52 => 8,
                53 => 10,
                54 => 12,
                55 => 14,
                56 => 16,
                57 => 18,
                58 => 20,
                59 => 22,
                60 => 24,
                61 => 26,
                62 => 28,
                63 => 30,
                64 => 0,
                65 => 2,
                66 => 4,
                67 => 6,
                68 => 8,
                69 => 10,
                70 => 12,
                71 => 14,
                72 => 16,
                73 => 18,
                74 => 20,
                75 => 22,
                76 => 24,
                77 => 26,
                78 => 28,
                79 => 30,
                80 => 0,
                81 => 2,
                82 => 4,
                83 => 6,
                84 => 8,
                85 => 10,
                86 => 12,
                87 => 14,
                88 => 16,
                89 => 18,
                90 => 20,
                91 => 22,
                92 => 24,
                93 => 26,
                94 => 28,
                95 => 30,
                unknown => panic!("Unknown IRQ ID: {unknown}"),
            },
            InterruptsPerAddress::Count32 => match interrupt_id {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 4,
                5 => 5,
                6 => 6,
                7 => 7,
                8 => 8,
                9 => 9,
                10 => 10,
                11 => 11,
                12 => 12,
                13 => 13,
                14 => 14,
                15 => 15,
                16 => 16,
                17 => 17,
                18 => 18,
                19 => 19,
                20 => 20,
                21 => 21,
                22 => 22,
                23 => 23,
                24 => 24,
                25 => 25,
                26 => 26,
                27 => 27,
                28 => 28,
                29 => 29,
                30 => 30,
                31 => 31,
                32 => 0,
                33 => 1,
                34 => 2,
                35 => 3,
                36 => 4,
                37 => 5,
                38 => 6,
                39 => 7,
                40 => 8,
                41 => 9,
                42 => 10,
                43 => 11,
                44 => 12,
                45 => 13,
                46 => 14,
                47 => 15,
                48 => 16,
                49 => 17,
                50 => 18,
                51 => 19,
                52 => 20,
                53 => 21,
                54 => 22,
                55 => 23,
                56 => 24,
                57 => 25,
                58 => 26,
                59 => 27,
                60 => 28,
                61 => 29,
                62 => 30,
                63 => 31,
                64 => 0,
                65 => 1,
                66 => 2,
                67 => 3,
                68 => 4,
                69 => 5,
                70 => 6,
                71 => 7,
                72 => 8,
                73 => 9,
                74 => 10,
                75 => 11,
                76 => 12,
                77 => 13,
                78 => 14,
                79 => 15,
                80 => 16,
                81 => 17,
                82 => 18,
                83 => 19,
                84 => 20,
                85 => 21,
                86 => 22,
                87 => 23,
                88 => 24,
                89 => 25,
                90 => 26,
                91 => 27,
                92 => 28,
                93 => 29,
                94 => 30,
                95 => 31,
                unknown => panic!("Unknown IRQ ID: {unknown}"),
            },
        };
        SolvedAddressOffset {
            offset_register,
            offset_bit,
        }
    }
}

/// General Interrupt Controller (GIC).
///
/// GIC is responsible for monitoring peripheral interrupt signals and forwarding pending interrupt to CPU interfaces.
pub struct Gic {
    /// Distributor control register.
    pub address_distributor_control: *mut u32,

    /// Interrupt controller type register.
    pub address_interrupt_controller_type: *mut u32,

    /// Distributor implementer identification register.
    pub address_distributor_implementer_identification: *mut u32,

    /// Interrupt security registers.
    pub addresses_interrupt_security: [*mut u32; 3],

    /// Interrupt set-enable registers.
    pub addresses_interrupt_set_enable: [*mut u32; 3],

    /// Interrupt clear-enable registers.
    pub addresses_interrupt_clear_enable: [*mut u32; 3],

    /// Interrupt set-pending registers.
    pub addresses_interrupt_set_pending: [*mut u32; 3],

    /// Interrupt clear-pending registers.
    pub addresses_interrupt_clear_pending: [*mut u32; 3],

    /// Active bit registers.
    pub addresses_active_bit: [*mut u32; 3],

    /// Interrupt priority registers.
    pub addresses_interrupt_priority: [*mut u32; 24],

    /// Interrupt processor targets registers.
    pub addresses_interrupt_processor_targets: [*mut u32; 24],

    /// Interrupt configuration registers.
    ///
    /// Corresponding interrupt must be disabled before altering.
    pub addresses_interrupt_configuration: [*mut u32; 6],

    /// Software generated interrupt register.
    pub address_software_generated_interrupt: *mut u32,

    /// Peripheral ID2 register.
    pub address_peripheral_id2: *mut u32,

    /// ARM-defined fixed values for the preamble for component discovery.
    pub addresses_component_id: [*mut u32; 4],

    /// ARM-defined identification registers.
    pub addresses_peripheral_id: [*mut u32; 8],
}

impl Gic {
    /// Enable or disable GIC.
    #[inline]
    pub fn toggle(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_distributor_control, 0);
    }

    // TODO: interrupt controller type register
    // TODO: distributor implementer identification register

    /// Set interrupt security.
    #[inline]
    pub fn set_interrupt_security(&self, interrupt: Irq, security: InterruptSecurity) {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count32);
        let address = self.addresses_interrupt_security[offset_register];
        let action = if security.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(address, offset_bit);
    }

    /// Enable or disable interrupt.
    #[inline]
    pub fn toggle_interrupt(&self, interrupt: Irq, enable: bool) {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count32);
        let addresses = if enable {
            self.addresses_interrupt_set_enable
        } else {
            self.addresses_interrupt_clear_enable
        };
        let address = addresses[offset_register];
        set_address_bit(address, offset_bit);
    }

    /// True if interrupt is enabled.
    #[inline]
    #[must_use]
    pub fn is_interrupt_enabled(&self, interrupt: Irq) -> bool {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count32);
        let address = self.addresses_interrupt_set_enable[offset_register];
        read_address_bit(address, offset_bit)
    }

    /// Enable or disable interrupt's pending status.
    #[inline]
    pub fn toggle_interrupt_pending(&self, interrupt: Irq, enable: bool) {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count32);
        let addresses = if enable {
            self.addresses_interrupt_set_pending
        } else {
            self.addresses_interrupt_clear_pending
        };
        let address = addresses[offset_register];
        set_address_bit(address, offset_bit);
    }

    /// True if interrupt is pending.
    #[inline]
    #[must_use]
    pub fn is_interrupt_pending(&self, interrupt: Irq) -> bool {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count32);
        let address = self.addresses_interrupt_set_pending[offset_register];
        read_address_bit(address, offset_bit)
    }

    /// True if interrupt is active.
    #[inline]
    #[must_use]
    pub fn is_interrupt_active(&self, interrupt: Irq) -> bool {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count32);
        let address = self.addresses_active_bit[offset_register];
        read_address_bit(address, offset_bit)
    }

    #[inline]
    pub fn set_interrupt_priority(&self, interrupt: Irq, priority: InterruptPriority) {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count4);
        let address = self.addresses_interrupt_priority[offset_register];
        let indices = offset_bit..=offset_bit + 8;
        write_address_bits(address, indices, priority as u32);
    }

    #[inline]
    #[must_use]
    pub fn read_interrupt_priority(&self, interrupt: Irq) -> InterruptPriority {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count4);
        let address = self.addresses_interrupt_priority[offset_register];
        let indices = offset_bit..=offset_bit + 8;
        let value = read_address_bits(address, indices) as u8;
        InterruptPriority::from_u8(value)
    }

    /// Select which CPU handles given interrupt.
    ///
    /// Private peripheral interrupts are always handled by corresponding CPU.
    #[inline]
    fn set_interrupt_targets(&self, interrupt: Irq, targets: InterruptTargets) {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count4);
        let address = self.addresses_interrupt_processor_targets[offset_register];
        let indices = offset_bit..=offset_bit + 8;
        let targets = targets.as_u32();
        write_address_bits(address, indices, targets);
    }

    /// Select which CPU handles given sofware generated interrupt.
    pub fn set_software_generated_interrupt_targets(&self, sgi: SgiIrq, targets: InterruptTargets) {
        self.set_interrupt_targets(Irq::Sgi(sgi), targets);
    }

    /// Select which CPU handles given shared peripheral interrupt.
    pub fn set_shared_peripheral_interrupt_targets(&self, spi: SpiIrq, targets: InterruptTargets) {
        self.set_interrupt_targets(Irq::Spi(spi), targets);
    }

    /// Read which CPU handles given interrupt.
    #[inline]
    pub fn read_interrupt_targets(&self, interrupt: Irq) -> InterruptTargets {
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(interrupt.as_u32(), InterruptsPerAddress::Count4);
        let address = self.addresses_interrupt_processor_targets[offset_register];
        let indices = offset_bit..=offset_bit + 8;
        let targets = read_address_bits(address, indices);
        InterruptTargets::from_u32(targets)
    }

    /// Configure how shared peripheral interrupt is triggered.
    ///
    /// Notice! All SPIs except PLs require specific sensitivity configuration.
    ///
    /// 1. Software generated interrupts are fixed to edge-triggered configuration.
    /// 2. Private peripheral interrupts have fixed sensitivity.
    #[inline]
    pub fn set_shared_peripheral_interrupt_sensitivity(
        &self,
        spi: SpiIrq,
        sensitivity: InterruptSensitivity,
    ) -> Result<(), ()> {
        // TODO: disable corresponding interrupt before altering
        let SolvedAddressOffset {
            offset_register,
            offset_bit,
        } = SolvedAddressOffset::solve(spi.as_u32(), InterruptsPerAddress::Count16);
        let action = if sensitivity.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        let address = self.addresses_interrupt_configuration[offset_register];
        action(address, offset_bit);
        Ok(())
    }

    /// TODO: is this deprecated?
    #[inline]
    pub fn toggle_interrupt_handling_model(&self, interrupt: Irq, model: InterruptHandlingModel) {
        // TODO: disable corresponding interrupt before altering
        todo!();
    }

    #[inline]
    pub fn generate_software_interrupt(
        &self,
        sgi: SgiIrq,
        // TODO: use enum
        satt: bool,
        // TODO: use enum
        cpu_target_list: u8,
        target_list_filter: TargetListFilter,
    ) {
        let address = self.address_software_generated_interrupt;
        write_address_bits(address, 0..=3, sgi.as_u32());
        let action = if satt {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(address, 15);
        write_address_bits(address, 16..=23, cpu_target_list as u32);
        write_address_bits(address, 24..=25, target_list_filter.as_u32());
    }

    // TODO: use enum
    /// Read revision field for the GIC architecture.
    #[inline]
    #[must_use]
    pub fn read_architecture_revision(&self) -> u32 {
        read_address_bits(self.address_peripheral_id2, 4..=7)
    }

    // TODO: identification registers
    // TODO: GIC identification registers

    /// Configure sensitivities for interrupts.
    ///
    /// 1. Software generated interrupts are fixed to edge-triggered configuration.
    /// 2. Private peripheral interrupts have fixed sensitivity.
    /// 3. Shared peripheral interrupts (except PLs) require specific sensitivity configuration.
    pub fn configure_sensitivities(&self) {
        // Reset to initial values.
        // Each interrupt should be level-sensitive (majority of interrupts).
        unsafe {
            // Software generated interrupts are always edge-triggered.
            // Private peripheral interrupts have fixed sensitivities.
            core::ptr::write_volatile(self.addresses_interrupt_configuration[2], 0x5555_5555);
            core::ptr::write_volatile(self.addresses_interrupt_configuration[3], 0x5555_5555);
            core::ptr::write_volatile(self.addresses_interrupt_configuration[4], 0x5555_5555);
        }
        // Configure edge-triggered interrupts (minority of interrupts).
        self.set_shared_peripheral_interrupt_sensitivity(SpiIrq::Cpu0, InterruptSensitivity::Edge);
        self.set_shared_peripheral_interrupt_sensitivity(SpiIrq::Cpu1, InterruptSensitivity::Edge);
        self.set_shared_peripheral_interrupt_sensitivity(SpiIrq::Swdt, InterruptSensitivity::Edge);
        self.set_shared_peripheral_interrupt_sensitivity(
            SpiIrq::Ethernet0Wakeup,
            InterruptSensitivity::Edge,
        );
        self.set_shared_peripheral_interrupt_sensitivity(
            SpiIrq::Ethernet1Wakeup,
            InterruptSensitivity::Edge,
        );
        self.set_shared_peripheral_interrupt_sensitivity(
            SpiIrq::Parity,
            InterruptSensitivity::Edge,
        );
    }
}

/// Base address for memory mapped interrupt controller distributor.
const ADDRESS_ICD_BASE: u32 = 0xF8F0_1000;

const ADDRESS_ICDISR_START: u32 = ADDRESS_ICD_BASE + 0x080;
const ADDRESS_ICDISR_FINAL: u32 = ADDRESS_ICD_BASE + 0x088;
const ADDRESSES_ICDISR: [*mut u32; 3] = [
    (ADDRESS_ICD_BASE + 0x0000_0080) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0084) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0088) as *mut u32,
];

const ADDRESS_ISER_START: u32 = ADDRESS_ICD_BASE + 0x100;
const ADDRESS_ISER_FINAL: u32 = ADDRESS_ICD_BASE + 0x108;
const ADDRESSES_ISER: [*mut u32; 3] = [
    (ADDRESS_ICD_BASE + 0x0000_0100) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0104) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0108) as *mut u32,
];

const ADDRESS_ICER_START: u32 = ADDRESS_ICD_BASE + 0x180;
const ADDRESS_ICER_FINAL: u32 = ADDRESS_ICD_BASE + 0x188;
const ADDRESSES_ICER: [*mut u32; 3] = [
    (ADDRESS_ICD_BASE + 0x0000_0180) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0184) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0188) as *mut u32,
];

const ADDRESS_ISPR_START: u32 = ADDRESS_ICD_BASE + 0x200;
const ADDRESS_ISPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x208;
const ADDRESSES_ISPR: [*mut u32; 3] = [
    (ADDRESS_ICD_BASE + 0x0000_0200) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0204) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0208) as *mut u32,
];

const ADDRESS_ICPR_START: u32 = ADDRESS_ICD_BASE + 0x280;
const ADDRESS_ICPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x288;
const ADDRESSES_ICPR: [*mut u32; 3] = [
    (ADDRESS_ICD_BASE + 0x0000_0280) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0284) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0288) as *mut u32,
];

const ADDRESS_ABR_START: u32 = ADDRESS_ICD_BASE + 0x300;
const ADDRESS_ABR_FINAL: u32 = ADDRESS_ICD_BASE + 0x308;
const ADDRESSES_ABR: [*mut u32; 3] = [
    (ADDRESS_ICD_BASE + 0x0000_0300) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0304) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0308) as *mut u32,
];

const ADDRESS_IPR_START: u32 = ADDRESS_ICD_BASE + 0x400;
const ADDRESS_IPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x45C;
const ADDRESSES_IPR: [*mut u32; 24] = [
    (ADDRESS_ICD_BASE + 0x0000_0400) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0404) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0408) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_040C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0410) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0414) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0418) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_041C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0420) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0424) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0428) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_042C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0430) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0434) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0438) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_043C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0440) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0444) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0448) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_044C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0450) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0454) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0458) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_045C) as *mut u32,
];

const ADDRESS_IPTR_START: u32 = ADDRESS_ICD_BASE + 0x800;
const ADDRESS_IPTR_FINAL: u32 = ADDRESS_ICD_BASE + 0x85C;
const ADDRESSES_IPTR: [*mut u32; 24] = [
    (ADDRESS_ICD_BASE + 0x0000_0800) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0804) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0808) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_080C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0810) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0814) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0818) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_081C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0820) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0824) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0828) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_082C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0830) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0834) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0838) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_083C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0840) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0844) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0848) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_084C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0850) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0854) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0858) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_085C) as *mut u32,
];

const ADDRESS_ICFR_START: u32 = ADDRESS_ICD_BASE + 0xC00;
const ADDRESS_ICFR_FINAL: u32 = ADDRESS_ICD_BASE + 0xC14;
const ADDRESSES_ICFR: [*mut u32; 6] = [
    (ADDRESS_ICD_BASE + 0x0000_0C00) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C04) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C08) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C0C) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C10) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C14) as *mut u32,
];

const ADDRESS_COMPONENT_ID_START: u32 = ADDRESS_ICD_BASE + 0x0000_0C00;
const ADDRESS_COMPONENT_ID_FINAL: u32 = ADDRESS_ICD_BASE + 0x0000_0C0C;
const ADDRESSES_COMPONENT_ID: [*mut u32; 4] = [
    (ADDRESS_ICD_BASE + 0x0000_0C00) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C04) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C08) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0C0C) as *mut u32,
];

const ADDRESS_PERIPHERAL_ID_START: u32 = ADDRESS_ICD_BASE + 0x0000_0FE0;
const ADDRESS_PERIPHERAL_ID_FINAL: u32 = ADDRESS_ICD_BASE + 0x0000_0FDC;
const ADDRESSES_PERIPHERAL_ID: [*mut u32; 8] = [
    (ADDRESS_ICD_BASE + 0x0000_0FE0) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0FE4) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0FE8) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0FEC) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0FD0) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0FD4) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0FD8) as *mut u32,
    (ADDRESS_ICD_BASE + 0x0000_0FDC) as *mut u32,
];

/// General interrupt controller.
pub static mut GIC: Gic = Gic {
    address_distributor_control: (ADDRESS_ICD_BASE + 0x000) as *mut u32,
    address_interrupt_controller_type: (ADDRESS_ICD_BASE + 0x004) as *mut u32,
    address_distributor_implementer_identification: (ADDRESS_ICD_BASE + 0x008) as *mut u32,
    addresses_interrupt_security: ADDRESSES_ICDISR,
    addresses_interrupt_set_enable: ADDRESSES_ISER,
    addresses_interrupt_clear_enable: ADDRESSES_ICER,
    addresses_interrupt_set_pending: ADDRESSES_ISPR,
    addresses_interrupt_clear_pending: ADDRESSES_ICPR,
    addresses_active_bit: ADDRESSES_ABR,
    addresses_interrupt_priority: ADDRESSES_IPR,
    addresses_interrupt_processor_targets: ADDRESSES_IPTR,
    addresses_interrupt_configuration: ADDRESSES_ICFR,
    address_software_generated_interrupt: (ADDRESS_ICD_BASE + 0xF00) as *mut u32,
    address_peripheral_id2: (ADDRESS_ICD_BASE + 0xFE8) as *mut u32,
    addresses_component_id: ADDRESSES_COMPONENT_ID,
    addresses_peripheral_id: ADDRESSES_PERIPHERAL_ID,
};
