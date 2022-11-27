//! Light-emitting diode with colors.

use crate::common::memman::{clear_address_bit, set_address_bit};

/// Interface to board RGB LED.
pub struct RgbLed {
    address_red: *mut u32,
    address_green: *mut u32,
    address_blue: *mut u32,
    index_red: u32,
    index_green: u32,
    index_blue: u32,
}

impl RgbLed {
    /// Get addresses.
    #[inline]
    #[must_use]
    pub fn addresses(&self) -> [*mut u32; 3] {
        [self.address_red, self.address_green, self.address_blue]
    }

    /// Get indices.
    #[inline]
    #[must_use]
    pub fn indices(&self) -> [u32; 3] {
        [self.index_red, self.index_green, self.index_blue]
    }

    /// Enable or disable red color.
    #[inline]
    pub fn toggle_red(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_red, self.index_red);
    }

    /// Enable or disable green color.
    #[inline]
    pub fn toggle_green(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_green, self.index_green);
    }

    /// Enable or disable blue color.
    #[inline]
    pub fn toggle_blue(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_blue, self.index_blue);
    }

    /// Enable or disable colors.
    #[inline]
    pub fn toggle(&self, enable_red: bool, enable_green: bool, enable_blue: bool) {
        self.toggle_red(enable_red);
        self.toggle_green(enable_green);
        self.toggle_blue(enable_blue);
    }

    /// Disable RGB LED.
    #[inline]
    pub fn clear(&self) {
        self.toggle(false, false, false);
    }
}

/// Interface to board RGB LEDs.
pub struct RgbLeds {
    leds: [RgbLed; 2],
}

impl RgbLeds {
    // TODO

    /// Disable all board RGB LEDs.
    #[inline]
    pub fn clear(&self) {
        for led in &self.leds {
            led.clear();
        }
    }
}
