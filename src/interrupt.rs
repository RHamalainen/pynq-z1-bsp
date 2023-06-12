//! Interrupt functionality.
//!
//! # How to use?
//!
//! To enable routing and detection of interrupt requests, you have to configure both `GIC` and `ICC`.
//!
//! ```ignore
//! GIC.toggle_enable(false);
//! ICC.toggle_enable(false);
//!
//! GIC.toggle_interrupt_enable(<irq number>, true);
//! GIC.set_interrupt_targets(<irq number>, 0b1);
//! GIC.set_interrupt_priority(<irq number>, 0b0);
//!
//! ICC.set_interrupt_priority_filter(0b1111_1111);
//!
//! ICC.toggle_enable(true);
//! GIC.toggle_enable(true);
//! ```

pub mod gic;
pub mod handler;
pub mod icc;
pub mod irq_numbers;
