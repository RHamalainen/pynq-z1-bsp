//! Universal asynchronous receiver-transmitter.
//!
//! # How to use?
//!
//! ```ignore
//! UART0::configure();
//! UART0::enable();
//! UART0::println("Hello, World!");
//! ```
//!
//! # TODO
//!
//! - modem

use crate::common::bitman::ReadBitwise;
use crate::common::instruction::nop;
use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_to_address;
use core::ops::BitAnd;

/// UART clock source configuration.
#[derive(Clone, Copy)]
pub enum ClockSource {
    /// Use pure input clock.
    UartRefClk,

    /// Pre-scaler 8 is applied to the baud rate generator input clock.
    UartRefClkDiv8,
}

impl ClockSource {
    /// Transform to boolean.
    #[inline]
    #[must_use]
    pub const fn as_bool(self) -> bool {
        match self {
            Self::UartRefClk => false,
            Self::UartRefClkDiv8 => true,
        }
    }
}

/// Character length defines how many bits are used to represent one character.
#[derive(Clone, Copy)]
pub enum CharacterLength {
    /// Six bits represent one character.
    Six,

    /// Seven bits represent one character.
    Seven,

    /// Eight bits represent one character.
    Eight,
}

impl CharacterLength {
    /// Transform to unsigned 32-bit integer.
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::Eight => 0b00,
            Self::Seven => 0b10,
            Self::Six => 0b11,
        }
    }
}

/// UART parity bits configuration.
#[derive(Clone, Copy)]
pub enum ParityType {
    /// Even parity.
    Even,

    /// Odd parity.
    Odd,

    /// Forced to 0 parity.
    ForcedTo0,

    /// Forced to 1 parity.
    ForcedTo1,

    /// No parity.
    Disabled,
}

impl ParityType {
    /// Transform to unsigned 32-bit integer.
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::Even => 0b000,
            Self::Odd => 0b001,
            Self::ForcedTo0 => 0b010,
            Self::ForcedTo1 => 0b011,
            Self::Disabled => 0b100,
        }
    }
}

/// Stop bits define how many stop bits are detected when receiving or generated when transmitting.
#[derive(Clone, Copy)]
pub enum StopBits {
    /// One stop bit.
    One,

    /// One and half stop bits.
    OneAndHalf,

    /// Two stop bits.
    Two,
}

impl StopBits {
    /// Transform to unsigned 32-bit integer.
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::One => 0b00,
            Self::OneAndHalf => 0b01,
            Self::Two => 0b10,
        }
    }
}

/// UART channel mode.
#[derive(Clone, Copy)]
pub enum ChannelMode {
    /// Standard UART operation.
    Normal,

    /// Route received bytes back to external transmitter and to device.
    ///
    /// Transmitting disabled.
    AutomaticEcho,

    /// Route transmitted bytes back to receiver.
    LocalLoopback,

    /// Route received bytes back to external transmitter.
    ///
    /// Receiving and transmitting disabled.
    RemoteLoopback,
}

impl ChannelMode {
    /// Transform to unsigned 32-bit integer.
    #[inline]
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        match self {
            Self::Normal => 0b00,
            Self::AutomaticEcho => 0b01,
            Self::LocalLoopback => 0b10,
            Self::RemoteLoopback => 0b11,
        }
    }
}

/// UART interrupt cause.
#[derive(Clone, Copy)]
pub enum InterruptCause {
    /// Receiver FIFO level reached given trigger level.
    ReceiverFifoTrigger,

    /// Receiver FIFO is empty.
    ReceiverFifoEmpty,

    /// Receiver FIFO is full.
    ReceiverFifoFull,

    /// Transmitter FIFO is empty.
    TransmitterFifoEmpty,

    /// Transmitter FIFO is full.
    TransmitterFifoFull,

    /// Receiver FIFO was full when new byte was received.
    ReceiverOverflow,

    /// Receiver failed to receive valid stop bit at the end of a frame.
    ReceiverFraming,

    /// Parity calculated from received bytes was not equal to received parity bit(s).
    ReceiverParity,

    /// Receiver timeout counter reached zero.
    ReceiverTimeout,

    /// TODO: what is this?
    ModemIndicator,

    /// Transmitter FIFO level reached given trigger level.
    TransmitterFifoTrigger,

    /// Transmitter FIFO capacity has only one byte left.
    TransmitterFifoNearlyFull,

    /// Transmitter FIFO was full when attempted to transmit new byte.
    TransmitterFifoOverflow,
}

