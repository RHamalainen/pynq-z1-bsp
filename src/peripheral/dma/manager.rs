//! DMA manager thread.

use crate::common::bitman::ClearBitwise;
use crate::common::bitman::SetBitwise;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::read_from_address;
use crate::common::memman::write_to_address;
use crate::peripheral::dma::ChannelId;
use crate::peripheral::dma::SecurityStatus;
use crate::peripheral::dma::ADDRESS_DMA_CONTROLLER_BASE;

pub enum ManagerStatus {
    Stopped,
    Executing,
    CacheMiss,
    UpdatingPC,
    WaitingForEvent,
    Faulting,
}

impl ManagerStatus {
    fn from_u32(value: u32) -> Self {
        match value {
            0b0000 => Self::Stopped,
            0b0001 => Self::Executing,
            0b0010 => Self::CacheMiss,
            0b0011 => Self::UpdatingPC,
            0b0100 => Self::WaitingForEvent,
            0b1111 => Self::Faulting,
            unknown => panic!("Unknown DMA manager status: {}", unknown),
        }
    }
}

// TODO
pub enum Instruction {}

pub enum FaulType {
    UndefinedInstruction,
    InvalidOperand,
    InsufficientPermission(Instruction),
    ExokaySlverrDecerr,
    AbortFromSystemMemory,
    AbortFromDebugInterface,
}

/// Interface for a DMA manager.
#[derive(Clone, Copy)]
pub struct Manager {
    address_status: *mut u32,
    address_program_counter: *mut u32,
    address_interrupt_enable: *mut u32,
    address_event_interrupt_raw_status: *mut u32,
    address_interrupt_status: *mut u32,
    address_interrupt_clear: *mut u32,

    address_fault_status_manager: *mut u32,
    address_fault_status_channels: *mut u32,
    address_fault_type_manager: *mut u32,

    address_debug_status: *mut u32,
    address_debug_command: *mut u32,
    address_debug_instruction_0: *mut u32,
    address_debug_instruction_1: *mut u32,
    address_configuration_0: *mut u32,
    address_configuration_1: *mut u32,
    address_configuration_2: *mut u32,
    address_configuration_3: *mut u32,
    address_configuration_4: *mut u32,
    address_dma_configuration: *mut u32,
    address_watchdog: *mut u32,
    addresses_peripheral_identification: [*mut u32; 4],
    addresses_component_identification: [*mut u32; 4],
}

impl Manager {
    /// Read DMA manager security status.
    fn security_status(&self) -> SecurityStatus {
        let value = read_address_bit(self.address_status, 9);
        SecurityStatus::from_bool(value)
    }

    // TODO:

    /// Read DMA manager operating state.
    fn status(&self) -> ManagerStatus {
        let value = read_address_bits(self.address_status, 0..=3);
        ManagerStatus::from_u32(value)
    }

    fn toggle_interrupt(&self, interrupt: u32, enable: bool) {
        let old = read_from_address(self.address_interrupt_enable);
        let new = if enable {
            old.set_bit(interrupt)
        } else {
            old.clear_bit(interrupt)
        };
        write_to_address(self.address_interrupt_enable, new);
    }

    fn clear_interrupt(&self, interrupt: u32) {
        let old = read_from_address(self.address_event_interrupt_raw_status);
        let new = old.set_bit(interrupt);
        write_to_address(self.address_event_interrupt_raw_status, new);
    }

    /// True if manager thread is in faulting state.
    fn is_faulting(&self) -> bool {
        read_address_bit(self.address_fault_status_manager, 0)
    }

    /// True if given channel thread is in faulting or faulting completing state.
    fn is_channel_faulting(&self, channel: ChannelId) -> bool {
        let index = channel.to_u32();
        read_address_bit(self.address_fault_status_channels, index)
    }

    // TODO: what if multiple faults at same time? -> make struct with bools
    fn fault_type(&self) -> FaulType {
        let value = read_from_address(self.address_fault_type_manager);
        todo!()
    }

    fn channel_fault_type(&self, channel: ChannelId) -> FaulType {
        todo!()
    }
}

/// DMA manager.
pub static mut MANAGER: Manager = Manager {
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x000) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x004) as *mut u32,

    address_interrupt_enable: (ADDRESS_DMA_CONTROLLER_BASE + 0x020) as *mut u32,
    address_event_interrupt_raw_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x024) as *mut u32,
    address_interrupt_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x028) as *mut u32,
    address_interrupt_clear: (ADDRESS_DMA_CONTROLLER_BASE + 0x02C) as *mut u32,

    address_fault_status_manager: (ADDRESS_DMA_CONTROLLER_BASE + 0x030) as *mut u32,
    address_fault_status_channels: (ADDRESS_DMA_CONTROLLER_BASE + 0x034) as *mut u32,
    address_fault_type_manager: (ADDRESS_DMA_CONTROLLER_BASE + 0x038) as *mut u32,

    address_debug_status: (ADDRESS_DMA_CONTROLLER_BASE + 0xD00) as *mut u32,
    address_debug_command: (ADDRESS_DMA_CONTROLLER_BASE + 0xD04) as *mut u32,
    address_debug_instruction_0: (ADDRESS_DMA_CONTROLLER_BASE + 0xD08) as *mut u32,
    address_debug_instruction_1: (ADDRESS_DMA_CONTROLLER_BASE + 0xD0C) as *mut u32,

    address_configuration_0: (ADDRESS_DMA_CONTROLLER_BASE + 0xE00) as *mut u32,
    address_configuration_1: (ADDRESS_DMA_CONTROLLER_BASE + 0xE04) as *mut u32,
    address_configuration_2: (ADDRESS_DMA_CONTROLLER_BASE + 0xE08) as *mut u32,
    address_configuration_3: (ADDRESS_DMA_CONTROLLER_BASE + 0xE0C) as *mut u32,
    address_configuration_4: (ADDRESS_DMA_CONTROLLER_BASE + 0xE10) as *mut u32,
    address_dma_configuration: (ADDRESS_DMA_CONTROLLER_BASE + 0xE14) as *mut u32,
    address_watchdog: (ADDRESS_DMA_CONTROLLER_BASE + 0xE80) as *mut u32,

    addresses_peripheral_identification: [
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFE0) as *mut u32,
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFE4) as *mut u32,
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFE8) as *mut u32,
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFEC) as *mut u32,
    ],
    addresses_component_identification: [
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFF0) as *mut u32,
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFF4) as *mut u32,
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFF8) as *mut u32,
        (ADDRESS_DMA_CONTROLLER_BASE + 0xFFC) as *mut u32,
    ],
};
