//! Global 64-bit timer.

use crate::common::memman::clear_address_bit;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;

/// Application processing unit's base address.
const ADDRESS_BASE: u32 = 0xF8F0_0000;

/// Private timer mode.
#[derive(Clone, Copy)]
pub enum TimerMode {
    /// If counter reaches zero, event flag is set.
    SingleShot,

    /// If counter reaches zero, event flag is set and load value is copied to counter.
    AutoReload,
}

impl TimerMode {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::SingleShot => false,
            Self::AutoReload => true,
        }
    }
}

/// 64-bit counter's value.
#[derive(Clone, Copy)]
pub struct CounterValue {
    /// Lower 32 bits of counter's value.
    upper: u32,

    /// Upper 32 bits of counter's value.
    lower: u32,
}

/// Interface to global timer.
pub struct TimerGlobal {
    /// Lower 32 bits of counter's 64-bit value.
    address_counter_0: *mut u32,

    /// Upper 32 bits of counter's 64-bit value.
    address_counter_1: *mut u32,

    /// Global timer control register.
    address_control: *mut u32,

    /// Global timer interrupt status register.
    address_interrupt_status: *mut u32,
}

impl TimerGlobal {
    /// Set counter's value.
    #[inline]
    pub fn set_count(&self, value: CounterValue) {
        self.toggle_interrupt(false);
        write_to_address(self.address_counter_0, value.lower);
        write_to_address(self.address_counter_1, value.upper);
        self.toggle_interrupt(true);
    }

    /// Get counter's value.
    #[inline]
    #[must_use]
    pub fn get_count(&self) -> CounterValue {
        let upper_old = read_from_address(self.address_counter_1);
        let lower_old = read_from_address(self.address_counter_0);
        let upper_new = read_from_address(self.address_counter_1);
        if upper_old == upper_new {
            CounterValue {
                upper: upper_old,
                lower: lower_old,
            }
        } else {
            let lower_new = read_from_address(self.address_counter_0);
            CounterValue {
                upper: upper_new,
                lower: lower_new,
            }
        }
    }

    /// Configure timer's clock prescaler.
    #[inline]
    pub fn set_prescaler(&self, value: u8) {
        write_address_bits(self.address_control, 8..=15, value as u32);
    }

    /// Set counter mode.
    #[inline]
    pub fn set_mode(&self, mode: TimerMode) {
        let action = if mode.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 3);
    }

    /// Enable or disable timer interrupt.
    #[inline]
    pub fn toggle_interrupt(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 2);
    }

    /// Enable or disable comparison of counter's value with comparator's value.
    #[inline]
    pub fn toggle_comparator(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 1);
    }

    /// Enable or disable timer.
    #[inline]
    pub fn toggle(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 0);
    }

    /// Clear timer interrupt.
    #[inline]
    pub fn clear_interrupt(&self) {
        set_address_bit(self.address_interrupt_status, 0);
    }
}

/// Global timer.
pub static mut TIMER_GLOBAL: TimerGlobal = TimerGlobal {
    address_counter_0: (ADDRESS_BASE + 0x200) as *mut u32,
    address_counter_1: (ADDRESS_BASE + 0x204) as *mut u32,
    address_control: (ADDRESS_BASE + 0x208) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE + 0x20C) as *mut u32,
};
