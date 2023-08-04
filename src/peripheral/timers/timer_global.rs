//! Global 64-bit timer.
//!
//! # How to use?
//!
//! ```ignore
//! TIMER_GLOBAL.toggle(false);
//! TIMER_GLOBAL.clear_interrupt();
//! TIMER_GLOBAL.set_mode(TimerModeGlobal::AutoReload);
//! TIMER_GLOBAL.set_prescaler(0);
//! TIMER_GLOBAL.set_count(CounterValue { upper: 0, lower: 0 });
//! TIMER_GLOBAL.toggle_interrupt(false);
//! TIMER_GLOBAL.toggle_comparator(false);
//! COMPARATOR.set_comparator_value(CounterValue {
//!     upper: 0,
//!     lower: 10_000_000,
//! });
//! COMPARATOR.set_auto_increment_value(10_000_000);
//! TIMER_GLOBAL.toggle_comparator(true);
//! TIMER_GLOBAL.toggle(true);
//! ```

use core::ops::Not;

use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;

/// PYNQ-Z1 provides 50 MHz clock to Zynq's PS_CLK input.
/// This enables the processor to operate at maximum frequency of 650 MHz.
/// Global timer is clocked at half of the CPU's frequency, in this case 325 MHz.
/// Thus increments per µsecond := 325 MHz / 1_000_000 = 325.
const INCREMENTS_PER_USECOND: u32 = 325;

/// Global timer mode.
#[derive(Clone, Copy)]
pub enum TimerMode {
    /// If counter reaches comparator value, event flag is set.
    SingleShot,

    /// If counter reaches comparator value, comparator is auto-incremented.
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

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::AutoReload
        } else {
            Self::SingleShot
        }
    }
}

/// 64-bit counter's value.
#[derive(Clone, Copy, Eq)]
pub struct CounterValue {
    /// Lower 32 bits of counter's value.
    pub upper: u32,

    /// Upper 32 bits of counter's value.
    pub lower: u32,
}

impl core::ops::Add for CounterValue {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut upper: u32 = 0;
        let lower = match self.lower.checked_add(other.lower) {
            Some(lower) => lower,
            None => {
                upper += 1;
                let steps_to_maximum = u32::MAX - self.lower;
                other.lower - steps_to_maximum
            }
        };
        // Upper 32 bits can overflow freely.
        upper = upper + self.upper + other.upper;
        Self { upper, lower }
    }
}

impl Ord for CounterValue {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering;

        if self.upper < other.upper {
            Ordering::Less
        } else if self.upper == other.upper {
            if self.lower < other.lower {
                Ordering::Less
            } else if self.lower == other.lower {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for CounterValue {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CounterValue {
    fn eq(&self, other: &Self) -> bool {
        let uppers_equal = self.upper == other.upper;
        let lowers_equal = self.lower == other.lower;
        uppers_equal && lowers_equal
    }
}

/*
impl PartialOrd for CounterValue {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
*/

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
        let interrupt_enabled = self.is_interrupt_enabled();
        if interrupt_enabled {
            self.toggle_interrupt(false);
        }
        write_to_address(self.address_counter_0, value.lower);
        write_to_address(self.address_counter_1, value.upper);
        if interrupt_enabled {
            self.toggle_interrupt(true);
        }
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

    /// Set timer's clock prescaler.
    #[inline]
    pub fn set_prescaler(&self, value: u8) {
        write_address_bits(self.address_control, 8..=15, value as u32);
    }

    /// Get timer's clock prescaler.
    pub fn get_prescaler(&self) -> u8 {
        let value = read_address_bits(self.address_control, 8..=15);
        value as u8
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

    /// Get counter mode.
    pub fn get_mode(&self) -> TimerMode {
        let value = read_address_bit(self.address_control, 3);
        TimerMode::from_bool(value)
    }

    /// True if interrupt is enabled.
    pub fn is_interrupt_enabled(&self) -> bool {
        read_address_bit(self.address_control, 2)
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

    /// True if comparator is enabled.
    pub fn is_comparator_enabled(&self) -> bool {
        read_address_bit(self.address_control, 1)
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

    /// True if timer is enabled.
    pub fn is_enabled(&self) -> bool {
        read_address_bit(self.address_control, 0)
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

    /// True if counter has reached comparator's value.
    pub fn interrupt_status(&self) -> bool {
        read_address_bit(self.address_interrupt_status, 0)
    }

    /// Clear timer interrupt.
    #[inline]
    pub fn clear_interrupt(&self) {
        set_address_bit(self.address_interrupt_status, 0);
    }

    /// Reset peripheral.
    pub fn reset(&self) -> Result<(), ()> {
        write_to_address(self.address_control, 0);
        write_to_address(self.address_counter_0, 0);
        write_to_address(self.address_counter_1, 0);
        write_to_address(self.address_interrupt_status, 0xFFFF_FFFF);
        if read_from_address(self.address_control) != 0 {
            return Err(());
        }
        if read_from_address(self.address_counter_0) != 0 {
            return Err(());
        }
        if read_from_address(self.address_counter_1) != 0 {
            return Err(());
        }
        if read_from_address(self.address_interrupt_status) != 0 {
            return Err(());
        }
        Ok(())
    }

    /// Sleep given microseconds.
    ///
    /// This function blocks.
    pub fn usleep(&self, useconds: u32) -> Result<(), ()> {
        if self.is_enabled().not() {
            return Err(());
        }
        // TODO: maybe also check if comparator is enabled?
        let mut count_now = self.get_count();
        let count_end = count_now
            + CounterValue {
                // TODO: this is not optimal...
                lower: useconds * INCREMENTS_PER_USECOND,
                upper: 0,
            };
        while count_now < count_end {
            count_now = self.get_count();
        }
        Ok(())
    }
}

pub struct Comparator {
    address_comparator_lower: *mut u32,
    address_comparator_upper: *mut u32,
    address_auto_increment: *mut u32,
}

impl Comparator {
    pub fn get_comparator_value(&self) -> CounterValue {
        let lower = read_from_address(self.address_comparator_lower);
        let upper = read_from_address(self.address_comparator_upper);
        CounterValue { lower, upper }
    }

    pub fn set_comparator_value(&self, value: CounterValue) {
        write_to_address(self.address_comparator_lower, value.lower);
        write_to_address(self.address_comparator_upper, value.upper);
    }

    pub fn get_auto_increment_value(&self) -> u32 {
        read_from_address(self.address_auto_increment)
    }

    pub fn set_auto_increment_value(&self, value: u32) {
        write_to_address(self.address_auto_increment, value);
    }
}

/// Application processing unit's base address.
const ADDRESS_BASE: u32 = 0xF8F0_0000;

/// Global timer.
pub static mut TIMER_GLOBAL: TimerGlobal = TimerGlobal {
    address_counter_0: (ADDRESS_BASE + 0x200) as *mut u32,
    address_counter_1: (ADDRESS_BASE + 0x204) as *mut u32,
    address_control: (ADDRESS_BASE + 0x208) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE + 0x20C) as *mut u32,
};

pub static mut COMPARATOR: Comparator = Comparator {
    address_comparator_lower: (ADDRESS_BASE + 0x210) as *mut u32,
    address_comparator_upper: (ADDRESS_BASE + 0x214) as *mut u32,
    address_auto_increment: (ADDRESS_BASE + 0x218) as *mut u32,
};
