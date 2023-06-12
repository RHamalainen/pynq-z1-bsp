//! General interrupt controller.
//!
//! Board uses 5 bits to indicate interrupt's priority.
//! The lower the value, the higher the priority.
//!
//! # How to use?
//!
//! ```ignore
//! GIC.toggle_enable(false);
//! GIC.toggle_interrupt_enable(<irq number>, true);
//! GIC.set_interrupt_targets(<irq number>, 0b1);
//! GIC.set_interrupt_priority(<irq number>, 0b0);
//! GIC.toggle_enable(true);
//! ```
//!
//! Note that you also need to configure `ICC` to enable detection of interrupts by the processor core.

#![allow(unused)]
#![allow(clippy::missing_docs_in_private_items)]

use core::ops::Div;
use core::ops::Mul;
use core::ops::Rem;

use crate::common::bitman::ReadBitwiseRange;
use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;

/// Base address for memory mapped interrupt controller distributor.
pub const ADDRESS_ICD_BASE: u32 = 0xF8F0_1000;

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

/// TODO: is this deprecated after v1.0 GIC?
#[derive(Copy, Clone)]
pub enum InterruptHandlingModel {
    NToN,
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

/// General Interrupt Controller (GIC).
pub struct Gic {
    /// Distributor control register.
    pub address_distributor_control: *mut u32,

    /// Interrupt controller type register.
    pub address_interrupt_controller_type: *mut u32,

    /// Distributor implementer identification register.
    pub address_distributor_implementer_identification: *mut u32,

    /// Interrupt security registers.
    pub addresses_interrupt_security: [u32; 3],

    /// Interrupt set-enable registers.
    pub addresses_interrupt_set_enable: [u32; 3],

    /// Interrupt clear-enable registers.
    pub addresses_interrupt_clear_enable: [u32; 3],

    /// Interrupt set-pending registers.
    pub addresses_interrupt_set_pending: [u32; 3],

    /// Interrupt clear-pending registers.
    pub addresses_interrupt_clear_pending: [u32; 3],

    /// Active bit registers.
    pub addresses_active_bit: [u32; 3],

    /// Interrupt priority registers.
    pub addresses_interrupt_priority: [u32; 24],

    /// Interrupt processor targets registers.
    pub addresses_interrupt_processor_targets: [u32; 24],

    /// Interrupt configuration registers.
    ///
    /// Corresponding interrupt must be disabled before altering.
    pub addresses_interrupt_configuration: [u32; 6],

    /// Software generated interrupt register.
    pub address_software_generated_interrupt: *mut u32,

    /// Peripheral ID2 register.
    pub address_peripheral_id2: *mut u32,

    /// ARM-defined fixed values for the preamble for component discovery.
    pub addresses_component_id: [u32; 4],

