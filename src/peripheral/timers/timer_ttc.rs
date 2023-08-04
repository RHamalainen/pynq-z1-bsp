//! Triple timer counters.

use core::ops::Not;

use crate::common::bitman::ReadBitwise;
use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;

// TODO: is this correct value?
//const CHANGES_PER_USECOND: u32 = 325;

struct Parameters {
    // TODO: is actually 4 bits value
    prescaler: u8,
    interval_value: u16,
}

impl Parameters {
    /// Solve prescaler and interval values from requested µseconds.
    pub fn solve(interval_us: u32) -> Self {
        use crate::common::timing::FREQUENCY_PERIPHERALS;

        let mut best_prescaler = 0u8;
        let mut best_interval = 0u16;
        let mut best_difference = u32::MAX;
        for prescaler in 0..16u32 {
            let frequency_scaler = 2u32.pow(prescaler + 1u32);
            let ticks_per_second = FREQUENCY_PERIPHERALS / frequency_scaler;
            let ticks_per_usecond = ticks_per_second / 1_000_000;
            if ticks_per_usecond == 0 {
                continue;
            }
            for ticks_per_interval in 0..0xFFFFu32 {
                let useconds_per_interval = ticks_per_interval / ticks_per_usecond;
                let difference = interval_us.abs_diff(useconds_per_interval);
                if difference < best_difference {
                    best_difference = difference;
                    best_prescaler = prescaler.try_into().unwrap();
                    best_interval = ticks_per_interval.try_into().unwrap();
                }
            }
        }
        Self {
            prescaler: best_prescaler,
            interval_value: best_interval,
        }
    }

    /// Maybe get µseconds per one interval.
    pub fn useconds_per_interval(&self) -> Option<u32> {
        use crate::common::timing::FREQUENCY_PERIPHERALS;

        let prescaler: u32 = self.prescaler.try_into().unwrap();
        let frequency_scaler = 2u32.pow(prescaler + 1u32);
        let ticks_per_second = FREQUENCY_PERIPHERALS / frequency_scaler;
        let ticks_per_usecond = ticks_per_second / 1_000_000;
        if ticks_per_usecond == 0 {
            None
        } else {
            let ticks_per_interval: u32 = self.interval_value.try_into().unwrap();
            let useconds_per_interval = ticks_per_interval / ticks_per_usecond;
            Some(useconds_per_interval)
        }
    }
}

/// Triple timer counter mode.
#[derive(Clone, Copy)]
pub enum ClockSource {
    /// Use processor's clock.
    ///
    /// Default.
    Internal,

    /// Use external clock.
    ///
    /// ext_clk
    External,
}

impl ClockSource {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Internal => false,
            Self::External => true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::External
        } else {
            Self::Internal
        }
    }
}

#[derive(Clone, Copy)]
pub enum ExternalClockEdge {
    /// Interpret external clock's positive edge as clock signal.
    ///
    /// Default configuration.
    Positive,

    /// Interpret external clock's negative edge as clock signal.
    Negative,
}

impl ExternalClockEdge {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Positive => false,
            Self::Negative => true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Negative
        } else {
            Self::Positive
        }
    }
}

#[derive(Clone, Copy)]
pub enum TimerMode {
    /// Interrupt is generated when counter overflows.
    ///
    /// Default configuration.
    Overflow,

    /// Interrupt is generated when counter's value reaches interval register's value.
    Interval,
}

impl TimerMode {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Overflow => false,
            Self::Interval => true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Interval
        } else {
            Self::Overflow
        }
    }
}

#[derive(Clone, Copy)]
pub enum TimerDirection {
    /// Counter counts up.
    ///
    /// Default configuration.
    Increment,

    /// Counter counts down.
    Decrement,
}

impl TimerDirection {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::Increment => false,
            Self::Decrement => true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Decrement
        } else {
            Self::Increment
        }
    }
}

#[derive(Clone, Copy)]
pub enum WaveformPolarity {
    /// Waveform goes from low to high on match interrupt and returns low on overflow or interval interrupt.
    ///
    /// Default configuration.
    LowToHigh,