impl InterruptCause {
    /// Interrupt cause bit index.
    #[inline]
    #[must_use]
    pub const fn bit_index(self) -> u32 {
        match self {
            Self::ReceiverFifoTrigger => 0,
            Self::ReceiverFifoEmpty => 1,
            Self::ReceiverFifoFull => 2,
            Self::TransmitterFifoEmpty => 3,
            Self::TransmitterFifoFull => 4,
            Self::ReceiverOverflow => 5,
            Self::ReceiverFraming => 6,
            Self::ReceiverParity => 7,
            Self::ReceiverTimeout => 8,
            Self::ModemIndicator => 9,
            Self::TransmitterFifoTrigger => 10,
            Self::TransmitterFifoNearlyFull => 11,
            Self::TransmitterFifoOverflow => 12,
        }
    }
}

/// UART interrupt's causes.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy)]
pub struct InterruptCauses {
    /// Receiver FIFO level reached given trigger level.
    pub receiver_fifo_trigger: bool,

    /// Receiver FIFO is empty.
    pub receiver_fifo_empty: bool,

    /// Receiver FIFO is full.
    pub receiver_fifo_full: bool,

    /// Transmitter FIFO is empty.
    pub transmitter_fifo_empty: bool,

    /// Transmitter FIFO is full.
    pub transmitter_fifo_full: bool,

    /// Receiver FIFO was full when new byte was received.
    pub receiver_overflow: bool,

    /// Receiver failed to receive valid stop bit at the end of a frame.
    pub receiver_framing: bool,

    /// Parity calculated from received bytes was not equal to received parity bit(s).
    pub receiver_parity: bool,

    /// Receiver timeout counter reached zero.
    pub receiver_timeout: bool,

    /// TODO: what is this?
    pub modem_indicator: bool,

    /// Transmitter FIFO level reached given trigger level.
    pub transmitter_fifo_trigger: bool,

    /// Transmitter FIFO capacity has only one byte left.
    pub transmitter_fifo_nearly_full: bool,

    /// Transmitter FIFO was full when attempted to transmit new byte.
    pub transmitter_fifo_overflow: bool,
}

impl InterruptCauses {
    /// Create new UART interrupt cause.
    #[inline]
    #[must_use]
    pub fn new(value: u32) -> Self {
        Self {
            receiver_fifo_trigger: value.read_bit(0),
            receiver_fifo_empty: value.read_bit(1),
            receiver_fifo_full: value.read_bit(2),
            transmitter_fifo_empty: value.read_bit(3),
            transmitter_fifo_full: value.read_bit(4),
            receiver_overflow: value.read_bit(5),
            receiver_framing: value.read_bit(6),
            receiver_parity: value.read_bit(7),
            receiver_timeout: value.read_bit(8),
            modem_indicator: value.read_bit(9),
            transmitter_fifo_trigger: value.read_bit(10),
            transmitter_fifo_nearly_full: value.read_bit(11),
            transmitter_fifo_overflow: value.read_bit(12),
        }
    }
}

/// Interface for UART peripheral.
pub struct Uart {
    /// UART control register.
    pub address_control: *mut u32,

    /// UART mode register.
    pub address_mode: *mut u32,

    /// Interrupt enable register.
    pub address_interrupt_enable: *mut u32,

    /// Interrupt disable register.
    pub address_interrupt_disable: *mut u32,

    /// Interrupt mask register.
    pub address_interrupt_mask: *mut u32,

    /// Channel interrupt status register.
    pub address_channel_interrupt_status: *mut u32,

    /// Baud rate generator register.
    pub address_baud_rate_generator: *mut u32,

    /// Receiver timeout register.
    pub address_receiver_timeout: *mut u32,

    /// Receiver FIFO trigger level register.
    pub address_receiver_fifo_trigger_level: *mut u32,

    /// Modem control register.
    pub address_modem_control: *mut u32,

    /// Modem status register.
    pub address_modem_status: *mut u32,

    /// Channel status register.
    pub address_channel_status: *mut u32,

    /// Transmit and receive FIFO.
    pub address_transmit_and_receive_fifo: *mut u32,

    /// Baud rate divider register.
    pub address_baud_rate_divider: *mut u32,

    /// Flow control delay register.
    pub address_flow_control_delay: *mut u32,

    /// Transmitter FIFO trigger level register.
    pub address_transmitter_fifo_trigger_level: *mut u32,
}

