//! "Hello, World!" for PYNQ-Z1.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use pynq_z1_bsp::{common::instruction::nop, peripheral::uart::UART0};

#[no_mangle]
#[inline(never)]
#[link_section = ".text"]
fn main() {
    unsafe {
        UART0.configure();
        UART0.toggle(true);
        UART0.println("Hello, World!");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
