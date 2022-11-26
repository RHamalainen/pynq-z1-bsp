//! System timing values.

/// PYNQ-Z1 provides 50 MHz clock to Zynq `PS_CLK` pin.
/// This enables the processor to operate at maximum frequency of 650 MHz.
pub const FREQUENCY_PROCESSOR: u32 = 650_000_000;

/// Peripherals are clocked half of the processor frequency.
pub const FREQUENCY_PERIPHERALS: u32 = FREQUENCY_PROCESSOR / 2;