impl Uart {
    /// Set at what transmitter FIFO buffer value an interrupt is generated.
    ///
    /// # Panics
    ///
    /// Given value is too big.
    #[inline]
    pub fn set_transmitter_fifo_trigger_value(&self, value: u32) {
        assert!(value < 2u32.pow(6));
        for index in 0..=5 {
            let action = if value.read_bit(index) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_transmitter_fifo_trigger_level, index);
        }
    }

    /// Set at what receiver FIFO buffer value an interrupt is generated.
    ///
    /// # Panics
    ///
    /// Given value is too big.
    #[inline]
    pub fn set_receiver_fifo_trigger_value(&self, value: u32) {
        assert!(value < 2u32.pow(6));
        for index in 0..=5 {
            let action = if value.read_bit(index) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_receiver_fifo_trigger_level, index);
        }
    }

    /// Helper for enabling and disabling interrupts.
    #[inline]
    fn toggle_interrupt(&self, enable: bool, index: u32) {
        let address = if enable {
            self.address_interrupt_enable
        } else {
            self.address_interrupt_disable
        };
        set_address_bit(address, index);
    }

    /// Enable or disable transmitter FIFO overflow interrupt.
    #[inline]
    pub fn toggle_transmitter_fifo_overflow_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 12);
    }

    /// Enable or disable transmitter FIFO nearly full interrupt.
    #[inline]
    pub fn toggle_transmitter_fifo_nearly_full_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 11);
    }

    /// Enable or disable transmitter FIFO trigger interrupt.
    #[inline]
    pub fn toggle_transmitter_fifo_trigger_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 10);
    }

    /// Enable or disable transmitter FIFO full interrupt.
    #[inline]
    pub fn toggle_transmitter_fifo_full_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 4);
    }

    /// Enable or disable transmitter FIFO empty interrupt.
    #[inline]
    pub fn toggle_transmitter_fifo_empty_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 3);
    }

    /// Enable or disable receiver timeout error interrupt.
    #[inline]
    pub fn toggle_receiver_timeout_error_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 8);
    }

    /// Enable or disable receiver parity error interrupt.
    #[inline]
    pub fn toggle_receiver_parity_error_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 7);
    }

    /// Enable or disable receiver framing error interrupt.
    #[inline]
    pub fn toggle_receiver_framing_error_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 6);
    }

    /// Enable or disable receiver overflow error interrupt.
    #[inline]
    pub fn toggle_receiver_overflow_error_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 5);
    }

    /// Enable or disable receiver FIFO full interrupt.
    #[inline]
    pub fn toggle_receiver_fifo_full_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 2);
    }

    /// Enable or disable receiver FIFO empty interrupt.
    #[inline]
    pub fn toggle_receiver_fifo_empty_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 1);
    }

    /// Enable or disable receiver FIFO trigger interrupt.
    #[inline]
    pub fn toggle_receiver_fifo_trigger_interrupt(&self, enable: bool) {
        self.toggle_interrupt(enable, 0);
    }

    /// Receiver logic is reset and all pending receiver data is discarded.
    #[inline]
    pub fn reset_receiver(&self) {
        set_address_bit(self.address_control, 0);
        clear_address_bit(self.address_control, 0);
    }

    /// Transmitter logic is reset and all pending transmitter data is discarded.
    #[inline]
    pub fn reset_transmitter(&self) {
        set_address_bit(self.address_control, 1);
        clear_address_bit(self.address_control, 1);
    }

    /// Reset both receiver and transmitter logics.
    #[inline]
    pub fn reset(&self) {
        self.reset_receiver();
        self.reset_transmitter();
    }

    /// Enable or disable receiving.
    #[inline]
    pub fn toggle_receiving(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 2);
    }

    /// Enable or disable transmitting.
    #[inline]
    pub fn toggle_transmitting(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_control, 4);
    }

    /// Enable or disable receiver and transmitter.
    #[inline]
    pub fn toggle(&self, enable: bool) {
        self.toggle_receiving(enable);
        self.toggle_transmitting(enable);
    }

    /// Set parity bit configuration.
    #[inline]
    pub fn set_parity(&self, value: ParityType) {
        for (index, bit_index) in (3..=5).into_iter().enumerate() {
            let action = if value.as_u32().read_bit(index as u32) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_mode, bit_index);
        }
    }

    /// Set clock source configuration.
    #[inline]
    pub fn set_clock_source(&self, value: ClockSource) {
        let action = if value.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_mode, 0);
    }

    /// Set character length configuration.
    #[inline]
    pub fn set_character_length(&self, value: CharacterLength) {
        for (index, bit_index) in (1..=2).into_iter().enumerate() {
            let action = if value.as_u32().read_bit(index as u32) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_mode, bit_index);
        }
    }

    /// Set stop bits configuration.
    #[inline]
    pub fn set_stop_bits(&self, value: StopBits) {
        for (index, bit_index) in (6..=7).into_iter().enumerate() {
            let action = if value.as_u32().read_bit(index as u32) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_mode, bit_index);
        }
    }

    /// Set channel mode configuration.
    #[inline]
    pub fn set_channel_mode(&self, value: ChannelMode) {
        for (index, bit_index) in (8..=9).into_iter().enumerate() {
            let action = if value.as_u32().read_bit(index as u32) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_mode, bit_index);
        }
    }

    /// Configure UART with default configuration.
    ///
    /// 1. Disable receiving and transmitting.
    /// 2. Reset receiver and transmitter.
    /// 3. Use input clock without prescaling.
    /// 4. Disable parity bits.
    /// 5. Use one stop bit.
    /// 6. Use standard UART channel mode.
    #[inline]
    pub fn configure(&self) {
        self.toggle(false);
        self.reset();
        self.set_clock_source(ClockSource::UartRefClk);
        self.set_character_length(CharacterLength::Eight);
        self.set_parity(ParityType::Disabled);
        self.set_stop_bits(StopBits::One);
        self.set_channel_mode(ChannelMode::Normal);
    }

    /// Is true if transmitter FIFO is full.
    #[inline]
    #[must_use]
    pub fn transmitter_fifo_is_full(&self) -> bool {
        read_address_bit(self.address_channel_status, 4)
    }

    /// Is true if transmitter FIFO is empty.
    #[inline]
    #[must_use]
    pub fn transmitter_fifo_is_empty(&self) -> bool {
        read_address_bit(self.address_channel_status, 3)
    }

    /// Is true if receiver FIFO is full.
    #[inline]
    #[must_use]
    pub fn receiver_fifo_is_full(&self) -> bool {
        read_address_bit(self.address_channel_status, 2)
    }

    /// Is true if receiver FIFO is empty.
    #[inline]
    #[must_use]
    pub fn receiver_fifo_is_empty(&self) -> bool {
        read_address_bit(self.address_channel_status, 1)
    }

    // TODO:
    // host can do useful work when transmitting multiple bytes
    // 1. send byte 0..N
    // 2. FIFO is full, continue other work
    // 3. FIFO empty interrupt -> send bytes N..M

    /// Transmit one byte.
    #[inline]
    pub fn transmit_byte(&self, byte: u8) {
        // Wait until transmit buffer has space for more bytes.
        while self.transmitter_fifo_is_full() {
            nop();
        }
        write_to_address(self.address_transmit_and_receive_fifo, byte as u32);
    }

    /// Transmit a string without ending the line.
    #[inline]
    pub fn print(&self, string: &str) {
        for byte in string.as_bytes() {
            self.transmit_byte(*byte);
        }
    }

    /// Transmit a string and end the line.
    #[inline]
    pub fn println(&self, string: &str) {
        self.print(string);
        self.print("\r\n");
    }

    /// Try to receive byte.
    #[inline]
    #[must_use]
    pub fn try_receive_byte(&self) -> Option<u8> {
        if self.receiver_fifo_is_empty() {
            None
        } else {
            Some(read_from_address(self.address_transmit_and_receive_fifo) as u8)
        }
    }

    /// Read UART interrupt causes.
    #[inline]
    #[must_use]
    pub fn read_interrupt_cause(&self) -> InterruptCauses {
        let unmasked = read_from_address(self.address_channel_interrupt_status);
        let mask = read_from_address(self.address_interrupt_mask);
        let value = unmasked.bitand(mask);
        InterruptCauses::new(value)
    }

    /// Clear UART interrupt cause.
    #[inline]
    pub fn clear_interrupt_cause(&self, cause: InterruptCause) {
        let index = cause.bit_index();
        set_address_bit(self.address_channel_interrupt_status, index);
    }

    /// Clear all UART interrupt causes.
    #[inline]
    pub fn clear_all_interrupt_causes(&self) {
        write_to_address(self.address_channel_interrupt_status, 0xFFFF_FFFF);
    }

    /*pub fn set_baud_rate(&self) {
        self.toggle(false);


        self.reset();
    }*/
}

