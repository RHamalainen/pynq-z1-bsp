//! CPU interrupt interface.
//!
//! # How to use?
//!
//! ```ignore
//! ICC.enable(false);
//! ICC.set_interrupt_priority_filter(InterruptPriorityFilter::AllowAll);
//! ICC.enable(true);
//! ```
//!
//! You must configure [`GIC`](super::gic) to enable routing of interrupts to processor core.

use super::irq_numbers::Irq;
use super::irq_numbers::PpiIrq;
use super::irq_numbers::SgiIrq;
use super::irq_numbers::SpiIrq;
use super::InterruptPriority;
use crate::common::bitman::ReadBitwiseRange;
use crate::common::bitman::WriteBitwise;
use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;

/// Which interrupt priorities are handled.
#[derive(Clone, Copy)]
pub enum InterruptPriorityFilter {
    /// Do not handle any interrupt.
    AllowNone,

    /// Handle interrupts to given priority.
    AllowSome { lowest_allowed: InterruptPriority },

    /// Handle all interrupts.
    AllowAll,
}

impl InterruptPriorityFilter {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::AllowNone => 0b0000_0000,
            Self::AllowSome { lowest_allowed } => {
                let priority = lowest_allowed.as_u8();
                priority.saturating_add(1)
            }
            Self::AllowAll => 0b1111_1111,
        }
    }
}

/// Information provided from `GIC` when acknowledging an interrupt.
#[derive(Clone, Copy)]
pub enum InterruptAcknowledge {
    Sgi {
        /// Interrupt identifier.
        sgi: SgiIrq,

        /// CPU identifier that requested the interrupt.
        cpu_id: u32,
    },
    Ppi {
        /// Interrupt identifier.
        ppi: PpiIrq,
    },
    Spi {
        /// Interrupt identifier.
        spi: SpiIrq,
    },
}

impl InterruptAcknowledge {
    pub fn from_u32(value: u32) -> Self {
        let interrupt_id = value.read_bits(0..=9);
        match Irq::from_u32(interrupt_id) {
            Irq::Sgi(sgi) => {
                // Solve which processor requested this interrupt.
                let cpu_id = value.read_bits(10..=12);
                Self::Sgi { sgi, cpu_id }
            }
            Irq::Ppi(ppi) => Self::Ppi { ppi },
            Irq::Spi(spi) => Self::Spi { spi },
        }
    }

    pub fn as_u32(self) -> u32 {
        let mut result = 0;
        match self {
            Self::Sgi { sgi, cpu_id } => {
                let interrupt_id = sgi.as_u32();
                result = result.write_bits(0, interrupt_id, 10);
                result = result.write_bits(10, cpu_id, 3);
            }
            Self::Ppi { ppi } => {
                let interrupt_id = ppi.as_u32();
                result = result.write_bits(0, interrupt_id, 10);
            }
            Self::Spi { spi } => {
                let interrupt_id = spi.as_u32();
                result = result.write_bits(0, interrupt_id, 10);
            }
        }
        result
    }
}

/// CPU interrupt interface.
///
/// `ICC` is responsible for signaling interrupts to CPU.
pub struct Icc {
    address_interface_control: *mut u32,
    address_interrupt_priority_mask: *mut u32,
    address_binary_point: *mut u32,
    address_interrupt_acknowledge: *mut u32,
    address_end_of_interrupt: *mut u32,
    address_running_priority: *mut u32,
    address_highest_pending_interrupt: *mut u32,
    address_aliased_non_secure_binary_point_register: *mut u32,
    address_implementer_identification: *mut u32,
}

impl Icc {
    // TODO: other controls

    /// Enable or disable `ICC`.
    #[inline]
    pub fn toggle(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        // Secure interrupts.
        action(self.address_interface_control, 0);
        // Non-secure interrupts.
        action(self.address_interface_control, 1);
    }

    /// Set which interrupts are handled.
    #[inline]
    pub fn set_interrupt_priority_filter(&self, value: InterruptPriorityFilter) {
        let value = value.as_u8();
        write_address_bits(self.address_interrupt_priority_mask, 0..=7, value as u32);
    }

    // TODO: helpers, set priority filter to minimum, maximum etc