    /// Waveform goes from high to low on match interrupt and returns high on overflow or interval interrupt.
    HighToLow,
}

impl WaveformPolarity {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::LowToHigh => false,
            Self::HighToLow => true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::HighToLow
        } else {
            Self::LowToHigh
        }
    }
}

#[derive(Clone, Copy)]
pub enum MatchIndex {
    One,
    Two,
    Three,
}

impl MatchIndex {
    // Transform to 32-bit unsigned integer.
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
        }
    }
}

#[derive(Clone, Copy)]
pub enum EventTimerPolarity {
    /// Event timer counts clock cycles when ext_clk is high.
    ///
    /// Default configuration.
    High,

    /// Event timer counts clock cycles when ext_clk is low.
    Low,
}

impl EventTimerPolarity {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::High => false,
            Self::Low => true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Low
        } else {
            Self::High
        }
    }
}

#[derive(Clone, Copy)]
pub enum EventTimerMode {
    /// Event timer is disabled and count is reset after overflow.
    ///
    /// Default configuration.
    StopAndResetAfterOverflow,

    /// Event timer continues counting after overflow.
    ContinueAfterOverflow,
}

impl EventTimerMode {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::StopAndResetAfterOverflow => false,
            Self::ContinueAfterOverflow => true,
        }
    }

    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::ContinueAfterOverflow
        } else {
            Self::StopAndResetAfterOverflow
        }
    }
}

pub struct InterruptStatus {
    pub interval_interrupt: bool,
    pub match_1_interrupt: bool,
    pub match_2_interrupt: bool,
    pub match_3_interrupt: bool,
    pub counter_overflow: bool,
    pub event_timer_overflow_interrupt: bool,
}

impl InterruptStatus {
    pub fn new(value: u32) -> Self {
        // TODO: check that value is 6 bits
        Self {
            interval_interrupt: value.read_bit(0),
            match_1_interrupt: value.read_bit(1),
            match_2_interrupt: value.read_bit(2),
            match_3_interrupt: value.read_bit(3),
            counter_overflow: value.read_bit(4),
            event_timer_overflow_interrupt: value.read_bit(5),
        }
    }
}

/// Interface for TTC peripheral.
pub struct TTCTimer {
    /// Clock control register.
    pub address_clock_control: *mut u32,

    /// Counter control register.
    pub address_counter_control: *mut u32,

    /// Counter's value.
    pub address_counter_value: *mut u32,

    /// Interval's value.
    pub address_interval_value: *mut u32,

    /// Match 0's value.
    pub address_match_value_0: *mut u32,

    /// Match 1's value.
    pub address_match_value_1: *mut u32,

    /// Match 2's value.
    pub address_match_value_2: *mut u32,

    /// Interrupt status register.
    pub address_interrupt_status: *mut u32,

    /// Interrupt enable register.
    pub address_interrupt_enable: *mut u32,

    /// Event control register.
    pub address_event_control_timer: *mut u32,

    /// Clock cycle count.
    pub address_event: *mut u32,

    /// True if timer is waiting for interrupt.
    pub sleeping: core::sync::atomic::AtomicBool,
}

