//! General purpose input and output.
//!
//! GPIO registers are divided to four banks.
//! - Bank 0 controls 32 MIO pins.
//! - Bank 1 controls 22 MIO pins.
//! - Bank 2 controls 32 EMIO pins.
//! - Bank 3 controls 32 EMIO pins.

use crate::common::memman::{clear_address_bit, read_address_bit, set_address_bit};
use core::ops::{RangeInclusive, Rem};

/// Base address for memory mapped GPIO.
pub const ADDRESS_GPIO_BASE: u32 = 0xE000_A000;

/// GPIO pin direction.
#[derive(Clone, Copy)]
pub enum PinDirection {
    /// Pin can accept voltage.
    Input,

    /// Pin can drive voltage.
    Output,
}

/// GPIO pin interrupt type.
#[derive(Clone, Copy)]
pub enum InterruptType {
    /// Interrupt is level-sensitive.
    Level,

    /// Interrupt is edge-sensitive.
    Edge,
}

impl InterruptType {
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

/// GPIO pin interrupt polarity.
#[derive(Clone, Copy)]
pub enum InterruptPolarity {
    /// Interrupt triggers on low voltage or falling edge.
    ActiveLowOrFallingEdge,

    /// Interrupt triggers on high voltage or rising edge.
    ActiveHighOrRisingEdge,
}

impl InterruptPolarity {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::ActiveLowOrFallingEdge => false,
            Self::ActiveHighOrRisingEdge => true,
        }
    }
}

/// GPIO pin interrupt edge triggering mode.
#[derive(Clone, Copy)]
pub enum InterruptEdgeTriggeringMode {
    /// Trigger on single edge.
    Single,

    /// Trigger on both edges.
    Both,
}

impl InterruptEdgeTriggeringMode {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Single => false,
            Self::Both => true,
        }
    }
}

/// Interface for a GPIO bank.
#[derive(Clone, Copy)]
pub struct Bank {
    pub address_maskable_output_data_lsw: *mut u32,
    pub address_maskable_output_data_msw: *mut u32,
    pub address_output_data: *mut u32,
    pub address_input_data: *mut u32,
    pub address_direction_mode: *mut u32,
    pub address_output_enable: *mut u32,
    pub address_interrupt_mask_status: *mut u32,
    pub address_interrupt_enable: *mut u32,
    pub address_interrupt_disable: *mut u32,
    pub address_interrupt_status: *mut u32,
    pub address_interrupt_type: *mut u32,
    pub address_interrupt_polarity: *mut u32,
    pub address_interrupt_any_edge_sensitive: *mut u32,
}

/// Interface for a GPIO peripheral.
pub struct Gpio {
    pub mio_pin_range: RangeInclusive<u32>,
    pub emio_pin_range: RangeInclusive<u32>,
    pub banks: [Bank; 4],
    pub bank_pin_ranges: [RangeInclusive<u32>; 4],
    pub mio_bank_indices: RangeInclusive<u32>,
    pub emio_bank_indices: RangeInclusive<u32>,
}

impl Gpio {
    /// Get MIO bank by pin index.
    ///
    /// # Panics
    ///
    /// Invalid index.
    #[inline]
    #[must_use]
    pub fn get_mio_bank_by_pin_index(&self, index: u32) -> &Bank {
        for bank_index in self.mio_bank_indices.clone() {
            let bank_pin_range = &self.bank_pin_ranges[bank_index as usize];
            if bank_pin_range.contains(&index) {
                return &self.banks[bank_index as usize];
            }
        }
        panic!("Invalid MIO index: {}", index);
    }

    /// Get EMIO bank by pin index.
    ///
    /// # Panics
    ///
    /// Invalid index.
    #[inline]
    #[must_use]
    pub fn get_emio_bank_by_pin_index(&self, index: u32) -> &Bank {
        for bank_index in self.emio_bank_indices.clone() {
            let bank_pin_range = &self.bank_pin_ranges[bank_index as usize];
            if bank_pin_range.contains(&index) {
                return &self.banks[bank_index as usize];
            }
        }
        panic!("Invalid EMIO index: {}", index);
    }

    /// Read MIO pin input.
    #[inline]
    #[must_use]
    pub fn read_mio_input(&self, index: u32) -> bool {
        let bank = self.get_mio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        read_address_bit(bank.address_input_data, bit_index)
    }

    /// Read EMIO pin input.
    #[inline]
    #[must_use]
    pub fn read_emio_input(&self, index: u32) -> bool {
        let bank = self.get_emio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        read_address_bit(bank.address_input_data, bit_index)
    }

    /// Set MIO pin direction.
    #[inline]
    pub fn set_mio_direction(&self, index: u32, direction: PinDirection) {
        let bank = self.get_mio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = match direction {
            PinDirection::Input => clear_address_bit,
            PinDirection::Output => set_address_bit,
        };
        action(bank.address_direction_mode, bit_index);
    }

    /// Set EMIO pin direction.
    #[inline]
    pub fn set_emio_direction(&self, index: u32, direction: PinDirection) {
        let bank = self.get_emio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = match direction {
            PinDirection::Input => clear_address_bit,
            PinDirection::Output => set_address_bit,
        };
        action(bank.address_direction_mode, bit_index);
    }

