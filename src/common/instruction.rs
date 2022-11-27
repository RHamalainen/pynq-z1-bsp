//! Commonly used instructions.

use core::arch::asm;

/// Perform no-operation.
#[inline]
pub fn nop() {
    // Safety:
    // This instruction always exists and can not fail.
    unsafe { asm!("nop") };
}
