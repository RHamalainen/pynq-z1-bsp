//! "Hello, World!" for PYNQ-Z1.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use pynq_z1_bsp::common::instruction::nop;
use pynq_z1_bsp::peripheral::uart::UART0;

#[no_mangle]
#[inline(never)]
#[link_section = ".text"]
fn main() {
    unsafe {
        UART0.configure().unwrap();
        UART0.toggle(true);
    }
    loop {
        for _ in 0..1_000 {
            nop();
        }
        unsafe { UART0.transmit_line("Hello, World!") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
