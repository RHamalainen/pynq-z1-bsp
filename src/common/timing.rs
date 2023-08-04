//! System timing values.

/// Processor frequency.
///
/// `PYNQ-Z1` provides 50 MHz clock to `Zynq-7000`'s `PS_CLK` pin.
/// This enables the processor to operate at maximum frequency of 650 MHz.
pub const FREQUENCY_PROCESSOR: u32 = 650_000_000;

/// Peripherals are clocked half of the [processor frequency](FREQUENCY_PROCESSOR).
pub const FREQUENCY_PERIPHERALS: u32 = FREQUENCY_PROCESSOR / 2;
