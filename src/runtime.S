.section .interrupt_vector
.global _interrupt_vector
_interrupt_vector:
    /* reset */
    b _boot
    /* undefined instruction */
    b _abort
    /* supervisor call */
    b _abort
    /* secure monitor call */
    b _abort
    /* hypervisor call */
    b _abort
    /* prefetch abort */
    b _abort
    /* data abort */
    b _abort
    /* irq */
    b _abort
    /* fiq */
    b _abort

.section .text
.global _boot
_boot:
    bl main
    b _boot

.section .text
.global _abort
_abort:
    b _abort
