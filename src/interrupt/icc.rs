//! CPU interface.
//!
//! # How to use?
//!
//! ```ignore
//! ICC.toggle_enable(false);
//! ICC.set_interrupt_priority_filter(0b1111_1111);
//! ICC.toggle_enable(true);
//! ```
//!
//! Note that you also need to configure `GIC` to enable routing of interrupts to processor core.

use crate::common::memman::clear_address_bit;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;

pub const ADDRESS_ICC_BASE: u32 = 0xF8F0_0100;

/// TODO
pub struct Icc {
    pub address_interface_control: *mut u32,
    pub address_interrupt_priority_mask: *mut u32,
    pub address_interrupt_acknowledge: *mut u32,
    pub address_end_of_interrupt: *mut u32,
}

impl Icc {
    /// Enable or disable signalling of interrupts to target CPU.
    #[inline]
    pub fn toggle_enable(&self, enable: bool) {
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

    #[inline]
    pub fn set_interrupt_priority_filter(&self, value: u8) {
        write_address_bits(self.address_interrupt_priority_mask, 0..=7, value as u32);
    }

    // TODO: helpers, set priority filter to minimum, maximum etc

    // TODO: return structure
    #[inline]
    pub fn acknowledge_interrupt(&self) -> u32 {
        let iar = read_from_address(self.address_interrupt_acknowledge);
        iar
    }

    #[inline]
    pub fn complete_interrupt(&self, value: u32) {
        write_to_address(self.address_end_of_interrupt, value);
    }

    // TODO: other functions
}

pub static mut ICC: Icc = Icc {
    address_interface_control: (ADDRESS_ICC_BASE + 0x00) as *mut u32,
    address_interrupt_priority_mask: (ADDRESS_ICC_BASE + 0x04) as *mut u32,
    address_interrupt_acknowledge: (ADDRESS_ICC_BASE + 0x0C) as *mut u32,
    address_end_of_interrupt: (ADDRESS_ICC_BASE + 0x10) as *mut u32,
    // TODO: other registers
};
