//! IRQ handler.

use crate::common::bitman::ReadBitwiseRange;

/// Base address of ICC.
pub const ADDRESS_ICC_BASE: u32 = 0xF8F0_0100;
/// Interrupt acknowledge register.
pub const ADDRESS_ICC_IAR: *mut u32 = (ADDRESS_ICC_BASE + 0x0C) as *mut u32;
/// End of interrupt register.
pub const ADDRESS_ICC_EOIR: *mut u32 = (ADDRESS_ICC_BASE + 0x10) as *mut u32;

pub struct IrqHandler {
    pub handle_global_timer: fn(),
    pub handle_nfiq: fn(),
    pub handle_private_timer: fn(),
    pub handle_watchdog_timer: fn(),
    pub handle_nirq: fn(),

    pub handle_ttc0_0: fn(),

    pub handle_gpio: fn(),

    pub handle_uart0: fn(),
    pub handle_uart1: fn(),
}

pub static mut IRQ_HANDLER: IrqHandler = unsafe {
    IrqHandler {
        handle_global_timer: || {},
        handle_nfiq: || {},
        handle_private_timer: || {},
        handle_watchdog_timer: || {},
        handle_nirq: || {},

        handle_ttc0_0: || {},

        handle_gpio: || {},

        handle_uart0: || {},
        handle_uart1: || {},
    }
};

/// Handle IRQ interrupt.
#[no_mangle]
#[inline(never)]
fn handle_irq() {
    use crate::interrupt::icc::ICC;
    use crate::interrupt::irq_numbers::ppi;
    use crate::interrupt::irq_numbers::Irq;
    use crate::peripheral::uart::UART0;

    // TODO: read into structure
    let iar = unsafe { ICC.acknowledge_interrupt() };

    // TODO: read into structure
    let interrupt_id = iar.read_bits(0..=9);
    match Irq::from_u32(interrupt_id) {
        Irq::IrqGlobalTimer => unsafe { (IRQ_HANDLER.handle_global_timer)() },
        Irq::IrqNFiq => unsafe { (IRQ_HANDLER.handle_nfiq)() },
        Irq::IrqCpuPrivateTimer => unsafe { (IRQ_HANDLER.handle_private_timer)() },
        Irq::IrqAwdt => unsafe { (IRQ_HANDLER.handle_watchdog_timer)() },
        Irq::IrqNIrq => unsafe { (IRQ_HANDLER.handle_nirq)() },

        Irq::IrqTtc00 => unsafe { (IRQ_HANDLER.handle_ttc0_0)() },

        Irq::IrqGpio => unsafe { (IRQ_HANDLER.handle_gpio)() },

        Irq::IrqUart0 => unsafe { (IRQ_HANDLER.handle_uart0)() },
        Irq::IrqUart1 => unsafe { (IRQ_HANDLER.handle_uart1)() },
        _ => (),
    }
    unsafe {
        ICC.complete_interrupt(iar);
    }
}
