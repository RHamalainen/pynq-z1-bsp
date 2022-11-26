//! Triple timer counters.

/// Interface for TTC peripheral.
pub struct TTCTimer {
    pub address_clock_control: *mut u32,
    pub address_counter_control: *mut u32,
    pub address_counter_value: *mut u32,
    pub address_interval_value: *mut u32,
    pub address_interrupt_status: *mut u32,
    pub address_interrupt_enable: *mut u32,
    pub address_event_control_timer: *mut u32,
    pub address_event: *mut u32,
}
