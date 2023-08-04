.section .interrupt_vector
.global _interrupt_vector
_interrupt_vector:
    /* 0x00 reset */
    b _boot
    /* 0x04 undefined instruction */
_interrupt_undefined_instruction:
    b _interrupt_undefined_instruction
    /* 0x08 supervisor call */
_interrupt_supervisor_call:
    b _interrupt_supervisor_call
    /* 0x0C prefetch abort */
_interrupt_prefetch_abort:
    b _interrupt_prefetch_abort    
    /* 0x10 data abort */
_interrupt_data_abort:
    b _interrupt_data_abort    
    /* 0x14 hypervisor call */
_interrupt_hypervisor_call:
    b _interrupt_hypervisor_call    
    /* 0x18 irq */
_interrupt_irq:
    b _handle_irq
    /* 0x1C fiq */
_interrupt_fiq:
    /* TODO */
    /* b _handle_fiq */
    b _interrupt_fiq

.section .text
.global _handle_irq
_handle_irq:
    /* processor is now in IRQ mode */
    /*  - registers r0-r12, pc and cpsr contain data associated with interrupt context */
    /*  - register lr contains interrupt context's pc */
    /*  - register sp contains IRQ mode's stack */
    /*  - register spsr (saved program status register) contains interrupt context's cpsr */
    /*  - cpsr's mode bits equal to IRQ mode */
    /*  - cpsr's irq is masked */

    /* push context and return address to irq stack */
    push {r0-r12, lr}
    /* handle interrupt */
    bl handle_irq
    /* restore context from irq stack */
    pop {r0-r12, lr}
    /* return to interrupt context */
    /* cpu checks interrupt pins' status after executing current instruction */
    /* thus next instruction is already fetched to pc */
    /* thus lr contains interrupt context's next instruction and not the current */
    /* to return to current instruction, lr is corrected by moving it back one instruction */
    subs pc, lr, #4

/*.section .text
.global _handle_fiq
_handle_fiq:*/
    /* TODO: document */
    /*push {r0-r12, lr}
    bl handle_fiq
    pop {r0-r12, lr}
    subs pc, lr, #4*/

.section .text
.global _boot
_boot:
    /* TODO: maybe check security state? */
    /* disable interrupts */
    mrs r0, cpsr
    orr r0, r0, #0b111000000
    msr cpsr, r0
    /* set system mode */
    /* TODO: does this enable interrupts? */
    msr cpsr, #0b11111
    bl _initialize_bss
    /* TODO: initialize data */
    bl _initialize_stack
    bl main
_boot_trap:
    b _boot_trap

.section .text
.global _initialize_bss
_initialize_bss:
    ldr r0, =_bss_start
    ldr r1, =_bss_final
    mov r2, #0
    0:
        cmp r0, r1
        beq 1f
        /* MEM[R0] = R2 = 0 */
        /* R0 += 4 */
        stm r0!, {r2}
    1:
    bx lr

.section .text
.global _initialize_stack
_initialize_stack:
    /* initialize system mode stack */
    /* set system mode */
    msr cpsr, #0b11111
    /* set system mode stack pointer */
    ldr sp, =_stack_start_system
    /* set irq mode */
    msr cpsr, #0b10010
    /* set irq mode stack pointer */
    ldr sp, =_stack_start_irq
    /* set system mode */
    msr cpsr, #0b11111
    bx lr

/* TODO: is this needed? */
.section .text
.global _abort
_abort:
    b _abort