impl TTCTimer {
    /// Enable or disable prescaler.
    pub fn toggle_prescaler(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_clock_control, 0);
    }

    #[must_use]
    pub fn prescaler_enabled(&self) -> bool {
        read_address_bit(self.address_clock_control, 0)
    }

    // TODO: prescaler is 4 bits
    pub fn set_prescaler(&self, value: u8) {
        write_address_bits(self.address_clock_control, 1..=4, value as u32)
    }

    // TODO: prescaler is 4 bits
    #[must_use]
    pub fn get_prescaler(&self) -> u8 {
        read_address_bits(self.address_clock_control, 1..=4) as u8
    }

    pub fn set_clock_source(&self, source: ClockSource) {
        let action = if source.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_clock_control, 5);
    }

    #[must_use]
    pub fn get_clock_source(&self) -> ClockSource {
        let value = read_address_bit(self.address_clock_control, 5);
        ClockSource::from_bool(value)
    }

    pub fn set_external_clock_edge(&self, edge: ExternalClockEdge) {
        let action = if edge.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_clock_control, 6);
    }

    #[must_use]
    pub fn get_external_clock_edge(&self) -> ExternalClockEdge {
        let value = read_address_bit(self.address_clock_control, 6);
        ExternalClockEdge::from_bool(value)
    }

    /// Enable or disable counter.
    pub fn toggle_counter(&self, enable: bool) {
        // Counter is active low.
        let action = if enable {
            clear_address_bit
        } else {
            set_address_bit
        };
        action(self.address_counter_control, 0);
    }

    #[must_use]
    pub fn counter_enabled(&self) -> bool {
        // Counter is active low.
        read_address_bit(self.address_counter_control, 0).not()
    }

    pub fn set_mode(&self, mode: TimerMode) {
        let action = if mode.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_counter_control, 1);
    }

    #[must_use]
    pub fn get_mode(&self) -> TimerMode {
        let value = read_address_bit(self.address_counter_control, 1);
        TimerMode::from_bool(value)
    }

    pub fn set_direction(&self, direction: TimerDirection) {
        let action = if direction.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_counter_control, 2);
    }

    #[must_use]
    pub fn get_direction(&self) -> TimerDirection {
        let value = read_address_bit(self.address_counter_control, 1);
        TimerDirection::from_bool(value)
    }

    pub fn toggle_match_mode(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_counter_control, 3);
    }

    #[must_use]
    pub fn match_mode_enabled(&self) -> bool {
        read_address_bit(self.address_counter_control, 3)
    }

    pub fn reset(&self) {
        let address = self.address_counter_control;
        let index = 4;
        set_address_bit(address, index);
        clear_address_bit(address, index);
    }

    pub fn toggle_output_waveform(&self, enable: bool) {
        // Output waveform is active low.
        let action = if enable {
            clear_address_bit
        } else {
            set_address_bit
        };
        action(self.address_counter_control, 5);
    }

    #[must_use]
    pub fn output_waveform_enabled(&self) -> bool {
        // Output waveform is active low.
        read_address_bit(self.address_counter_control, 5).not()
    }

    pub fn set_waveform_polarity(&self, polarity: WaveformPolarity) {
        let action = if polarity.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_counter_control, 6);
    }

    #[must_use]
    pub fn get_waveform_polarity(&self) -> WaveformPolarity {
        let value = read_address_bit(self.address_counter_control, 6);
        WaveformPolarity::from_bool(value)
    }

    #[must_use]
    pub fn get_counter_value(&self) -> u16 {
        read_from_address(self.address_counter_value) as u16
    }

    pub fn set_interval_value(&self, value: u16) {
        write_to_address(self.address_interval_value, value as u32);
    }

    #[must_use]
    pub fn get_interval_value(&self) -> u16 {
        read_from_address(self.address_interval_value) as u16
    }

    pub fn set_match_value(&self, index: MatchIndex, value: u16) {
        let address = match index {
            MatchIndex::One => self.address_match_value_0,
            MatchIndex::Two => self.address_match_value_1,
            MatchIndex::Three => self.address_match_value_2,
        };
        write_to_address(address, value as u32);
    }

    #[must_use]
    pub fn get_match_value(&self, index: MatchIndex) -> u16 {
        let address = match index {
            MatchIndex::One => self.address_match_value_0,
            MatchIndex::Two => self.address_match_value_1,
            MatchIndex::Three => self.address_match_value_2,
        };
        read_from_address(address) as u16
    }

    #[must_use]
    pub fn clear_interrupt(&self) -> InterruptStatus {
        let value = read_from_address(self.address_interrupt_status);
        InterruptStatus::new(value)
    }

    pub fn toggle_interval_interrupt(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_interrupt_enable, 0);
    }

    pub fn toggle_match_interrupt(&self, match_index: MatchIndex, enable: bool) {
        let bit_index = match match_index {
            MatchIndex::One => 1,
            MatchIndex::Two => 2,
            MatchIndex::Three => 3,
        };
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_interrupt_enable, bit_index);
    }

    pub fn toggle_counter_overflow_interrupt(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_interrupt_enable, 4);
    }

    pub fn toggle_event_timer_overflow_interrupt(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_interrupt_enable, 5);
    }

    pub fn toggle_all_interrupts(&self, enable: bool) {
        self.toggle_interval_interrupt(enable);
        self.toggle_match_interrupt(MatchIndex::One, enable);
        self.toggle_match_interrupt(MatchIndex::Two, enable);
        self.toggle_match_interrupt(MatchIndex::Three, enable);
        self.toggle_counter_overflow_interrupt(enable);
        self.toggle_event_timer_overflow_interrupt(enable);
    }

    pub fn toggle_event_timer(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_event_control_timer, 0);
    }

    pub fn set_event_timer_polarity(&self, polarity: EventTimerPolarity) {
        let action = if polarity.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_event_control_timer, 1);
    }

    // TODO: get

    pub fn set_event_timer_mode(&self, mode: EventTimerMode) {
        let action = if mode.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_event_control_timer, 2);
    }

    // TODO: get

    #[must_use]
    pub fn get_event_timer_count(&self) -> u16 {
        read_from_address(self.address_event) as u16
    }

    /// Solve and set prescaler and interval value from requested µseconds.
    pub fn set_interval_useconds(&self, useconds: u32) {
        use crate::sprintln;

        let parameters = Parameters::solve(useconds);
        let useconds_per_interval = parameters.useconds_per_interval().unwrap();

        sprintln!("Requested µseconds: {useconds}");
        sprintln!(" - Solved prescaler value: {}", parameters.prescaler);
        sprintln!(" - Solved inverval value: {}", parameters.interval_value);
        sprintln!(" - µseconds per interval: {useconds_per_interval}");

        let lower_bound: u32 = (0.9 * (useconds as f32)) as u32;
        let upper_bound: u32 = (1.1 * (useconds as f32)) as u32;
        if lower_bound <= useconds_per_interval {
            if useconds_per_interval <= upper_bound {
                self.set_prescaler(parameters.prescaler);
                self.set_interval_value(parameters.interval_value);
            } else {
                panic!("Could not solve prescaler and interval value to reach {useconds} µseconds per interval. Upper bound: {upper_bound}.");
            }
        } else {
            panic!("Could not solve prescaler and interval value to reach {useconds} µseconds per interval. Lower bound: {lower_bound}.");
        }
    }

    /// Sleep given µseconds.
    ///
    /// Only works for short sleeps, under 100 000 µseconds.
    pub fn usleep(&mut self, useconds: u32) {
        // TODO: return error if timer is not enabled
        // TODO: return error if event mode is enabled
        // TODO: return error if direction is not up
        // TODO: return error if matches are enabled

        // TODO: maybe store timer's context and restore after sleep?

        self.toggle_counter(false);
        self.toggle_event_timer(false);
        let _ = self.clear_interrupt();
        self.toggle_all_interrupts(false);
        self.set_clock_source(ClockSource::Internal);
        self.toggle_prescaler(true);
        self.set_interval_useconds(useconds);
        self.set_mode(TimerMode::Interval);
        //self.set_direction(TimerDirection::Increment);
        self.set_direction(TimerDirection::Decrement);
        self.toggle_match_mode(false);
        self.toggle_output_waveform(false);
        //assert_eq!(self.get_counter_value(), 0);
        //assert_eq!(self.get_counter_value(), 0);
        self.reset();
        assert!(0 < self.get_counter_value());

        self.toggle_interval_interrupt(true);

        self.sleeping = core::sync::atomic::AtomicBool::new(true);
        self.toggle_counter(true);
        while self.sleeping.load(core::sync::atomic::Ordering::Relaxed) {
            crate::common::instruction::nop();
        }
        self.toggle_counter(false);
    }
}

