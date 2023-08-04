//! Commonly used instructions.

use core::arch::asm;

/// Perform no-operation.
#[inline]
pub fn nop() {
    // Safety:
    // Does not cause any side-effects.
    unsafe { asm!("nop") };
}

/// Return control to debugger.
#[inline]
pub fn breakpoint() {
    // Safety:
    // Does not cause any side-effects.
    unsafe { asm!("bkpt") };
}
