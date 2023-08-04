//! CPU private watchdog timer.

use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;

#[derive(Clone, Copy)]
pub enum ReloadMode {
    SingleShot,
    AutoReload,
}

impl ReloadMode {
    pub fn as_bool(self) -> bool {
        match self {
            Self::SingleShot => false,
            Self::AutoReload => true,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TimerMode {
    TimerMode,
    WatchdogMode,
}

pub struct PrivateWatchdogTimer {
    address_load: *mut u32,
    address_counter: *mut u32,
    address_control: *mut u32,
    address_interrupt_status: *mut u32,
    address_reset_status: *mut u32,
    address_disable_watchdog: *mut u32,
}

impl PrivateWatchdogTimer {
    pub fn get_load(&self) -> u32 {
        read_from_address(self.address_load)
    }

    pub fn set_load(&self, value: u32) {
        write_to_address(self.address_load, value);
    }

    pub fn get_count(&self) -> u32 {
        read_from_address(self.address_counter)
    }

    pub fn set_count(&self, value: u32) {
        write_to_address(self.address_counter, value);
    }

    /// Enable or disable watchdog timer.
    pub fn toggle(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 0);
    }

    pub fn set_reload_mode(&self, mode: ReloadMode) {
        let action = if mode.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 1)
    }

    pub fn toggle_interrupt(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 2);
    }

    pub fn set_timer_mode(&self, mode: TimerMode) {
        match mode {
            TimerMode::TimerMode => {
                write_to_address(self.address_disable_watchdog, 0x1234_5678);
                write_to_address(self.address_disable_watchdog, 0x8765_4321);
            }
            TimerMode::WatchdogMode => {
                set_address_bit(self.address_control, 3);
            }
        }
    }

    pub fn set_prescaler(&self, value: u8) {
        write_address_bits(self.address_control, 8..=15, value as u32);
    }

    /// True if counter has reached zero in timer mode.
    pub fn read_interrupt_status(&self) -> bool {
        read_address_bit(self.address_interrupt_status, 0)
    }

    pub fn clear_interrupt(&self) {
        set_address_bit(self.address_interrupt_status, 0);
    }

    pub fn read_reset_status(&self) -> bool {
        read_address_bit(self.address_reset_status, 0)
    }

    pub fn clear_reset(&self) {
        set_address_bit(self.address_reset_status, 0);
    }
}

const ADDRESS_BASE: u32 = 0xF8F0_0000;
const ADDRESS_BASE_WATCHDOG: u32 = ADDRESS_BASE + 0x600;

pub static mut PRIVATE_WATCHDOG_TIMER: PrivateWatchdogTimer = PrivateWatchdogTimer {
    address_load: (ADDRESS_BASE_WATCHDOG + 0x20) as *mut u32,
    address_counter: (ADDRESS_BASE_WATCHDOG + 0x24) as *mut u32,
    address_control: (ADDRESS_BASE_WATCHDOG + 0x28) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE_WATCHDOG + 0x2C) as *mut u32,
    address_reset_status: (ADDRESS_BASE_WATCHDOG + 0x30) as *mut u32,
    address_disable_watchdog: (ADDRESS_BASE_WATCHDOG + 0x34) as *mut u32,
};