    // TODO: what is this?
    pub fn set_binary_point(&self, value: u8) {
        // TODO: value is 2 bits
        write_address_bits(self.address_binary_point, 0..=2, value as u32)
    }

    // TODO: what is this?
    pub fn get_binary_point(&self) -> u8 {
        // TODO: value is 2 bits
        read_address_bits(self.address_binary_point, 0..=2) as u8
    }

    /// Accept interrupt from `GIC`.
    ///
    /// After acknowledgement, the `GIC` updates interrupt's state.
    /// Next state is either *active* or *active and pending*.
    #[inline]
    pub fn acknowledge_interrupt(&self) -> InterruptAcknowledge {
        // IAR must be read once.
        let iar = read_from_address(self.address_interrupt_acknowledge);
        InterruptAcknowledge::from_u32(iar)
    }

    /// Signal to `GIC` that interrupt handling is complete.
    #[inline]
    pub fn complete_interrupt(&self, value: InterruptAcknowledge) {
        // EOIR must be written once.
        let value = value.as_u32();
        write_to_address(self.address_end_of_interrupt, value);
    }

    /// Get priority of highest priority interrupt that is active.
    pub fn running_priority(&self) -> InterruptPriority {
        let value = read_address_bits(self.address_running_priority, 0..=7);
        InterruptPriority::from_u8(value as u8)
    }

    /// Set priority of highest priority interrupt that is active.
    pub fn set_running_priority(&self, value: InterruptPriority) {
        let value = value.as_u8();
        write_address_bits(self.address_running_priority, 0..=7, value as u32)
    }

    /// Get highest pending interrupt.
    // TODO: is it possible to alias IAR to HPI?
    pub fn highest_pending_interrupt(&self) -> InterruptAcknowledge {
        let iar = read_from_address(self.address_highest_pending_interrupt);
        InterruptAcknowledge::from_u32(iar)
    }

    /// Set highest pending interrupt.
    // TODO: is it possible to alias IAR to HPI?
    pub fn set_highest_pending_interrupt(&self, iar: InterruptAcknowledge) {
        let value = iar.as_u32();
        write_to_address(self.address_highest_pending_interrupt, value);
    }

    // TODO: what is this?
    pub fn set_non_secure_binary_point(&self, value: u8) {
        // TODO: value is 2 bits
        write_address_bits(
            self.address_aliased_non_secure_binary_point_register,
            0..=2,
            value as u32,
        )
    }

    // TODO: what is this?
    pub fn get_non_secure_binary_point(&self) -> u8 {
        // TODO: value is 2 bits
        read_address_bits(self.address_aliased_non_secure_binary_point_register, 0..=2) as u8
    }

    // TODO: maybe give identification as a struct

    #[inline]
    #[must_use]
    pub fn implementer(&self) -> u32 {
        read_address_bits(self.address_implementer_identification, 0..=11)
    }

    #[inline]
    #[must_use]
    pub fn revision_number(&self) -> u32 {
        read_address_bits(self.address_implementer_identification, 12..=15)
    }

    #[inline]
    #[must_use]
    pub fn architecture_version(&self) -> u32 {
        read_address_bits(self.address_implementer_identification, 16..=19)
    }

    #[inline]
    #[must_use]
    pub fn part_number(&self) -> u32 {
        read_address_bits(self.address_implementer_identification, 20..=31)
    }
}

const ADDRESS_BASE: u32 = 0xF8F0_0100;

/// CPU interrupt interface.
pub static mut ICC: Icc = Icc {
    address_interface_control: (ADDRESS_BASE + 0x00) as *mut u32,
    address_interrupt_priority_mask: (ADDRESS_BASE + 0x04) as *mut u32,
    address_binary_point: (ADDRESS_BASE + 0x08) as *mut u32,
    address_interrupt_acknowledge: (ADDRESS_BASE + 0x0C) as *mut u32,
    address_end_of_interrupt: (ADDRESS_BASE + 0x10) as *mut u32,
    address_running_priority: (ADDRESS_BASE + 0x14) as *mut u32,
    address_highest_pending_interrupt: (ADDRESS_BASE + 0x18) as *mut u32,
    address_aliased_non_secure_binary_point_register: (ADDRESS_BASE + 0x1C) as *mut u32,
    address_implementer_identification: (ADDRESS_BASE + 0xFC) as *mut u32,
};