    /// ARM-defined identification registers.
    pub addresses_peripheral_id: [u32; 8],
}

impl Gic {
    /// Enable or disable monitoring peripheral interrupt signals and forwarding pending interrupts to the CPU interfaces.
    #[inline]
    pub fn toggle_enable(&self, enable: bool) {
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
    pub fn set_interrupt_security(&self, interrupt_id: usize, security: InterruptSecurity) {
        let offset_register = interrupt_id.div(32).mul(4);
        let offset_bit = interrupt_id.rem(32) as u32;
        let address = self.addresses_interrupt_security[offset_register] as *mut u32;
        let action = if security.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(address, offset_bit);
    }

    /// Enable or disable interrupt.
    #[inline]
    pub fn toggle_interrupt_enable(&self, interrupt_id: usize, enable: bool) {
        let offset_register = interrupt_id.div(32);
        let offset_bit = interrupt_id.rem(32) as u32;
        let addresses = if enable {
            self.addresses_interrupt_set_enable
        } else {
            self.addresses_interrupt_clear_enable
        };
        let address = addresses[offset_register] as *mut u32;
        set_address_bit(address, offset_bit);
    }

    /// True if interrupt is enabled.
    #[inline]
    #[must_use]
    pub fn is_interrupt_enabled(&self, interrupt_id: usize) -> bool {
        // TODO: maybe remove mul4
        let offset_register = interrupt_id.div(32).mul(4);
        let offset_bit = interrupt_id.rem(32) as u32;
        let address = self.addresses_interrupt_set_enable[offset_register] as *mut u32;
        read_address_bit(address, offset_bit)
    }

    /// Enable or disable interrupt's pending status.
    #[inline]
    pub fn toggle_interrupt_pending(&self, interrupt_id: usize, enable: bool) {
        // TODO: maybe remove mul4
        let offset_register = interrupt_id.div(32).mul(4);
        let offset_bit = interrupt_id.rem(32) as u32;
        let addresses = if enable {
            self.addresses_interrupt_set_pending
        } else {
            self.addresses_interrupt_clear_pending
        };
        let address = addresses[offset_register] as *mut u32;
        set_address_bit(address, offset_bit);
    }

    /// True if interrupt is pending.
    #[inline]
    #[must_use]
    pub fn is_interrupt_pending(&self, interrupt_id: usize) -> bool {
        // TODO: maybe remove mul4
        let offset_register = interrupt_id.div(32).mul(4);
        let offset_bit = interrupt_id.rem(32) as u32;
        let address = self.addresses_interrupt_set_pending[offset_register] as *mut u32;
        read_address_bit(address, offset_bit)
    }

    /// True if interrupt is active.
    #[inline]
    #[must_use]
    pub fn is_interrupt_active(&self, interrupt_id: usize) -> bool {
        let offset_register = interrupt_id.div(32).mul(4);
        let offset_bit = interrupt_id.rem(32) as u32;
        let address = self.addresses_active_bit[offset_register] as *mut u32;
        read_address_bit(address, offset_bit)
    }

    #[inline]
    pub fn set_interrupt_priority(&self, interrupt_id: usize, priority: u8) {
        let offset_register = interrupt_id.div(4);
        let offset_bits = interrupt_id.rem(4).mul(8) as u32;
        let address = self.addresses_interrupt_priority[offset_register] as *mut u32;
        let indices = offset_bits..=offset_bits + 8;
        write_address_bits(address, indices, priority as u32);
    }

    #[inline]
    #[must_use]
    pub fn read_interrupt_priority(&self, interrupt_id: usize) -> u8 {
        let offset_register = interrupt_id.div(4);
        let offset_bits = interrupt_id.rem(4).mul(8) as u32;
        let address = self.addresses_interrupt_priority[offset_register] as *mut u32;
        let indices = offset_bits..=offset_bits + 8;
        read_address_bits(address, indices) as u8
    }

    // TODO: enumeration for targets
    #[inline]
    pub fn set_interrupt_targets(&self, interrupt_id: usize, targets: u8) {
        let offset_register = interrupt_id.div(4);
        let offset_bits = interrupt_id.rem(4).mul(8) as u32;
        let address = self.addresses_interrupt_processor_targets[offset_register] as *mut u32;
        let indices = offset_bits..=offset_bits + 8;
        write_address_bits(address, indices, targets as u32);
    }

    #[inline]
    pub fn read_interrupt_targets(&self, interrupt_id: usize) -> u8 {
        let offset_register = interrupt_id.div(4);
        let offset_bits = interrupt_id.rem(4).mul(8) as u32;
        let address = self.addresses_interrupt_processor_targets[offset_register] as *mut u32;
        let indices = offset_bits..=offset_bits + 8;
        read_address_bits(address, indices) as u8
    }

    #[inline]
    pub fn toggle_interrupt_sensitivity(
        &self,
        interrupt_id: usize,
        sensitivity: InterruptSensitivity,
    ) {
        // TODO: disable corresponding interrupt before altering
        // TODO: maybe remove mul4
        let offset_register = interrupt_id.div(16).mul(4);
        let offset_bit = interrupt_id.rem(16).mul(2) + 1;
        let action = if sensitivity.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        let address = self.addresses_interrupt_configuration[offset_register] as *mut u32;
        action(address, offset_bit as u32);
    }

    /// TODO: is this deprecated?
    #[inline]
    pub fn toggle_interrupt_handling_model(
        &self,
        interrupt_id: usize,
        model: InterruptHandlingModel,
    ) {
        // TODO: disable corresponding interrupt before altering
        todo!();
    }

    #[inline]
    pub fn generate_software_interrupt(
        &self,
        // TODO: this field is actually 4 bits
        interrupt_id: usize,
        satt: bool,
        cpu_target_list: u8,
        target_list_filter: TargetListFilter,
    ) {
        let address = self.address_software_generated_interrupt;
        write_address_bits(address, 0..=3, interrupt_id as u32);
        let action = if satt {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(address, 15);
        write_address_bits(address, 16..=23, cpu_target_list as u32);
        write_address_bits(address, 24..=25, target_list_filter.as_u32());
    }

    /// Read revision field for the GIC architecture.
    #[inline]
    #[must_use]
    pub fn read_architecture_revision(&self) -> u32 {
        read_address_bits(self.address_peripheral_id2, 4..=7)
    }

    // TODO: identification registers
    // TODO: GIC identification registers
}

const ADDRESS_ICDISR_START: u32 = ADDRESS_ICD_BASE + 0x080;
const ADDRESS_ICDISR_FINAL: u32 = ADDRESS_ICD_BASE + 0x088;
const ADDRESSES_ICDISR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0080,
    ADDRESS_ICD_BASE + 0x0000_0084,
    ADDRESS_ICD_BASE + 0x0000_0088,
];

const ADDRESS_ISER_START: u32 = ADDRESS_ICD_BASE + 0x100;
const ADDRESS_ISER_FINAL: u32 = ADDRESS_ICD_BASE + 0x108;
const ADDRESSES_ISER: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0100,
    ADDRESS_ICD_BASE + 0x0000_0104,
    ADDRESS_ICD_BASE + 0x0000_0108,
];

const ADDRESS_ICER_START: u32 = ADDRESS_ICD_BASE + 0x180;
const ADDRESS_ICER_FINAL: u32 = ADDRESS_ICD_BASE + 0x188;
const ADDRESSES_ICER: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0180,
    ADDRESS_ICD_BASE + 0x0000_0184,
    ADDRESS_ICD_BASE + 0x0000_0188,
];