/// UART 0 base address.
pub const ADDRESS_UART0_BASE: u32 = 0xE000_0000;
/// UART 1 base address.
pub const ADDRESS_UART1_BASE: u32 = 0xE000_1000;

/// UART 0 peripheral.
pub static mut UART0: Uart = Uart {
    address_control: (ADDRESS_UART0_BASE + 0x00) as *mut u32,
    address_mode: (ADDRESS_UART0_BASE + 0x04) as *mut u32,
    address_interrupt_enable: (ADDRESS_UART0_BASE + 0x08) as *mut u32,
    address_interrupt_disable: (ADDRESS_UART0_BASE + 0x0C) as *mut u32,
    address_interrupt_mask: (ADDRESS_UART0_BASE + 0x10) as *mut u32,
    address_channel_interrupt_status: (ADDRESS_UART0_BASE + 0x14) as *mut u32,
    address_baud_rate_generator: (ADDRESS_UART0_BASE + 0x18) as *mut u32,
    address_receiver_timeout: (ADDRESS_UART0_BASE + 0x1C) as *mut u32,
    address_receiver_fifo_trigger_level: (ADDRESS_UART0_BASE + 0x20) as *mut u32,
    address_modem_control: (ADDRESS_UART0_BASE + 0x24) as *mut u32,
    address_modem_status: (ADDRESS_UART0_BASE + 0x28) as *mut u32,
    address_channel_status: (ADDRESS_UART0_BASE + 0x2C) as *mut u32,
    address_transmit_and_receive_fifo: (ADDRESS_UART0_BASE + 0x30) as *mut u32,
    address_baud_rate_divider: (ADDRESS_UART0_BASE + 0x34) as *mut u32,
    address_flow_control_delay: (ADDRESS_UART0_BASE + 0x38) as *mut u32,
    address_transmitter_fifo_trigger_level: (ADDRESS_UART0_BASE + 0x44) as *mut u32,
};

