//! Universal asynchronous receiver-transmitter.
//!
//! # How to use?
//!
//! ```ignore
//! UART0::configure();
//! UART0::toggle(true);
//! UART0::transmit2_line("Hello, World!");
//! ```
//!
//! # TODO
//!
//! - modem

// TODO: separate to receiver and transmitter substructs

use crate::common::bitman::ReadBitwise;
use crate::common::instruction::nop;
use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_from_address;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;
use core::ops::BitAnd;
use core::ops::Not;

#[derive(Clone, Copy)]
pub enum DeviceIndex {
    Uart0,
    Uart1,
}

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

/// How many bits are used to represent one character.
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

/// How many stop bits are detected when receiving or generated when transmitting.
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

/// UART receiver interrupt.
#[derive(Clone, Copy)]
pub enum ReceiverInterrupt {
    /// Receiver FIFO level reached given trigger level.
    FifoTrigger,

    /// Receiver FIFO is empty.
    FifoEmpty,

    /// Receiver FIFO is full.
    FifoFull,

    /// Receiver FIFO was full when new byte was received.
    FifoOverflow,

    /// Receiver failed to receive valid stop bit at the end of a frame.
    FramingError,

    /// Parity calculated from received bytes was not equal to received parity bit(s).
    ParityError,

    /// Receiver timeout counter reached zero.
    Timeout,
}

impl ReceiverInterrupt {
    #[must_use]
    pub const fn as_index(self) -> u32 {
        match self {
            Self::FifoTrigger => 0,
            Self::FifoEmpty => 1,
            Self::FifoFull => 2,
            Self::FifoOverflow => 5,
            Self::FramingError => 6,
            Self::ParityError => 7,
            Self::Timeout => 8,
        }
    }
}

/// UART transmitter interrupt.
#[derive(Clone, Copy)]
pub enum TransmitterInterrupt {
    /// Transmitter FIFO is empty.
    FifoEmpty,

    /// Transmitter FIFO is full.
    FifoFull,

    /// Transmitter FIFO level reached given trigger level.
    FifoTrigger,

    /// Transmitter FIFO capacity has only one byte left.
    FifoNearlyFull,

    /// Transmitter FIFO was full when attempted to transmit new byte.
    FifoOverflow,
}

impl TransmitterInterrupt {
    #[must_use]
    pub const fn as_index(self) -> u32 {
        match self {
            Self::FifoEmpty => 3,
            Self::FifoFull => 4,
            Self::FifoTrigger => 10,
            Self::FifoNearlyFull => 11,
            Self::FifoOverflow => 12,
        }
    }
}

/// UART interrupt.
#[derive(Clone, Copy)]
pub enum Interrupt {
    /// Receiver interrupt.
    Receiver(ReceiverInterrupt),

    /// Transmitter interrupt.
    Transmitter(TransmitterInterrupt),

    /// TODO: what is this?
    ModemIndicator,
}