    /// Enable or disable MIO pin interrupts.
    #[inline]
    pub fn toggle_mio_interrupt(&self, index: u32, enabled: bool) {
        let bank = self.get_mio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let address = if enabled {
            bank.address_interrupt_enable
        } else {
            bank.address_interrupt_disable
        };
        set_address_bit(address, bit_index);
    }

    /// Enable or disable EMIO pin interrupts.
    #[inline]
    pub fn toggle_emio_interrupt(&self, index: u32, enabled: bool) {
        let bank = self.get_emio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let address = if enabled {
            bank.address_interrupt_enable
        } else {
            bank.address_interrupt_disable
        };
        set_address_bit(address, bit_index);
    }

    /// Read MIO pin interrupt status.
    #[inline]
    #[must_use]
    pub fn read_mio_interrupt_status(&self, index: u32) -> bool {
        let bank = self.get_mio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        read_address_bit(bank.address_interrupt_status, bit_index)
    }

    /// Read EMIO pin interrupt status.
    #[inline]
    #[must_use]
    pub fn read_emio_interrupt_status(&self, index: u32) -> bool {
        let bank = self.get_emio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        read_address_bit(bank.address_interrupt_status, bit_index)
    }

    /// Set MIO pin interrupt type.
    #[inline]
    pub fn set_mio_interrupt_type(&self, index: u32, value: InterruptType) {
        let bank = self.get_mio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = if value.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(bank.address_interrupt_type, bit_index);
    }

    /// Set EMIO pin interrupt type.
    #[inline]
    pub fn set_emio_interrupt_type(&self, index: u32, value: InterruptType) {
        let bank = self.get_emio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = if value.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(bank.address_interrupt_type, bit_index);
    }

    /// Set MIO pin interrupt polarity.
    #[inline]
    pub fn set_mio_interrupt_polarity(&self, index: u32, value: InterruptPolarity) {
        let bank = self.get_mio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = if value.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(bank.address_interrupt_polarity, bit_index);
    }

    /// Set EMIO pin interrupt polarity.
    #[inline]
    pub fn set_emio_interrupt_polarity(&self, index: u32, value: InterruptPolarity) {
        let bank = self.get_emio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = if value.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(bank.address_interrupt_polarity, bit_index);
    }

    /// Set MIO pin edge triggering mode.
    #[inline]
    pub fn set_mio_edge_triggering_mode(&self, index: u32, value: InterruptEdgeTriggeringMode) {
        let bank = self.get_mio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = if value.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(bank.address_interrupt_any_edge_sensitive, bit_index);
    }

    /// Set EMIO pin edge triggering mode.
    #[inline]
    pub fn set_emio_edge_triggering_mode(&self, index: u32, value: InterruptEdgeTriggeringMode) {
        let bank = self.get_emio_bank_by_pin_index(index);
        let bit_index = index.rem(32);
        let action = if value.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(bank.address_interrupt_any_edge_sensitive, bit_index);
    }
}

/// GPIO bank 0 base address.
const ADDRESS_BANK0_BASE: u32 = ADDRESS_GPIO_BASE + 0x204;
/// GPIO bank 1 base address.
const ADDRESS_BANK1_BASE: u32 = ADDRESS_GPIO_BASE + 0x244;
/// GPIO bank 2 base address.
const ADDRESS_BANK2_BASE: u32 = ADDRESS_GPIO_BASE + 0x284;
/// GPIO bank 3 base address.
const ADDRESS_BANK3_BASE: u32 = ADDRESS_GPIO_BASE + 0x2C4;

/// GPIO bank 0.
static mut BANK0: Bank = Bank {
    // These addresses are not grouped by the bank.
    address_maskable_output_data_lsw: (ADDRESS_GPIO_BASE + 0x000) as *mut u32,
    address_maskable_output_data_msw: (ADDRESS_GPIO_BASE + 0x004) as *mut u32,
    address_output_data: (ADDRESS_GPIO_BASE + 0x040) as *mut u32,
    address_input_data: (ADDRESS_GPIO_BASE + 0x060) as *mut u32,
    // These addresses are grouped by the bank.
    address_direction_mode: (ADDRESS_BANK0_BASE + 0x000) as *mut u32,
    address_output_enable: (ADDRESS_BANK0_BASE + 0x004) as *mut u32,
    address_interrupt_mask_status: (ADDRESS_BANK0_BASE + 0x008) as *mut u32,
    address_interrupt_enable: (ADDRESS_BANK0_BASE + 0x00C) as *mut u32,
    address_interrupt_disable: (ADDRESS_BANK0_BASE + 0x010) as *mut u32,
    address_interrupt_status: (ADDRESS_BANK0_BASE + 0x014) as *mut u32,
    address_interrupt_type: (ADDRESS_BANK0_BASE + 0x018) as *mut u32,
    address_interrupt_polarity: (ADDRESS_BANK0_BASE + 0x01C) as *mut u32,
    address_interrupt_any_edge_sensitive: (ADDRESS_BANK0_BASE + 0x020) as *mut u32,
};