const ADDRESS_ISPR_START: u32 = ADDRESS_ICD_BASE + 0x200;
const ADDRESS_ISPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x208;
const ADDRESSES_ISPR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0200,
    ADDRESS_ICD_BASE + 0x0000_0204,
    ADDRESS_ICD_BASE + 0x0000_0208,
];

const ADDRESS_ICPR_START: u32 = ADDRESS_ICD_BASE + 0x280;
const ADDRESS_ICPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x288;
const ADDRESSES_ICPR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0280,
    ADDRESS_ICD_BASE + 0x0000_0284,
    ADDRESS_ICD_BASE + 0x0000_0288,
];

const ADDRESS_ABR_START: u32 = ADDRESS_ICD_BASE + 0x300;
const ADDRESS_ABR_FINAL: u32 = ADDRESS_ICD_BASE + 0x308;
const ADDRESSES_ABR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0300,
    ADDRESS_ICD_BASE + 0x0000_0304,
    ADDRESS_ICD_BASE + 0x0000_0308,
];

const ADDRESS_IPR_START: u32 = ADDRESS_ICD_BASE + 0x400;
const ADDRESS_IPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x45C;
const ADDRESSES_IPR: [u32; 24] = [
    ADDRESS_ICD_BASE + 0x0000_0400,
    ADDRESS_ICD_BASE + 0x0000_0404,
    ADDRESS_ICD_BASE + 0x0000_0408,
    ADDRESS_ICD_BASE + 0x0000_040C,
    ADDRESS_ICD_BASE + 0x0000_0410,
    ADDRESS_ICD_BASE + 0x0000_0414,
    ADDRESS_ICD_BASE + 0x0000_0418,
    ADDRESS_ICD_BASE + 0x0000_041C,
    ADDRESS_ICD_BASE + 0x0000_0420,
    ADDRESS_ICD_BASE + 0x0000_0424,
    ADDRESS_ICD_BASE + 0x0000_0428,
    ADDRESS_ICD_BASE + 0x0000_042C,
    ADDRESS_ICD_BASE + 0x0000_0430,
    ADDRESS_ICD_BASE + 0x0000_0434,
    ADDRESS_ICD_BASE + 0x0000_0438,
    ADDRESS_ICD_BASE + 0x0000_043C,
    ADDRESS_ICD_BASE + 0x0000_0440,
    ADDRESS_ICD_BASE + 0x0000_0444,
    ADDRESS_ICD_BASE + 0x0000_0448,
    ADDRESS_ICD_BASE + 0x0000_044C,
    ADDRESS_ICD_BASE + 0x0000_0450,
    ADDRESS_ICD_BASE + 0x0000_0454,
    ADDRESS_ICD_BASE + 0x0000_0458,
    ADDRESS_ICD_BASE + 0x0000_045C,
];

