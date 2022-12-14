//! Direct memory access.

/*

8 concurrent DMA channels threads


1. CPU initiates transfer
    - How many words to transfer.
    - What memory address to use.
2. CPU does other things while transfer is happening
3. DMA generates interrupt when transfer is ready

DMAC includes small variable-length instruction set
    - code is stored to memory DMAC accesses using its AXI master interface
    - DMAC stores instructions temporarily in cache

dual APB slave interfaces, secure (s) and non-secure (ns) for accessing registers of DMAC

*/

pub mod channel;
pub mod manager;

use crate::common::bitman::ClearBitwise;
use crate::common::bitman::SetBitwise;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::read_from_address;
use crate::common::memman::write_to_address;
use channel::Channel;
use manager::Manager;

// s
//const ADDRESS_DMA_CONTROLLER_BASE: u32 = 0xF800_3000;

// ns
const ADDRESS_DMA_CONTROLLER_BASE: u32 = 0xF800_4000;

enum SecurityStatus {
    Secure,
    NonSecure,
}

impl SecurityStatus {
    fn from_bool(value: bool) -> Self {
        if value {
            Self::NonSecure
        } else {
            Self::Secure
        }
    }
}

enum ChannelId {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
}

impl ChannelId {
    fn to_u32(self) -> u32 {
        match self {
            Self::Channel0 => 0,
            Self::Channel1 => 1,
            Self::Channel2 => 2,
            Self::Channel3 => 3,
            Self::Channel4 => 4,
            Self::Channel5 => 5,
            Self::Channel6 => 6,
            Self::Channel7 => 7,
        }
    }
}

/// Interface for a DMA controller peripheral.
struct DmaController {
    /// DMA manager thread.
    manager: Manager,

    /// DMA channel threads.
    channels: [Channel; 8],
}

impl DmaController {}

/// DMA controller peripheral.
static mut DMA_CONTROLLER: DmaController = unsafe {
    DmaController {
        manager: manager::MANAGER,
        channels: [
            channel::DMA_CHANNEL_0,
            channel::DMA_CHANNEL_1,
            channel::DMA_CHANNEL_2,
            channel::DMA_CHANNEL_3,
            channel::DMA_CHANNEL_4,
            channel::DMA_CHANNEL_5,
            channel::DMA_CHANNEL_6,
            channel::DMA_CHANNEL_7,
        ],
    }
};
