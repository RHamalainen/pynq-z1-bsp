//! DMA channel thread.

use crate::common::bitman::ClearBitwise;
use crate::common::bitman::SetBitwise;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::read_from_address;
use crate::common::memman::write_to_address;
use crate::peripheral::dma::SecurityStatus;
use crate::peripheral::dma::ADDRESS_DMA_CONTROLLER_BASE;

/// DMA channel status.
enum ChannelStatus {
    /// Thread has invalid PC and is not fetching instructions.
    Stopped,

    /// Thread has valid PC and DMAC includes the thread when it arbitrates.
    Executing,

    /// Thread is stalled while DMAC is performing a cache line fill.
    CacheMiss,

    /// DMAC is calculating the address of the next access in the cache.
    UpdatingPC,

    /// Thread is stalled and is waiting for DMAC to execute `DMASEV` using the corresponding event number.
    WaitingForEvent,

    /// Thread is stalled and DMAC is waiting for transactions on the AXI bus to complete.
    AtBarrier,

    /// Thread is stalled and DMAC is waiting for peripheral to provide requested data.
    WaitingForPeripheral,

    /// Thread is waiting for AXI master interface to signal that the outstanding load or store transactions are complete.
    Killing,

    /// Thread is waiting for AXI master interface to signal that the outstanding load or store transactions are complete.
    Completing,

    /// Thread is waiting for AXI master interface to signal that the outstanding load or store transactions are complete.
    FaultingCompleting,

    /// Thread is stalled indefinitely.
    Faulting,
}

impl ChannelStatus {
    fn from_u32(value: u32) -> Self {
        todo!()
    }
}

enum OperandSet {
    Single,
    Burst,
}

impl OperandSet {
    fn from_bool(value: bool) -> Self {
        if value {
            OperandSet::Burst
        } else {
            OperandSet::Single
        }
    }
}

pub enum FaultType {}

/// Interface for a DMA channel.
#[derive(Clone, Copy)]
pub struct Channel {
    /// Contains type of fault that caused DMA channel to transition to faulting state.
    address_fault_type: *mut u32,

    /// Status of DMA program on a DMA channel.
    address_status: *mut u32,

    /// Channel program counter.
    address_program_counter: *mut u32,

    /// Address where data is read from.
    address_source_address: *mut u32,

    /// Address where data is written to.
    address_destination_address: *mut u32,

    /// Controls AXI transactions.
    address_channel_control: *mut u32,

    /// Status of loop counter 0.
    address_loop_counter_0: *mut u32,

    /// Status of loop counter 1.
    address_loop_counter_1: *mut u32,
}

impl Channel {
    fn fault_type(&self) -> FaultType {
        todo!()
    }

    fn status(&self) -> ChannelStatus {
        let value = read_address_bits(self.address_status, 0..=3);
        ChannelStatus::from_u32(value)
    }

    fn wakeup_number(&self) -> u32 {
        read_address_bits(self.address_status, 4..=8)
    }

    fn used_operand_set(&self) -> OperandSet {
        let value = read_address_bit(self.address_status, 14);
        OperandSet::from_bool(value)
    }

    fn used_peripheral_operand_set(&self) -> bool {
        read_address_bit(self.address_status, 15)
    }

    fn security_status(&self) -> SecurityStatus {
        let value = read_address_bit(self.address_status, 21);
        SecurityStatus::from_bool(value)
    }

    fn program_counter(&self) -> u32 {
        read_from_address(self.address_program_counter)
    }

    fn source_address(&self) -> u32 {
        read_from_address(self.address_source_address)
    }

    fn destination_address(&self) -> u32 {
        read_from_address(self.address_destination_address)
    }

    // TODO control

    fn loop_counter_0(&self) -> u32 {
        read_address_bits(self.address_loop_counter_0, 0..=7)
    }

    fn loop_counter_1(&self) -> u32 {
        read_address_bits(self.address_loop_counter_1, 0..=7)
    }
}

#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_0: u32 = 0x400;
#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_1: u32 = 0x420;
#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_2: u32 = 0x440;
#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_3: u32 = 0x460;
#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_4: u32 = 0x480;
#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_5: u32 = 0x4A0;
#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_6: u32 = 0x4C0;
#[allow(clippy::missing_docs_in_private_items)]
const OFFSET_CHANNEL_7: u32 = 0x4E0;

// TODO: remove DMA_ prefix

/// DMA channel 0.
pub static mut DMA_CHANNEL_0: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x040) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x100) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x104) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_0 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_0 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_0 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_0 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_0 + 0x10) as *mut u32,
};

/// DMA channel 1.
pub static mut DMA_CHANNEL_1: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x044) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x108) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x10C) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_1 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_1 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_1 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_1 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_1 + 0x10) as *mut u32,
};

/// DMA channel 2.
pub static mut DMA_CHANNEL_2: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x048) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x110) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x114) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_2 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_2 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_2 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_2 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_2 + 0x10) as *mut u32,
};

/// DMA channel 3.
pub static mut DMA_CHANNEL_3: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x04C) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x118) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x11C) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_3 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_3 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_3 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_3 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_3 + 0x10) as *mut u32,
};

/// DMA channel 4.
pub static mut DMA_CHANNEL_4: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x050) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x120) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x124) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_4 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_4 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_4 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_4 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_4 + 0x10) as *mut u32,
};

/// DMA channel 5.
pub static mut DMA_CHANNEL_5: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x054) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x128) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x12C) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_5 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_5 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_5 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_5 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_5 + 0x10) as *mut u32,
};

/// DMA channel 6.
pub static mut DMA_CHANNEL_6: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x058) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x130) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x134) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_6 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_6 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_6 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_6 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_6 + 0x10) as *mut u32,
};

/// DMA channel 7.
pub static mut DMA_CHANNEL_7: Channel = Channel {
    address_fault_type: (ADDRESS_DMA_CONTROLLER_BASE + 0x05C) as *mut u32,
    address_status: (ADDRESS_DMA_CONTROLLER_BASE + 0x138) as *mut u32,
    address_program_counter: (ADDRESS_DMA_CONTROLLER_BASE + 0x13C) as *mut u32,
    address_source_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_7 + 0x00) as *mut u32,
    address_destination_address: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_7 + 0x04)
        as *mut u32,
    address_channel_control: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_7 + 0x08) as *mut u32,
    address_loop_counter_0: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_7 + 0x0C) as *mut u32,
    address_loop_counter_1: (ADDRESS_DMA_CONTROLLER_BASE + OFFSET_CHANNEL_7 + 0x10) as *mut u32,
};
