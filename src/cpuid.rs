//! CPUID identification scheme.

use core::arch::asm;

/*
from copro to reg
MRC<c> <coproc>, <opc1>, <Rt>, <CRn>, <CRm>{, <opc2>}

from reg to copro
MCR<c> <coproc>, <opc1>, <Rt>, <CRn>, <CRm>{, <opc2>}
*/

enum CpuIdRegister {
    /* CRn == c0 */
    /* opc1 == 0 */

    /* CRm == c1 */
    /// Processor feature register 0.
    PFR0,
    /// Processor feature register 1.
    PFR1,
    /// Debug feature register 0.
    DFR0,
    /// Auxiliary feature register 0.
    AFR0,
    /// Memory model feature register 0.
    MMFR0,
    /// Memory model feature register 1.
    MMFR1,
    /// Memory model feature register 2.
    MMFR2,
    /// Memory model feature register 3.
    MMFR3,

    /* CRm == c2 */
    /// ISA feature register 0.
    ISAR0,
    /// ISA feature register 1.
    ISAR1,
    /// ISA feature register 2.
    ISAR2,
    /// ISA feature register 3.
    ISAR3,
    /// ISA feature register 4.
    ISAR4,
    /// ISA feature register 5.
    ISAR5,
}

impl CpuIdRegister {}

fn isar0() -> u32 {
    let x: u32;
    unsafe {
        asm!("mrc p15, 0, {x}, c0, c2, 0", x=out(reg) x);
    }
    x
}

fn isar1() -> u32 {
    let x: u32;
    unsafe {
        asm!("mrc p15, 0, {x}, c0, c2, 1", x=out(reg) x);
    }
    x
}

fn isar2() -> u32 {
    let x: u32;
    unsafe {
        asm!("mrc p15, 0, {x}, c0, c2, 2", x=out(reg) x);
    }
    x
}

fn isar3() -> u32 {
    let x: u32;
    unsafe {
        asm!("mrc p15, 0, {x}, c0, c2, 3", x=out(reg) x);
    }
    x
}

fn isar4() -> u32 {
    let x: u32;
    unsafe {
        asm!("mrc p15, 0, {x}, c0, c2, 4", x=out(reg) x);
    }
    x
}

fn isar5() -> u32 {
    let x: u32;
    unsafe {
        asm!("mrc p15, 0, {x}, c0, c2, 5", x=out(reg) x);
    }
    x
}
