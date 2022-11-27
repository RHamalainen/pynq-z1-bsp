//! General interrupt controller.

#![allow(unused)]
#![allow(clippy::missing_docs_in_private_items)]

// Board uses 5 bits for priority.
// The lower the value, the higher the priority.

/// Base address for memory mapped interrupt controller distributor.
pub const ADDRESS_ICD_BASE: u32 = 0xF8F0_1000;

/// General Interrupt Controller (GIC).
pub struct Gic {
    pub address_distributor_control: *mut u32,
    pub address_interrupt_controller_type: *mut u32,
    pub address_distributor_implementer_identification: *mut u32,
    pub addresses_interrupt_security: [u32; 3],
    pub addresses_interrupt_set_enable: [u32; 3],
    pub addresses_interrupt_clear_enable: [u32; 3],
    pub addresses_interrupt_set_pending: [u32; 3],
    pub addresses_interrupt_clear_pending: [u32; 3],
    pub addresses_active_bit: [u32; 3],
    pub addresses_interrupt_priority: [u32; 24],
    pub addresses_interrupt_processor_targets: [u32; 24],
    pub addresses_interrupt_configuration: [u32; 6],
    pub address_software_generated_interrupt: *mut u32,
}

impl Gic {
    // TODO
}

const ADDRESS_ICDISR_START: u32 = ADDRESS_ICD_BASE + 0x080;
const ADDRESS_ICDISR_FINAL: u32 = ADDRESS_ICD_BASE + 0x088;
const ADDRESSES_ICDISR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0080,
    ADDRESS_ICD_BASE + 0x0000_0084,
    ADDRESS_ICD_BASE + 0x0000_0088,
];

const ADDRESS_ISER_START: u32 = ADDRESS_ICD_BASE + 0x100;
const ADDRESS_ISER_FINAL: u32 = ADDRESS_ICD_BASE + 0x108;
const ADDRESSES_ISER: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0100,
    ADDRESS_ICD_BASE + 0x0000_0104,
    ADDRESS_ICD_BASE + 0x0000_0108,
];

const ADDRESS_ICER_START: u32 = ADDRESS_ICD_BASE + 0x180;
const ADDRESS_ICER_FINAL: u32 = ADDRESS_ICD_BASE + 0x188;
const ADDRESSES_ICER: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0180,
    ADDRESS_ICD_BASE + 0x0000_0184,
    ADDRESS_ICD_BASE + 0x0000_0188,
];

const ADDRESS_ISPR_START: u32 = ADDRESS_ICD_BASE + 0x200;
const ADDRESS_ISPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x208;
const ADDRESSES_ISPR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0200,
    ADDRESS_ICD_BASE + 0x0000_0204,
    ADDRESS_ICD_BASE + 0x0000_0208,
];

const ADDRESS_ICPR_START: u32 = ADDRESS_ICD_BASE + 0x280;
const ADDRESS_ICPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x288;
const ADDRESSES_ICPR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0280,
    ADDRESS_ICD_BASE + 0x0000_0284,
    ADDRESS_ICD_BASE + 0x0000_0288,
];

const ADDRESS_ABR_START: u32 = ADDRESS_ICD_BASE + 0x300;
const ADDRESS_ABR_FINAL: u32 = ADDRESS_ICD_BASE + 0x308;
const ADDRESSES_ABR: [u32; 3] = [
    ADDRESS_ICD_BASE + 0x0000_0300,
    ADDRESS_ICD_BASE + 0x0000_0304,
    ADDRESS_ICD_BASE + 0x0000_0308,
];

const ADDRESS_IPR_START: u32 = ADDRESS_ICD_BASE + 0x400;
const ADDRESS_IPR_FINAL: u32 = ADDRESS_ICD_BASE + 0x45C;
const ADDRESSES_IPR: [u32; 24] = [
    ADDRESS_ICD_BASE + 0x0000_0400,
    ADDRESS_ICD_BASE + 0x0000_0404,
    ADDRESS_ICD_BASE + 0x0000_0408,
    ADDRESS_ICD_BASE + 0x0000_040C,
    ADDRESS_ICD_BASE + 0x0000_0410,
    ADDRESS_ICD_BASE + 0x0000_0414,
    ADDRESS_ICD_BASE + 0x0000_0418,
    ADDRESS_ICD_BASE + 0x0000_041C,
    ADDRESS_ICD_BASE + 0x0000_0420,
    ADDRESS_ICD_BASE + 0x0000_0424,
    ADDRESS_ICD_BASE + 0x0000_0428,
    ADDRESS_ICD_BASE + 0x0000_042C,
    ADDRESS_ICD_BASE + 0x0000_0430,
    ADDRESS_ICD_BASE + 0x0000_0434,
    ADDRESS_ICD_BASE + 0x0000_0438,
    ADDRESS_ICD_BASE + 0x0000_043C,
    ADDRESS_ICD_BASE + 0x0000_0440,
    ADDRESS_ICD_BASE + 0x0000_0444,
    ADDRESS_ICD_BASE + 0x0000_0448,
    ADDRESS_ICD_BASE + 0x0000_044C,
    ADDRESS_ICD_BASE + 0x0000_0450,
    ADDRESS_ICD_BASE + 0x0000_0454,
    ADDRESS_ICD_BASE + 0x0000_0458,
    ADDRESS_ICD_BASE + 0x0000_045C,
];