impl Interrupt {
    /// Interrupt bit index.
    #[inline]
    #[must_use]
    pub const fn as_index(self) -> u32 {
        match self {
            Self::Receiver(interrupt) => interrupt.as_index(),
            Self::Transmitter(interrupt) => interrupt.as_index(),
            Self::ModemIndicator => 9,
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
    /// Peripheral index.
    index: DeviceIndex,

    /// UART control register.
    address_control: *mut u32,

    /// UART mode register.
    address_mode: *mut u32,

    /// Interrupt enable register.
    address_interrupt_enable: *mut u32,

    /// Interrupt disable register.
    address_interrupt_disable: *mut u32,

    /// Interrupt mask register.
    address_interrupt_mask: *mut u32,

    /// Channel interrupt status register.
    address_channel_interrupt_status: *mut u32,

    /// Baud rate generator register.
    address_baud_rate_generator: *mut u32,

    /// Receiver timeout register.
    address_receiver_timeout: *mut u32,

    /// Receiver FIFO trigger level register.
    address_receiver_fifo_trigger_level: *mut u32,

    /// Modem control register.
    address_modem_control: *mut u32,

    /// Modem status register.
    address_modem_status: *mut u32,

    /// Channel status register.
    address_channel_status: *mut u32,

    /// Transmit and receive FIFO.
    address_transmit_and_receive_fifo: *mut u32,

    /// Baud rate divider register.
    address_baud_rate_divider: *mut u32,

    /// Flow control delay register.
    address_flow_control_delay: *mut u32,

    /// Transmitter FIFO trigger level register.
    address_transmitter_fifo_trigger_level: *mut u32,
}

impl Uart {
    /// Receiver logic is reset and all pending receiver data is discarded.
    #[inline]
    pub fn reset_receiver(&self) {
        set_address_bit(self.address_control, 0);
        // Bit is cleared automatically.
    }

    /// Transmitter logic is reset and all pending transmitter data is discarded.
    #[inline]
    pub fn reset_transmitter(&self) {
        set_address_bit(self.address_control, 1);
        // Bit is cleared automatically.
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

    // TODO: receiver disabled register

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

    // TODO: transmitter disable register

    // TODO: restart receiver timeout counter

    // TODO: start transmitter break

    // TODO: stop transmitter break

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

    /// True if given interrupt is enabled.
    pub fn is_interrupt_enabled(&self, interrupt: Interrupt) -> bool {
        let index = interrupt.as_index();
        read_address_bit(self.address_interrupt_mask, index)
    }

    /// Helper for enabling and disabling interrupts.
    #[inline]
    pub fn toggle_interrupt(&self, interrupt: Interrupt, enable: bool) {
        let address = if enable {
            self.address_interrupt_enable
        } else {
            self.address_interrupt_disable
        };
        let index = interrupt.as_index();
        set_address_bit(address, index);
    }

    /// Read interrupt causes.
    ///
    /// Also disabled interrupts are returned.
    #[inline]
    #[must_use]
    pub fn read_unmaked_interrupt_causes(&self) -> InterruptCauses {
        let unmasked = read_from_address(self.address_channel_interrupt_status);
        InterruptCauses::new(unmasked)
    }

    /// Read interrupt causes.
    ///
    /// Only enabled interrupts are returned.
    #[inline]
    #[must_use]
    pub fn read_interrupt_causes(&self) -> InterruptCauses {
        let unmasked = read_from_address(self.address_channel_interrupt_status);
        let mask = read_from_address(self.address_interrupt_mask);
        let value = unmasked.bitand(mask);
        InterruptCauses::new(value)
    }

    /// Clear given interrupt.
    #[inline]
    pub fn clear_interrupt(&self, interrupt: Interrupt) {
        let index = interrupt.as_index();
        set_address_bit(self.address_channel_interrupt_status, index);
    }

    // TODO: order registers

    // TODO: baud rate register

    // TODO: timeout

    /// Set at what transmitter FIFO buffer value an interrupt is generated.
    #[inline]
    pub fn set_transmitter_fifo_trigger_value(&self, value: u32) -> Result<(), ()> {
        if (0..=63).contains(&value).not() {
            return Err(());
        }
        write_address_bits(self.address_transmitter_fifo_trigger_level, 0..=5, value);
        Ok(())

        // TODO: remove
        /*assert!(value < 2u32.pow(6));
        for index in 0..=5 {
            let action = if value.read_bit(index) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_transmitter_fifo_trigger_level, index);
        }*/
    }

    // TODO: modem registers

    /// Set at what receiver FIFO buffer value an interrupt is generated.
    #[inline]
    pub fn set_receiver_fifo_trigger_value(&self, value: u32) -> Result<(), ()> {
        if (0..=63).contains(&value).not() {
            return Err(());
        }
        write_address_bits(self.address_receiver_fifo_trigger_level, 0..=5, value);
        Ok(())

        /*
        assert!(value < 2u32.pow(6));
        for index in 0..=5 {
            let action = if value.read_bit(index) {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_receiver_fifo_trigger_level, index);
        }
        */
    }

    // TODO: channel status register

    // TODO: reset registers
    /// Reset peripheral.
    ///
    /// TODO: maybe implement using slcr registers?
    #[inline]
    pub fn reset(&self) {
        self.reset_receiver();
        self.reset_transmitter();
        write_to_address(self.address_control, 0x128);
        write_to_address(self.address_mode, 0);
        write_to_address(self.address_interrupt_disable, 0xFFFF_FFFF);
        write_to_address(self.address_channel_interrupt_status, 0xFFFF_FFFF);
        // TODO
        write_to_address(self.address_receiver_fifo_trigger_level, 0x20);
        // TODO
        write_to_address(self.address_transmitter_fifo_trigger_level, 0x20);
    }

    /// Enable or disable receiver and transmitter.
    #[inline]
    pub fn toggle(&self, enable: bool) {
        self.toggle_receiving(enable);
        self.toggle_transmitting(enable);
    }

    pub fn is_transmitter_fifo_nearly_full(&self) -> bool {
        read_address_bit(self.address_channel_status, 14)
    }

    pub fn is_transmitter_fifo_trigger_reached(&self) -> bool {
        read_address_bit(self.address_channel_status, 13)
    }

    pub fn is_receiver_flow_delay_trigger_reached(&self) -> bool {
        read_address_bit(self.address_channel_status, 12)
    }

    pub fn is_transmitter_active(&self) -> bool {
        read_address_bit(self.address_channel_status, 11)
    }

    pub fn is_receiver_active(&self) -> bool {
        read_address_bit(self.address_channel_status, 10)
    }

    /// True if transmitter FIFO is full.
    #[inline]
    #[must_use]
    pub fn is_transmitter_fifo_full(&self) -> bool {
        read_address_bit(self.address_channel_status, 4)
    }

    /// True if transmitter FIFO is empty.
    #[inline]
    #[must_use]
    pub fn is_transmitter_fifo_empty(&self) -> bool {
        read_address_bit(self.address_channel_status, 3)
    }

    /// True if receiver FIFO is full.
    #[inline]
    #[must_use]
    pub fn is_receiver_fifo_full(&self) -> bool {
        read_address_bit(self.address_channel_status, 2)
    }

    /// True if receiver FIFO is empty.
    #[inline]
    #[must_use]
    pub fn is_receiver_fifo_empty(&self) -> bool {
        read_address_bit(self.address_channel_status, 1)
    }

    /// True if receiver FIFO has reached trigger level.
    #[must_use]
    pub fn is_receiver_fifo_trigger_reached(&self) -> bool {
        read_address_bit(self.address_channel_status, 0)
    }

    /// Configure UART with default configuration.
    ///
    /// 1. Enable AMBA and reference clocks.
    /// 2. Reset peripheral.
    /// 3. Use input clock without prescaling.
    /// 4. Disable parity bits.
    /// 5. Use one stop bit.
    /// 6. Use standard UART channel mode.
    ///
    /// # Errors
    ///
    /// - System level control registers are locked and they can not be unlocked.
    #[inline]
    #[must_use]
    pub fn configure(&self) -> Result<(), ()> {
        use crate::peripheral::slcr::AmbaClockControl;
        use crate::peripheral::slcr::SLCR;

        // Check that system level control registers are unlocked.
        if unsafe { SLCR.is_system_level_configuration_registers_locked() } {
            unsafe { SLCR.toggle_system_level_configuration_registers(false) };
        }
        if unsafe { SLCR.is_system_level_configuration_registers_locked() } {
            return Err(());
        }
        // Enable AMBA and reference clocks.
        let target = match self.index {
            DeviceIndex::Uart0 => AmbaClockControl::Uart0,
            DeviceIndex::Uart1 => AmbaClockControl::Uart1,
        };
        unsafe { SLCR.toggle_amba_clocks(target, true) };
        match self.index {
            DeviceIndex::Uart0 => {
                unsafe { SLCR.toggle_uart_0_reference_clock(true) };
            }
            DeviceIndex::Uart1 => {
                unsafe { SLCR.toggle_uart_1_reference_clock(true) };
            }
        }

        self.toggle(false);
        self.reset();
        self.set_clock_source(ClockSource::UartRefClk);
        self.set_character_length(CharacterLength::Eight);
        self.set_parity(ParityType::Disabled);
        self.set_stop_bits(StopBits::One);
        self.set_channel_mode(ChannelMode::Normal);
        Ok(())
    }

    // TODO:
    // host can do useful work when transmitting multiple bytes
    // 1. send byte 0..N
    // 2. FIFO is full, continue other work
    // 3. FIFO empty interrupt -> send bytes N..M

    /// Transmit one byte.
    ///
    /// This function blocks.
    #[inline]
    pub fn transmit_byte(&self, byte: u8) {
        // Wait until transmit buffer has space for more bytes.
        while self.is_transmitter_fifo_full() {}
        write_to_address(self.address_transmit_and_receive_fifo, byte as u32);
    }

    /// Transmit string.
    ///
    /// This function blocks.
    #[inline]
    pub fn transmit_string(&self, string: &str) {
        for byte in string.as_bytes() {
            self.transmit_byte(*byte);
        }
    }

    /// Transmit line.
    ///
    /// This function blocks.
    #[inline]
    pub fn transmit_line(&self, line: &str) {
        self.transmit_string(line);
        self.transmit_string("\r\n");
    }

    /// Receive one byte.
    ///
    /// This function blocks.
    #[must_use]
    pub fn receive_byte(&self) -> u8 {
        while self.is_receiver_fifo_empty() {}
        let value = read_from_address(self.address_transmit_and_receive_fifo);
        value as u8
    }

    /* TODO: requires heapless string
    pub fn receive_string(&self) -> &str {}

    pub fn receive_line(&self) -> &str {}
    */

    /// Try to receive one byte.
    #[inline]
    #[must_use]
    pub fn try_receive_byte(&self) -> Option<u8> {
        if self.is_receiver_fifo_empty() {
            None
        } else {
            let value = read_from_address(self.address_transmit_and_receive_fifo);
            let byte = value as u8;
            Some(byte)
        }
    }

    /*
    /// Clear all UART interrupt causes.
    #[inline]
    pub fn clear_all_interrupt_causes(&self) {
        write_to_address(self.address_channel_interrupt_status, 0xFFFF_FFFF);
    }
    */

    // TODO: set baud rate
    /*pub fn set_baud_rate(&self) {
        self.toggle(false);
        self.reset();
    }*/
}

/// UART 0 base address.
const ADDRESS_UART0_BASE: u32 = 0xE000_0000;
/// UART 1 base address.
const ADDRESS_UART1_BASE: u32 = 0xE000_1000;

/// UART 0 peripheral.
pub static mut UART0: Uart = Uart {
    index: DeviceIndex::Uart0,
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
    index: DeviceIndex::Uart1,
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
        unsafe { UART0.transmit_string(s) }
        Ok(())
    }
}

/// Print formatted string using [`UART0`](crate::peripheral::uart::UART0).
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

/// Print formatted line using [`UART0`](crate::peripheral::uart::UART0).
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
