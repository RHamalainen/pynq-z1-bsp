//! Interrupt functionality.
//!
//! # How to use?
//!
//! To enable routing and detection of interrupt requests, you must configure [`GIC`](gic) and [`ICC`](icc).
//!
//! Following example configures `PL0` shared peripheral interrupt.
//!
//! ```ignore
//! GIC.toggle(false);
//! ICC.toggle(false);
//!
//! let interrupt = Irq::Spi(SpiIrq::Pl0);
//! GIC.toggle_interrupt(interrupt, true);
//! GIC.set_shared_peripheral_interrupt_sensitivity(interrupt, InterruptSensitivity::Edge);
//! GIC.set_shared_peripheral_interrupt_targets(interrupt, InterruptTargets::Cpu0);
//! GIC.set_interrupt_priority(interrupt, InterruptPriority::Priority0);
//!
//! ICC.set_interrupt_priority_filter(InterruptPriorityFilter::AllowAll);
//!
//! ICC.toggle(true);
//! GIC.toggle(true);
//! ```

// TODO: clear interrupts when reset

pub mod gic;
pub mod handler;
pub mod icc;
pub mod irq_numbers;

/// Used to determine in which order parallel interrupts are handled.
///
/// The higher the number, the lower the priority.
#[derive(Clone, Copy)]
pub enum InterruptPriority {
    /// Highest priority.
    Priority0,
    Priority1,
    Priority2,
    Priority3,
    Priority4,
    Priority5,
    Priority6,
    Priority7,
    Priority8,
    Priority9,
    Priority10,
    Priority11,
    Priority12,
    Priority13,
    Priority14,
    Priority15,
    Priority16,
    Priority17,
    Priority18,
    Priority19,
    Priority20,
    Priority21,
    Priority22,
    Priority23,
    Priority24,
    Priority25,
    Priority26,
    Priority27,
    Priority28,
    Priority29,
    Priority30,

    /// Lowest priority.
    Priority31,
}

impl InterruptPriority {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0b00000000 => Self::Priority0,
            0b00001000 => Self::Priority1,
            0b00010000 => Self::Priority2,
            0b00011000 => Self::Priority3,
            0b00100000 => Self::Priority4,
            0b00101000 => Self::Priority5,
            0b00110000 => Self::Priority6,
            0b00111000 => Self::Priority7,
            0b01000000 => Self::Priority8,
            0b01001000 => Self::Priority9,
            0b01010000 => Self::Priority10,
            0b01011000 => Self::Priority11,
            0b01100000 => Self::Priority12,
            0b01101000 => Self::Priority13,
            0b01110000 => Self::Priority14,
            0b01111000 => Self::Priority15,
            0b10000000 => Self::Priority16,
            0b10001000 => Self::Priority17,
            0b10010000 => Self::Priority18,
            0b10011000 => Self::Priority19,
            0b10100000 => Self::Priority20,
            0b10101000 => Self::Priority21,
            0b10110000 => Self::Priority22,
            0b10111000 => Self::Priority23,
            0b11000000 => Self::Priority24,
            0b11001000 => Self::Priority25,
            0b11010000 => Self::Priority26,
            0b11011000 => Self::Priority27,
            0b11100000 => Self::Priority28,
            0b11101000 => Self::Priority29,
            0b11110000 => Self::Priority30,
            0b11111000 => Self::Priority31,
            other => panic!("Invalid interrupt priority: {other}"),
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Self::Priority0 => 0b00000000,
            Self::Priority1 => 0b00001000,
            Self::Priority2 => 0b00010000,
            Self::Priority3 => 0b00011000,
            Self::Priority4 => 0b00100000,
            Self::Priority5 => 0b00101000,
            Self::Priority6 => 0b00110000,
            Self::Priority7 => 0b00111000,
            Self::Priority8 => 0b01000000,
            Self::Priority9 => 0b01001000,
            Self::Priority10 => 0b01010000,
            Self::Priority11 => 0b01011000,
            Self::Priority12 => 0b01100000,
            Self::Priority13 => 0b01101000,
            Self::Priority14 => 0b01110000,
            Self::Priority15 => 0b01111000,
            Self::Priority16 => 0b10000000,
            Self::Priority17 => 0b10001000,
            Self::Priority18 => 0b10010000,
            Self::Priority19 => 0b10011000,
            Self::Priority20 => 0b10100000,
            Self::Priority21 => 0b10101000,
            Self::Priority22 => 0b10110000,
            Self::Priority23 => 0b10111000,
            Self::Priority24 => 0b11000000,
            Self::Priority25 => 0b11001000,
            Self::Priority26 => 0b11010000,
            Self::Priority27 => 0b11011000,
            Self::Priority28 => 0b11100000,
            Self::Priority29 => 0b11101000,
            Self::Priority30 => 0b11110000,
            Self::Priority31 => 0b11111000,
        }
    }
}