const ADDRESS_IPTR_START: u32 = ADDRESS_ICD_BASE + 0x800;
const ADDRESS_IPTR_FINAL: u32 = ADDRESS_ICD_BASE + 0x85C;
const ADDRESSES_IPTR: [u32; 24] = [
    ADDRESS_ICD_BASE + 0x0000_0800,
    ADDRESS_ICD_BASE + 0x0000_0804,
    ADDRESS_ICD_BASE + 0x0000_0808,
    ADDRESS_ICD_BASE + 0x0000_080C,
    ADDRESS_ICD_BASE + 0x0000_0810,
    ADDRESS_ICD_BASE + 0x0000_0814,
    ADDRESS_ICD_BASE + 0x0000_0818,
    ADDRESS_ICD_BASE + 0x0000_081C,
    ADDRESS_ICD_BASE + 0x0000_0820,
    ADDRESS_ICD_BASE + 0x0000_0824,
    ADDRESS_ICD_BASE + 0x0000_0828,
    ADDRESS_ICD_BASE + 0x0000_082C,
    ADDRESS_ICD_BASE + 0x0000_0830,
    ADDRESS_ICD_BASE + 0x0000_0834,
    ADDRESS_ICD_BASE + 0x0000_0838,
    ADDRESS_ICD_BASE + 0x0000_083C,
    ADDRESS_ICD_BASE + 0x0000_0840,
    ADDRESS_ICD_BASE + 0x0000_0844,
    ADDRESS_ICD_BASE + 0x0000_0848,
    ADDRESS_ICD_BASE + 0x0000_084C,
    ADDRESS_ICD_BASE + 0x0000_0850,
    ADDRESS_ICD_BASE + 0x0000_0854,
    ADDRESS_ICD_BASE + 0x0000_0858,
    ADDRESS_ICD_BASE + 0x0000_085C,
];

const ADDRESS_ICFR_START: u32 = ADDRESS_ICD_BASE + 0xC00;
const ADDRESS_ICFR_FINAL: u32 = ADDRESS_ICD_BASE + 0xC14;
const ADDRESSES_ICFR: [u32; 6] = [
    ADDRESS_ICD_BASE + 0x0000_0C00,
    ADDRESS_ICD_BASE + 0x0000_0C04,
    ADDRESS_ICD_BASE + 0x0000_0C08,
    ADDRESS_ICD_BASE + 0x0000_0C0C,
    ADDRESS_ICD_BASE + 0x0000_0C10,
    ADDRESS_ICD_BASE + 0x0000_0C14,
];

const ADDRESS_COMPONENT_ID_START: u32 = ADDRESS_ICD_BASE + 0x0000_0C00;
const ADDRESS_COMPONENT_ID_FINAL: u32 = ADDRESS_ICD_BASE + 0x0000_0C0C;
const ADDRESSES_COMPONENT_ID: [u32; 4] = [
    ADDRESS_ICD_BASE + 0x0000_0C00,
    ADDRESS_ICD_BASE + 0x0000_0C04,
    ADDRESS_ICD_BASE + 0x0000_0C08,
    ADDRESS_ICD_BASE + 0x0000_0C0C,
];

const ADDRESS_PERIPHERAL_ID_START: u32 = ADDRESS_ICD_BASE + 0x0000_0FE0;
const ADDRESS_PERIPHERAL_ID_FINAL: u32 = ADDRESS_ICD_BASE + 0x0000_0FDC;
const ADDRESSES_PERIPHERAL_ID: [u32; 8] = [
    ADDRESS_ICD_BASE + 0x0000_0FE0,
    ADDRESS_ICD_BASE + 0x0000_0FE4,
    ADDRESS_ICD_BASE + 0x0000_0FE8,
    ADDRESS_ICD_BASE + 0x0000_0FEC,
    ADDRESS_ICD_BASE + 0x0000_0FD0,
    ADDRESS_ICD_BASE + 0x0000_0FD4,
    ADDRESS_ICD_BASE + 0x0000_0FD8,
    ADDRESS_ICD_BASE + 0x0000_0FDC,
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