const ADDRESS_BASE_TTC0: u32 = 0xF800_1000;

/// Triple timer counter 0's timer/clock 0.
///
/// Can count events from MIO or EMIO.
pub static mut TIMER_TTC0_0: TTCTimer = TTCTimer {
    address_clock_control: (ADDRESS_BASE_TTC0 + 0x00) as *mut u32,
    address_counter_control: (ADDRESS_BASE_TTC0 + 0x0C) as *mut u32,
    address_counter_value: (ADDRESS_BASE_TTC0 + 0x18) as *mut u32,
    address_interval_value: (ADDRESS_BASE_TTC0 + 0x24) as *mut u32,
    address_match_value_0: (ADDRESS_BASE_TTC0 + 0x30) as *mut u32,
    address_match_value_1: (ADDRESS_BASE_TTC0 + 0x3C) as *mut u32,
    address_match_value_2: (ADDRESS_BASE_TTC0 + 0x48) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE_TTC0 + 0x54) as *mut u32,
    address_interrupt_enable: (ADDRESS_BASE_TTC0 + 0x60) as *mut u32,
    address_event_control_timer: (ADDRESS_BASE_TTC0 + 0x6C) as *mut u32,
    address_event: (ADDRESS_BASE_TTC0 + 0x78) as *mut u32,
    sleeping: core::sync::atomic::AtomicBool::new(false),
};

