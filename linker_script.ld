STACK_SIZE_SYSTEM = 0x800; /* 2 kB */
STACK_SIZE_IRQ = 0x800; /* 2 kB */

MEMORY {
    /* source: Zynq-7000 TRM */
    RAM (WX) : ORIGIN = 0x0, LENGTH = 0x3FFFF
}

ENTRY(_interrupt_vector)

SECTIONS {
    .text : {
        *(.interrupt_vector)
        *(.text)
    } > RAM

    .rodata : {
        _rodata_start = .;
        *(.rodata);
        _rodata_final = .;
    } > RAM

    .data : {
        _data_start = .;
        *(.data);
        _data_final = .;
    } > RAM

    .bss : {
        _bss_start = .;
        *(.bss);
        _bss_final = .;
    } > RAM

    /* ARM procedure call standard requires 8 byte alignment */
    /* TODO: verify this */
    .stack : {
        . = ALIGN(8);
        _stack_final_irq = .;
        . = . + STACK_SIZE_IRQ;
        . = ALIGN(8);
        _stack_start_irq = .;
        _stack_final_system = .;
        . = . + STACK_SIZE_SYSTEM;
        . = ALIGN(8);
        _stack_start_system = .;
    } > RAM
}