/// UART 1 peripheral.
pub static mut UART1: Uart = Uart {
    address_control: (ADDRESS_UART1_BASE + 0x00) as *mut u32,
    address_mode: (ADDRESS_UART1_BASE + 0x04) as *mut u32,
    address_interrupt_enable: (ADDRESS_UART1_BASE + 0x08) as *mut u32,
    address_interrupt_disable: (ADDRESS_UART1_BASE + 0x0C) as *mut u32,
    address_interrupt_mask: (ADDRESS_UART1_BASE + 0x10) as *mut u32,
    address_channel_interrupt_status: (ADDRESS_UART1_BASE + 0x14) as *mut u32,
    address_baud_rate_generator: (ADDRESS_UART1_BASE + 0x18) as *mut u32,
    address_receiver_timeout: (ADDRESS_UART1_BASE + 0x1C) as *mut u32,
    address_receiver_fifo_trigger_level: (ADDRESS_UART1_BASE + 0x20) as *mut u32,
    address_modem_control: (ADDRESS_UART1_BASE + 0x24) as *mut u32,
    address_modem_status: (ADDRESS_UART1_BASE + 0x28) as *mut u32,
    address_channel_status: (ADDRESS_UART1_BASE + 0x2C) as *mut u32,
    address_transmit_and_receive_fifo: (ADDRESS_UART1_BASE + 0x30) as *mut u32,
    address_baud_rate_divider: (ADDRESS_UART1_BASE + 0x34) as *mut u32,
    address_flow_control_delay: (ADDRESS_UART1_BASE + 0x38) as *mut u32,
    address_transmitter_fifo_trigger_level: (ADDRESS_UART1_BASE + 0x44) as *mut u32,
};

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe { UART0.print(s) }
        Ok(())
    }
}

/// Print text using `UART0`.
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        #[allow(unused_imports)]
        use core::fmt::*;
        unsafe {
            write!($crate::peripheral::uart::UART0, $s).unwrap();
        }
    };
    ($($tt:tt)*) => {
        #[allow(unused_imports)]
        use core::fmt::*;
        unsafe {
            write!($crate::peripheral::uart::UART0, $($tt)*).unwrap();
        }
    };
}

/// Print line using `UART0`.
#[macro_export]
macro_rules! sprintln {
    () => {
        $crate::sprint!("\r\n");
    };
    ($($tt:tt)*) => {
        $crate::sprint!($($tt)*);
        $crate::sprint!("\r\n");
    };
}