/// Triple timer counter 0's timer/clock 1.
///
/// Can count events from EMIO.
pub static mut TIMER_TTC0_1: TTCTimer = TTCTimer {
    address_clock_control: (ADDRESS_BASE_TTC0 + 0x04) as *mut u32,
    address_counter_control: (ADDRESS_BASE_TTC0 + 0x10) as *mut u32,
    address_counter_value: (ADDRESS_BASE_TTC0 + 0x1C) as *mut u32,
    address_interval_value: (ADDRESS_BASE_TTC0 + 0x28) as *mut u32,
    address_match_value_0: (ADDRESS_BASE_TTC0 + 0x34) as *mut u32,
    address_match_value_1: (ADDRESS_BASE_TTC0 + 0x40) as *mut u32,
    address_match_value_2: (ADDRESS_BASE_TTC0 + 0x4C) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE_TTC0 + 0x58) as *mut u32,
    address_interrupt_enable: (ADDRESS_BASE_TTC0 + 0x64) as *mut u32,
    address_event_control_timer: (ADDRESS_BASE_TTC0 + 0x70) as *mut u32,
    address_event: (ADDRESS_BASE_TTC0 + 0x7C) as *mut u32,
    sleeping: core::sync::atomic::AtomicBool::new(false),
};

/// Triple timer counter 0's timer/clock 2.
///
/// Can count events from EMIO.
pub static mut TIMER_TTC0_2: TTCTimer = TTCTimer {
    address_clock_control: (ADDRESS_BASE_TTC0 + 0x08) as *mut u32,
    address_counter_control: (ADDRESS_BASE_TTC0 + 0x14) as *mut u32,
    address_counter_value: (ADDRESS_BASE_TTC0 + 0x20) as *mut u32,
    address_interval_value: (ADDRESS_BASE_TTC0 + 0x2C) as *mut u32,
    address_match_value_0: (ADDRESS_BASE_TTC0 + 0x38) as *mut u32,
    address_match_value_1: (ADDRESS_BASE_TTC0 + 0x44) as *mut u32,
    address_match_value_2: (ADDRESS_BASE_TTC0 + 0x50) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE_TTC0 + 0x5C) as *mut u32,
    address_interrupt_enable: (ADDRESS_BASE_TTC0 + 0x68) as *mut u32,
    address_event_control_timer: (ADDRESS_BASE_TTC0 + 0x74) as *mut u32,
    address_event: (ADDRESS_BASE_TTC0 + 0x80) as *mut u32,
    sleeping: core::sync::atomic::AtomicBool::new(false),
};