/// GPIO bank 1.
static mut BANK1: Bank = Bank {
    // These addresses are not grouped by the bank.
    address_maskable_output_data_lsw: (ADDRESS_GPIO_BASE + 0x008) as *mut u32,
    address_maskable_output_data_msw: (ADDRESS_GPIO_BASE + 0x00C) as *mut u32,
    address_output_data: (ADDRESS_GPIO_BASE + 0x044) as *mut u32,
    address_input_data: (ADDRESS_GPIO_BASE + 0x064) as *mut u32,
    // These addresses are grouped by the bank.
    address_direction_mode: (ADDRESS_BANK1_BASE + 0x000) as *mut u32,
    address_output_enable: (ADDRESS_BANK1_BASE + 0x004) as *mut u32,
    address_interrupt_mask_status: (ADDRESS_BANK1_BASE + 0x008) as *mut u32,
    address_interrupt_enable: (ADDRESS_BANK1_BASE + 0x00C) as *mut u32,
    address_interrupt_disable: (ADDRESS_BANK1_BASE + 0x010) as *mut u32,
    address_interrupt_status: (ADDRESS_BANK1_BASE + 0x014) as *mut u32,
    address_interrupt_type: (ADDRESS_BANK1_BASE + 0x018) as *mut u32,
    address_interrupt_polarity: (ADDRESS_BANK1_BASE + 0x01C) as *mut u32,
    address_interrupt_any_edge_sensitive: (ADDRESS_BANK1_BASE + 0x020) as *mut u32,
};

/// GPIO bank 2.
static mut BANK2: Bank = Bank {
    // These addresses are not grouped by the bank.
    address_maskable_output_data_lsw: (ADDRESS_GPIO_BASE + 0x010) as *mut u32,
    address_maskable_output_data_msw: (ADDRESS_GPIO_BASE + 0x014) as *mut u32,
    address_output_data: (ADDRESS_GPIO_BASE + 0x048) as *mut u32,
    address_input_data: (ADDRESS_GPIO_BASE + 0x068) as *mut u32,
    // These addresses are grouped by the bank.
    address_direction_mode: (ADDRESS_BANK2_BASE + 0x000) as *mut u32,
    address_output_enable: (ADDRESS_BANK2_BASE + 0x004) as *mut u32,
    address_interrupt_mask_status: (ADDRESS_BANK2_BASE + 0x008) as *mut u32,
    address_interrupt_enable: (ADDRESS_BANK2_BASE + 0x00C) as *mut u32,
    address_interrupt_disable: (ADDRESS_BANK2_BASE + 0x010) as *mut u32,
    address_interrupt_status: (ADDRESS_BANK2_BASE + 0x014) as *mut u32,
    address_interrupt_type: (ADDRESS_BANK2_BASE + 0x018) as *mut u32,
    address_interrupt_polarity: (ADDRESS_BANK2_BASE + 0x01C) as *mut u32,
    address_interrupt_any_edge_sensitive: (ADDRESS_BANK2_BASE + 0x020) as *mut u32,
};

/// GPIO bank 3.
static mut BANK3: Bank = Bank {
    // These addresses are not grouped by the bank.
    address_maskable_output_data_lsw: (ADDRESS_GPIO_BASE + 0x018) as *mut u32,
    address_maskable_output_data_msw: (ADDRESS_GPIO_BASE + 0x01C) as *mut u32,
    address_output_data: (ADDRESS_GPIO_BASE + 0x04C) as *mut u32,
    address_input_data: (ADDRESS_GPIO_BASE + 0x06C) as *mut u32,
    // These addresses are grouped by the bank.
    address_direction_mode: (ADDRESS_BANK3_BASE + 0x000) as *mut u32,
    address_output_enable: (ADDRESS_BANK3_BASE + 0x004) as *mut u32,
    address_interrupt_mask_status: (ADDRESS_BANK3_BASE + 0x008) as *mut u32,
    address_interrupt_enable: (ADDRESS_BANK3_BASE + 0x00C) as *mut u32,
    address_interrupt_disable: (ADDRESS_BANK3_BASE + 0x010) as *mut u32,
    address_interrupt_status: (ADDRESS_BANK3_BASE + 0x014) as *mut u32,
    address_interrupt_type: (ADDRESS_BANK3_BASE + 0x018) as *mut u32,
    address_interrupt_polarity: (ADDRESS_BANK3_BASE + 0x01C) as *mut u32,
    address_interrupt_any_edge_sensitive: (ADDRESS_BANK3_BASE + 0x020) as *mut u32,
};

/// GPIO peripheral.
pub static mut GPIO: Gpio = unsafe {
    Gpio {
        mio_pin_range: 0..=53,
        emio_pin_range: 0..=63,
        banks: [BANK0, BANK1, BANK2, BANK3],
        bank_pin_ranges: [0..=31, 32..=53, 0..=31, 32..=63],
        mio_bank_indices: 0..=1,
        emio_bank_indices: 2..=3,
    }
};
