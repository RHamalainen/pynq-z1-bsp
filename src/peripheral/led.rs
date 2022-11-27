//! Light-emitting diodes.

use crate::common::memman::{clear_address_bit, set_address_bit};

pub mod rgb;

/// Interface for board LED.
pub struct Led {
    /// Address to LED.
    address: *mut u32,
    /// Bit index to LED.
    index: u32,
}

impl Led {
    /// Enable or disable LED.
    #[inline]
    pub fn toggle(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address, self.index);
    }
}

/// Interface for board LEDs.
pub struct Leds {
    /// Board LEDs.
    pub leds: [Led; 4],
}

impl Leds {
    /// Disable all LEDs.
    #[inline]
    pub fn clear(&self) {
        for led in &self.leds {
            led.toggle(false);
        }
    }
}
