//! Memory address manipulation.

use super::bitman::{ClearBitwise, ReadBitwise, SetBitwise};
use core::ptr::{read_volatile, write_volatile};

/// Read value from memory address.
#[inline]
#[must_use]
pub fn read_from_address(address: *mut u32) -> u32 {
    // SAFETY:
    // Programmer is responsible for reading from valid address.
    unsafe { read_volatile(address) }
}

/// Write value to memory address.
#[inline]
pub fn write_to_address(address: *mut u32, value: u32) {
    // SAFETY:
    // Programmer is responsible for reading from valid address.
    unsafe { write_volatile(address, value) }
}

/// Set memory address value's bit high.
#[inline]
pub fn set_address_bit(address: *mut u32, index: u32) {
    let old = read_from_address(address);
    let new = old.set_bit(index);
    write_to_address(address, new);
}

/// Set memory address value's bit low.
#[inline]
pub fn clear_address_bit(address: *mut u32, index: u32) {
    let old = read_from_address(address);
    let new = old.clear_bit(index);
    write_to_address(address, new);
}

/// Read memory address value's bit's value.
#[inline]
#[must_use]
pub fn read_address_bit(address: *mut u32, index: u32) -> bool {
    let value = read_from_address(address);
    value.read_bit(index)
}
