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
}