const ADDRESS_IPTR_START: u32 = ADDRESS_ICD_BASE + 0x800;
const ADDRESS_IPTR_FINAL: u32 = ADDRESS_ICD_BASE + 0x85C;
const ADDRESSES_IPTR: [u32; 24] = [
    ADDRESS_ICD_BASE + 0x0000_0800,
    ADDRESS_ICD_BASE + 0x0000_0804,
    ADDRESS_ICD_BASE + 0x0000_0808,
    ADDRESS_ICD_BASE + 0x0000_080C,
    ADDRESS_ICD_BASE + 0x0000_0810,
    ADDRESS_ICD_BASE + 0x0000_0814,
    ADDRESS_ICD_BASE + 0x0000_0818,
    ADDRESS_ICD_BASE + 0x0000_081C,
    ADDRESS_ICD_BASE + 0x0000_0820,
    ADDRESS_ICD_BASE + 0x0000_0824,
    ADDRESS_ICD_BASE + 0x0000_0828,
    ADDRESS_ICD_BASE + 0x0000_082C,
    ADDRESS_ICD_BASE + 0x0000_0830,
    ADDRESS_ICD_BASE + 0x0000_0834,
    ADDRESS_ICD_BASE + 0x0000_0838,
    ADDRESS_ICD_BASE + 0x0000_083C,
    ADDRESS_ICD_BASE + 0x0000_0840,
    ADDRESS_ICD_BASE + 0x0000_0844,
    ADDRESS_ICD_BASE + 0x0000_0848,
    ADDRESS_ICD_BASE + 0x0000_084C,
    ADDRESS_ICD_BASE + 0x0000_0850,
    ADDRESS_ICD_BASE + 0x0000_0854,
    ADDRESS_ICD_BASE + 0x0000_0858,
    ADDRESS_ICD_BASE + 0x0000_085C,
];

const ADDRESS_ICFR_START: u32 = ADDRESS_ICD_BASE + 0xC00;
const ADDRESS_ICFR_FINAL: u32 = ADDRESS_ICD_BASE + 0xC14;
const ADDRESSES_ICFR: [u32; 6] = [
    ADDRESS_ICD_BASE + 0x0000_0C00,
    ADDRESS_ICD_BASE + 0x0000_0C04,
    ADDRESS_ICD_BASE + 0x0000_0C08,
    ADDRESS_ICD_BASE + 0x0000_0C0C,
    ADDRESS_ICD_BASE + 0x0000_0C10,
    ADDRESS_ICD_BASE + 0x0000_0C14,
];

/// General interrupt controller.
pub static mut GIC: Gic = Gic {
    address_distributor_control: (ADDRESS_ICD_BASE + 0x000) as *mut u32,
    address_interrupt_controller_type: (ADDRESS_ICD_BASE + 0x004) as *mut u32,
    address_distributor_implementer_identification: (ADDRESS_ICD_BASE + 0x008) as *mut u32,
    addresses_interrupt_security: ADDRESSES_ICDISR,
    addresses_interrupt_set_enable: ADDRESSES_ISER,
    addresses_interrupt_clear_enable: ADDRESSES_ICER,
    addresses_interrupt_set_pending: ADDRESSES_ISPR,
    addresses_interrupt_clear_pending: ADDRESSES_ICPR,
    addresses_active_bit: ADDRESSES_ABR,
    addresses_interrupt_priority: ADDRESSES_IPR,
    addresses_interrupt_processor_targets: ADDRESSES_IPTR,
    addresses_interrupt_configuration: ADDRESSES_ICFR,
    address_software_generated_interrupt: (ADDRESS_ICD_BASE + 0xF00) as *mut u32,
};
