//! GPIO LED blinker for PYNQ-Z1.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use pynq_z1_bsp::common::instruction::nop;
use pynq_z1_bsp::peripheral::gpio::PinDirection;
use pynq_z1_bsp::peripheral::gpio::GPIO;
use pynq_z1_bsp::peripheral::uart::UART0;

#[no_mangle]
#[inline(never)]
#[link_section = ".text"]
fn main() {
    unsafe {
        GPIO.set_mio_direction(0, PinDirection::Input);
        GPIO.set_mio_direction(1, PinDirection::Output);
        GPIO.toggle_mio_interrupt(0, true);
        GPIO.toggle_mio_interrupt(1, true);

        UART0.configure().unwrap();
        UART0.toggle(true);
    }

    loop {
        for _ in 0..1_000 {
            nop();
        }
        unsafe {
            if GPIO.read_mio_input(0) {
                UART0.transmit_line("MIO pin 0 is high.");
            } else {
                UART0.transmit_line("MIO pin 0 is low.");
            }
            if GPIO.read_mio_output(1) {
                GPIO.write_mio_output(1, false);
            } else {
                GPIO.write_mio_output(1, true);
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
