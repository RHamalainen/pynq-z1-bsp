//! Interact with the board using UART input and output.

#![no_std]
#![no_main]

#[no_mangle]
#[inline(never)]
fn read_uart() {
    use pynq_z1_bsp::peripheral::uart::UART0;
    use pynq_z1_bsp::sprintln;

    while let Some(byte) = unsafe { UART0.try_receive_byte() } {
        let character = char::from(byte);
        sprintln!("received: {character}");
    }
}

#[no_mangle]
#[inline(never)]
fn handle_uart0() {
    use pynq_z1_bsp::peripheral::uart::Interrupt;
    use pynq_z1_bsp::peripheral::uart::ReceiverInterrupt;
    use pynq_z1_bsp::peripheral::uart::UART0;

    let causes = unsafe { UART0.read_interrupt_causes() };
    if causes.receiver_fifo_trigger || causes.receiver_fifo_full {
        read_uart();
        unsafe {
            UART0.clear_interrupt(Interrupt::Receiver(ReceiverInterrupt::FifoTrigger));
            UART0.clear_interrupt(Interrupt::Receiver(ReceiverInterrupt::FifoFull));
        }
    }
}

#[no_mangle]
#[inline(never)]
fn setup() {
    use pynq_z1_bsp::interrupt::gic::InterruptTargets;
    use pynq_z1_bsp::interrupt::gic::GIC;
    use pynq_z1_bsp::interrupt::handler::irq::IRQ_HANDLER;
    use pynq_z1_bsp::interrupt::icc::InterruptPriorityFilter;
    use pynq_z1_bsp::interrupt::icc::ICC;
    use pynq_z1_bsp::interrupt::irq_numbers::Irq;
    use pynq_z1_bsp::interrupt::irq_numbers::SpiIrq;
    use pynq_z1_bsp::peripheral::uart::Interrupt;
    use pynq_z1_bsp::peripheral::uart::ReceiverInterrupt;
    use pynq_z1_bsp::peripheral::uart::UART0;

    unsafe {
        IRQ_HANDLER.handle_uart0 = handle_uart0;

        // TODO: GIC.reset();
        GIC.toggle_interrupt(Irq::Spi(SpiIrq::Uart0), true);
        GIC.set_shared_peripheral_interrupt_targets(SpiIrq::Uart0, InterruptTargets::Cpu0);
        GIC.toggle(true);

        ICC.set_interrupt_priority_filter(InterruptPriorityFilter::AllowAll);
        ICC.toggle(true);

        // TODO: handle
        let _ = UART0.configure();
        UART0.toggle_interrupt(Interrupt::Receiver(ReceiverInterrupt::FifoTrigger), true);
        UART0.toggle_interrupt(Interrupt::Receiver(ReceiverInterrupt::FifoFull), true);
        let _ = UART0.set_receiver_fifo_trigger_value(1);
        UART0.toggle(true);

        // TODO: remove this
        core::arch::asm!(
            // Read CPSR.
            "mrs {temp}, cpsr",
            // Enable interrupts.
            "bic {temp}, {temp}, 0b10000000",
            // Write CPSR.
            "msr cpsr, {temp}",
            temp = in(reg) 0,
        );
    }
}

#[no_mangle]
#[inline(never)]
fn main() {
    setup();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use pynq_z1_bsp::peripheral::uart::UART0;
    use pynq_z1_bsp::sprintln;

    unsafe {
        // TODO: handle
        let _ = UART0.configure();
        UART0.toggle(true);
    }
    sprintln!("Panic: {info}");
    loop {}
}