const ADDRESS_BASE_TTC1: u32 = 0xF800_2000;

/// Triple timer counter 1's timer/clock 0.
///
/// Can count events from MIO or EMIO.
pub static mut TIMER_TTC1_0: TTCTimer = TTCTimer {
    address_clock_control: (ADDRESS_BASE_TTC1 + 0x00) as *mut u32,
    address_counter_control: (ADDRESS_BASE_TTC1 + 0x0C) as *mut u32,
    address_counter_value: (ADDRESS_BASE_TTC1 + 0x18) as *mut u32,
    address_interval_value: (ADDRESS_BASE_TTC1 + 0x24) as *mut u32,
    address_match_value_0: (ADDRESS_BASE_TTC1 + 0x30) as *mut u32,
    address_match_value_1: (ADDRESS_BASE_TTC1 + 0x3C) as *mut u32,
    address_match_value_2: (ADDRESS_BASE_TTC1 + 0x48) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE_TTC1 + 0x54) as *mut u32,
    address_interrupt_enable: (ADDRESS_BASE_TTC1 + 0x60) as *mut u32,
    address_event_control_timer: (ADDRESS_BASE_TTC1 + 0x6C) as *mut u32,
    address_event: (ADDRESS_BASE_TTC1 + 0x78) as *mut u32,
    sleeping: core::sync::atomic::AtomicBool::new(false),
};

/// Triple timer counter 1's timer/clock 1.
///
/// Can count events from EMIO.
pub static mut TIMER_TTC1_1: TTCTimer = TTCTimer {
    address_clock_control: (ADDRESS_BASE_TTC1 + 0x04) as *mut u32,
    address_counter_control: (ADDRESS_BASE_TTC1 + 0x10) as *mut u32,
    address_counter_value: (ADDRESS_BASE_TTC1 + 0x1C) as *mut u32,
    address_interval_value: (ADDRESS_BASE_TTC1 + 0x28) as *mut u32,
    address_match_value_0: (ADDRESS_BASE_TTC1 + 0x34) as *mut u32,
    address_match_value_1: (ADDRESS_BASE_TTC1 + 0x40) as *mut u32,
    address_match_value_2: (ADDRESS_BASE_TTC1 + 0x4C) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE_TTC1 + 0x58) as *mut u32,
    address_interrupt_enable: (ADDRESS_BASE_TTC1 + 0x64) as *mut u32,
    address_event_control_timer: (ADDRESS_BASE_TTC1 + 0x70) as *mut u32,
    address_event: (ADDRESS_BASE_TTC1 + 0x7C) as *mut u32,
    sleeping: core::sync::atomic::AtomicBool::new(false),
};

/// Triple timer counter 1's timer/clock 2.
///
/// Can count events from EMIO.
pub static mut TIMER_TTC1_2: TTCTimer = TTCTimer {
    address_clock_control: (ADDRESS_BASE_TTC1 + 0x08) as *mut u32,
    address_counter_control: (ADDRESS_BASE_TTC1 + 0x14) as *mut u32,
    address_counter_value: (ADDRESS_BASE_TTC1 + 0x20) as *mut u32,
    address_interval_value: (ADDRESS_BASE_TTC1 + 0x2C) as *mut u32,
    address_match_value_0: (ADDRESS_BASE_TTC1 + 0x38) as *mut u32,
    address_match_value_1: (ADDRESS_BASE_TTC1 + 0x44) as *mut u32,
    address_match_value_2: (ADDRESS_BASE_TTC1 + 0x50) as *mut u32,
    address_interrupt_status: (ADDRESS_BASE_TTC1 + 0x5C) as *mut u32,
    address_interrupt_enable: (ADDRESS_BASE_TTC1 + 0x68) as *mut u32,
    address_event_control_timer: (ADDRESS_BASE_TTC1 + 0x74) as *mut u32,
    address_event: (ADDRESS_BASE_TTC1 + 0x80) as *mut u32,
    sleeping: core::sync::atomic::AtomicBool::new(false),
};
