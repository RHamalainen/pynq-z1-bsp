//! "Hello, World!" for PYNQ-Z1.

#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[no_mangle]
#[inline(never)]
#[link_section = ".text"]
fn main() {
    unsafe { asm!("nop") };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
