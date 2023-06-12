//! Processor core's private timer.

use crate::common::memman::clear_address_bit;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;

/// Application processing unit's base address.
const ADDRESS_BASE: u32 = 0xF8F0_0000;

/// PYNQ-Z1 provides 50 MHz clock to Zynq's PS_CLK input.
/// This enables the processor to operate at maximum frequency of 650 MHz.
/// Private timer is clocked at half of the CPU's frequency, in this case 325 MHz.
/// Thus decrements per µsecond := 325 MHz / 1_000_000 = 325;
const DECREMENTS_PER_USECOND: u32 = 325;

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

/// Interface to private timer.
pub struct TimerPrivate {
    /// Contains the value copied to counter register when it decrements down to zero with auto reload mode enabled.
    address_load: *mut u32,

    /// Timer's current count.
    ///
    /// If timer is enabled, then value is decremented.
    /// If value reaches zero and timer interrupt is enabled, then timer interrupt status event flag is set.
    address_counter: *mut u32,

    /// Private timer control register.
    address_control: *mut u32,

    /// Private timer interrupt status register.
    address_interrupt_status: *mut u32,
}

impl TimerPrivate {
    /// Set counter's start value.
    #[inline]
    pub fn set_load(&self, value: u32) {
        write_to_address(self.address_load, value);
    }

    /// Get counter's start value.
    #[inline]
    #[must_use]
    pub fn get_load(&self) -> u32 {
        read_from_address(self.address_load)
    }

    /// Set counter's current value.
    #[inline]
    pub fn set_count(&self, value: u32) {
        write_to_address(self.address_counter, value);
    }

    /// Get counter's current value.
    #[inline]
    #[must_use]
    pub fn get_count(&self) -> u32 {
        read_from_address(self.address_counter)
    }

    /// Configure timer's clock prescaler.
    #[inline]
    pub fn set_prescaler(&self, value: u8) {
        write_address_bits(self.address_control, 8..=15, value as u32);
    }

    /// Enable or disable timer interrput.
    #[inline]
    pub fn toggle_interrupt(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 2);
    }

    /// Configure timer mode.
    #[inline]
    pub fn set_mode(&self, mode: TimerMode) {
        let action = if mode.as_bool() {
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

    pub fn usleep(&self, useconds: u32) {
        // TODO: give error if timer is not enabled...

        let load = self.get_load();
        let mut decrements_now = 0;
        let decrements_end = useconds * DECREMENTS_PER_USECOND;
        //let decrements_end = 1_000_000_000;

        let mut count_upper = self.get_count();
        let mut count_lower = count_upper;

        // Loop until enough decrements have been counted.
        while decrements_now <= decrements_end {
            // Count upper should be greater than count lower.
            let difference = if count_lower < count_upper {
                // Counter has progressed.
                let difference = count_upper - count_lower;
                // Swap upper and lower values.
                //let temp = count_upper;
                //count_upper = count_lower;
                //count_lower = temp;
                core::mem::swap(&mut count_upper, &mut count_lower);
                difference
            } else if count_lower == count_upper {
                // Counter has not progressed.
                0
            } else {
                // Counter has reached zero and loaded new value.
                let difference = count_upper + (load - count_lower);
                // Swap upper and lower values.
                //let temp = count_upper;
                //count_upper = count_lower;
                //count_lower = temp;
                core::mem::swap(&mut count_upper, &mut count_lower);
                difference
            };
            decrements_now += difference;
            count_lower = self.get_count();
        }
    }
}

/// Private timer.
pub static mut TIMER_PRIVATE: TimerPrivate = TimerPrivate {
    address_load: (ADDRESS_BASE + 0x600) as *mut u32,
    address_counter: (ADDRESS_BASE + 0x604) as *mut u32,
    address_control: (ADDRESS_BASE + 0x608) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE + 0x60C) as *mut u32,
};
