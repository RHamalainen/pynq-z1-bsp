//! IRQ handler.

use crate::common::bitman::ReadBitwiseRange;
use crate::common::memman::read_from_address;
use crate::common::memman::write_to_address;
use crate::interrupt::irq_numbers::Irq;

/// Base address of ICC.
pub const ADDRESS_ICC_BASE: u32 = 0xF8F0_0100;
/// Interrupt acknowledge register.
pub const ADDRESS_ICC_IAR: *mut u32 = (ADDRESS_ICC_BASE + 0x0C) as *mut u32;
/// End of interrupt register.
pub const ADDRESS_ICC_EOIR: *mut u32 = (ADDRESS_ICC_BASE + 0x10) as *mut u32;

/// Handle IRQ interrupt.
#[no_mangle]
#[inline(never)]
fn handle_irq() {
    let iar = read_from_address(ADDRESS_ICC_IAR);
    let interrupt_id = iar.read_bits(0..=9);
    match Irq::from_u32(interrupt_id) {
        _ => (),
    }
    write_to_address(ADDRESS_ICC_EOIR, iar);
}
