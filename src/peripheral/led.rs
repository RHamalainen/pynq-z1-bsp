//! Light-emitting diodes.

pub mod rgb;

pub enum LedIndex {
    Led0,
    Led1,
    Led2,
    Led3,
}

impl LedIndex {
    pub fn to_u32(self) -> u32 {
        match self {
            Self::Led0 => 0,
            Self::Led1 => 1,
            Self::Led2 => 2,
            Self::Led3 => 3,
        }
    }
}

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
        use crate::common::memman::clear_address_bit;
        use crate::common::memman::set_address_bit;

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
    /// Configure LEDs base address and order.
    pub fn configure(address: *mut u32, indices: [LedIndex; 4]) -> Self {
        let mut led0_index = None;
        let mut led1_index = None;
        let mut led2_index = None;
        let mut led3_index = None;
        for (address_index, led_index) in indices.iter().enumerate() {
            let some_address_index = Some(address_index as u32);
            match led_index {
                LedIndex::Led0 => {
                    led0_index = some_address_index;
                }
                LedIndex::Led1 => {
                    led1_index = some_address_index;
                }
                LedIndex::Led2 => {
                    led2_index = some_address_index;
                }
                LedIndex::Led3 => {
                    led3_index = some_address_index;
                }
            }
        }
        Self {
            leds: [
                Led {
                    address,
                    index: led0_index.unwrap(),
                },
                Led {
                    address,
                    index: led1_index.unwrap(),
                },
                Led {
                    address,
                    index: led2_index.unwrap(),
                },
                Led {
                    address,
                    index: led3_index.unwrap(),
                },
            ],
        }
    }

    /// Get LED by LED index.
    pub fn get_led(&self, index: LedIndex) -> &Led {
        let index = index.to_u32();
        &self.leds[index as usize]
    }

    /// Enable or disable LED.
    pub fn toggle_led(&self, index: LedIndex, enable: bool) {
        let led = self.get_led(index);
        led.toggle(enable);
    }

    /// Enable or disable all LEDs.
    pub fn toggle_all(&self, enable: bool) {
        for led in &self.leds {
            led.toggle(enable);
        }
    }
}
