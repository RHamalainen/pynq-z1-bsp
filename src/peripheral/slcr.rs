//! System level control registers.

// TODO: substructs for pll_configuration, clock_control, etc

use crate::common::bitman::SetBitwise;
use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::read_address_bits;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_address_bits;
use crate::common::memman::write_to_address;
use crate::peripheral::uart::DeviceIndex as UartDeviceIndex;

#[derive(Clone, Copy)]
pub enum Frst {
    Frst0,
    Frst1,
    Frst2,
    Frst3,
}

impl Frst {
    pub fn as_u32(self) -> u32 {
        match self {
            Self::Frst0 => 0,
            Self::Frst1 => 1,
            Self::Frst2 => 2,
            Self::Frst3 => 3,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cpu {
    Cpu0,
    Cpu1,
}

pub struct ResetCpuCommand {
    reset_cpu0: bool,
    reset_cpu1: bool,
    stop_cpu0_clock: bool,
    stop_cpu1_clock: bool,
    reset_cpu_peripherals: bool,
}

impl ResetCpuCommand {
    pub fn new() -> Self {
        Self {
            reset_cpu0: false,
            reset_cpu1: false,
            reset_cpu_peripherals: false,
            stop_cpu0_clock: false,
            stop_cpu1_clock: false,
        }
    }

    pub fn toggle_reset_cpu(&self, cpu: Cpu, enable: bool) -> Self {
        let (reset_cpu0, reset_cpu1) = match cpu {
            Cpu::Cpu0 => (true, false),
            Cpu::Cpu1 => (false, true),
        };
        Self {
            reset_cpu0,
            reset_cpu1,
            ..*self
        }
    }

    pub fn toggle_stop_cpu(&self, cpu: Cpu, enable: bool) -> Self {
        let (stop_cpu0_clock, stop_cpu1_clock) = match cpu {
            Cpu::Cpu0 => (true, false),
            Cpu::Cpu1 => (false, true),
        };
        Self {
            stop_cpu0_clock,
            stop_cpu1_clock,
            ..*self
        }
    }

    pub fn reset_cpu_peripherals(&self, enable: bool) -> Self {
        Self {
            reset_cpu_peripherals: enable,
            ..*self
        }
    }
}

#[derive(Clone, Copy)]
pub enum WatchdogIndex {
    Watchdog0,
    Watchdog1,
}

impl WatchdogIndex {
    pub fn as_u32(self) -> u32 {
        match self {
            Self::Watchdog0 => 0,
            Self::Watchdog1 => 1,
        }
    }
}

#[derive(Clone, Copy)]
pub enum WatchdogResetTarget {
    /// Watchdog resets the whole system with software reset.
    System,

    /// Watchdog resets the CPU associated with the watchdog.
    Cpu,
}

impl WatchdogResetTarget {
    pub fn as_bool(self) -> bool {
        match self {
            Self::System => false,
            Self::Cpu => true,
        }
    }
}

pub struct RebootStatus {
    /// Error code written by BootROM.
    pub bootrom_error_code: u16,

    /// Last reset was caused by system watchdog timeout.
    pub watchdog_timeout: bool,

    /// Last reset was caused by watchdog timer 0.
    pub watchdog0: bool,

    /// Last reset was caused by watchdog timer 1.
    pub watchdog1: bool,

    /// Last reset was caused by system level control register.
    pub slc_soft_reset: bool,

    /// Last reset was caused by debug system.
    pub debug_system_reset: bool,

    /// Last reset was soft reset.
    pub soft_reset: bool,

    /// Last reset was power on reset.
    pub power_on_reset: bool,

    /// Data that persists through all resets except power on reset.
    pub reboot_state: u8,
}

// TODO
pub struct BootMode {
    boot_mode: u32,
    pll_bypass: bool,
}

/// System level reset control registers.
pub struct Reset {
    address_ps_reset_control: *mut u32,
    address_ddr_reset_control: *mut u32,
    address_central_interconnect_reset_control: *mut u32,
    address_dmac_reset_control: *mut u32,
    address_usb_reset_control: *mut u32,
    address_ethernet_reset_control: *mut u32,
    address_sdio_reset_control: *mut u32,
    address_spi_reset_control: *mut u32,
    address_can_reset_control: *mut u32,
    address_i2c_reset_control: *mut u32,
    address_uart_reset_control: *mut u32,
    address_gpio_reset_control: *mut u32,
    address_quad_spi_reset_control: *mut u32,
    address_smc_reset_control: *mut u32,
    address_ocm_reset_control: *mut u32,
    address_fpga_reset_control: *mut u32,
    address_cpu_reset_and_clock_control: *mut u32,
    address_watchdog_timer_reset_control: *mut u32,
    // TODO other registers
    address_reboot_status: *mut u32,
    address_boot_mode: *mut u32,
}

impl Reset {
    /// Reset entire system.
    pub fn reset(&self) {
        // Generates reset pulse, no need to clear.
        set_address_bit(self.address_ps_reset_control, 0);
    }

    pub fn reset_ddr(&self) {
        let address = self.address_ddr_reset_control;
        set_address_bit(address, 0);
        // TODO: wait needed?
        clear_address_bit(address, 0);
    }

    pub fn reset_central_interconnect(&self) {
        let address = self.address_central_interconnect_reset_control;
        set_address_bit(address, 0);
        // TODO: wait needed?
        clear_address_bit(address, 0);
    }

    pub fn reset_dma_controller(&self) {
        let address = self.address_dmac_reset_control;
        set_address_bit(address, 0);
        // TODO: wait needed?
        clear_address_bit(address, 0);
    }

    // TODO
    /*
    // TODO: usb enum
    pub fn reset_usb(&self) {
        let address = self.address_usb_reset_control;
        //set_address_bit(address, 0);
        // TODO: wait needed?
        //clear_address_bit(address, 0);
    }

    // TODO: ethernet enum
    pub fn reset_ethernet(&self) {
        let address = self.address_ethernet_reset_control;
        //set_address_bit(address, 0);
        // TODO: wait needed?
        //clear_address_bit(address, 0);
    }

    pub fn reset_sdio(&self) {
        let address = self.address_sdio_reset_control;
    }

    pub fn reset_spi(&self) {
        let address = self.address_spi_reset_control;
    }

    pub fn reset_can(&self) {
        let address = self.address_can_reset_control;
    }

    pub fn reset_i2c(&self) {
        let address = self.address_i2c_reset_control;
    }
    */

    pub fn reset_uart(&self, uart: UartDeviceIndex, amba: bool, reference: bool) {
        let address = self.address_uart_reset_control;
        let mut value = 0u32;
        match uart {
            UartDeviceIndex::Uart0 => {
                if amba {
                    value = value.set_bit(0);
                }
                if reference {
                    value = value.set_bit(2);
                }
            }
            UartDeviceIndex::Uart1 => {
                if amba {
                    value = value.set_bit(1);
                }
                if reference {
                    value = value.set_bit(3);
                }
            }
        }
        write_to_address(address, value);
        // TODO maybe wait
        write_to_address(address, 0);
    }

    pub fn reset_gpio(&self) {
        let address = self.address_gpio_reset_control;
        set_address_bit(address, 0);
        // TODO maybe wait
        clear_address_bit(address, 0);
    }

    pub fn reset_quad_spi(&self, amba: bool, reference: bool) {
        let address = self.address_quad_spi_reset_control;
        let mut value = 0u32;
        if amba {
            value = value.set_bit(0);
        }
        if reference {
            value = value.set_bit(1);
        }
        write_to_address(address, value);
        // TODO maybe wait
        write_to_address(address, 0);
    }

    pub fn reset_smc(&self, amba: bool, reference: bool) {
        let address = self.address_smc_reset_control;
        let mut value = 0u32;
        if amba {
            value = value.set_bit(0);
        }
        if reference {
            value = value.set_bit(1);
        }
        write_to_address(address, value);
        // TODO maybe wait
        write_to_address(address, 0);
    }

    pub fn reset_ocm(&self) {
        let address = self.address_ocm_reset_control;
        set_address_bit(address, 0);
        // TODO maybe wait
        clear_address_bit(address, 0);
    }

    pub fn reset_fpga(&self, frst: Frst) {
        let address = self.address_fpga_reset_control;
        let index = frst.as_u32();
        set_address_bit(address, index);
        // TODO maybe wait
        clear_address_bit(address, index);
    }

    pub fn reset_cpu(&self, command: ResetCpuCommand) {
        let address = self.address_cpu_reset_and_clock_control;
        let mut value = 0u32;
        if command.reset_cpu0 {
            value = value.set_bit(0);
        }
        if command.reset_cpu1 {
            value = value.set_bit(1);
        }
        if command.stop_cpu0_clock {
            value = value.set_bit(4);
        }
        if command.stop_cpu1_clock {
            value = value.set_bit(5);
        }
        if command.reset_cpu_peripherals {
            value = value.set_bit(8);
        }
        write_to_address(address, value);
    }

    pub fn set_watchdog_reset_target(&self, watchdog: WatchdogIndex, route: WatchdogResetTarget) {
        let address = self.address_watchdog_timer_reset_control;
        let index = watchdog.as_u32();
        let action = if route.as_bool() {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(address, index);
    }

    pub fn reboot_status(&self) -> RebootStatus {
        let address = self.address_reboot_status;
        let bootrom_error_code = read_address_bits(address, 0..=15) as u16;
        let watchdog_timeout = read_address_bit(address, 16);
        let watchdog0 = read_address_bit(address, 17);
        let watchdog1 = read_address_bit(address, 18);
        let slc_soft_reset = read_address_bit(address, 19);
        let debug_system_reset = read_address_bit(address, 20);
        let soft_reset = read_address_bit(address, 21);
        let power_on_reset = read_address_bit(address, 22);
        let reboot_state = read_address_bits(address, 24..=31) as u8;
        RebootStatus {
            bootrom_error_code,
            watchdog_timeout,
            watchdog0,
            watchdog1,
            slc_soft_reset,
            debug_system_reset,
            soft_reset,
            power_on_reset,
            reboot_state,
        }
    }

    pub fn boot_mode(&self) -> BootMode {
        let address = self.address_boot_mode;
        let boot_mode = read_address_bits(address, 0..=3);
        let pll_bypass = read_address_bit(address, 4);
        BootMode {
            boot_mode,
            pll_bypass,
        }
    }
}

#[derive(Clone, Copy)]
pub enum AmbaClockControl {
    DmaController,
    UsbController0,
    UsbController1,
    Ethernet0,
    Ethernet1,
    Sdio0,
    Sdio1,
    Spi0,
    Spi1,
    Can0,
    Can1,
    Isc0,
    Isc1,
    Uart0,
    Uart1,
    Gpio,
    QuadSpi,
    Smc,
}

impl AmbaClockControl {
    pub fn as_u32(self) -> u32 {
        match self {
            Self::DmaController => 0,
            Self::UsbController0 => 2,
            Self::UsbController1 => 3,
            Self::Ethernet0 => 6,
            Self::Ethernet1 => 7,
            Self::Sdio0 => 10,
            Self::Sdio1 => 11,
            Self::Spi0 => 14,
            Self::Spi1 => 15,
            Self::Can0 => 16,
            Self::Can1 => 17,
            Self::Isc0 => 18,
            Self::Isc1 => 19,
            Self::Uart0 => 20,
            Self::Uart1 => 21,
            Self::Gpio => 22,
            Self::QuadSpi => 23,
            Self::Smc => 24,
        }
    }
}

/// System level control registers.
pub struct Slcr {
    pub address_secure_configuration_lock: *mut u32,
    pub address_write_protection_lock: *mut u32,
    pub address_write_protection_unlock: *mut u32,
    pub address_write_protection_status: *mut u32,
    // TODO: PLL
    // TODO: clock control
    pub address_amba_clock_control: *mut u32,
    // TODO:
    pub address_uart_clock_control: *mut u32,
    // TODO:
    reset: Reset,
}

impl Slcr {
    /// True if all writes to secure configuration registers are ignored.
    pub fn is_secure_configuration_registers_locked(&self) -> bool {
        read_address_bit(self.address_secure_configuration_lock, 0)
    }

    /// Lock secure configuration registers.
    pub fn lock_secure_configuration_registers(&self) {
        set_address_bit(self.address_secure_configuration_lock, 0);
    }

    /// Lock or unlock system level configuration registers.
    pub fn toggle_system_level_configuration_registers(&self, lock: bool) {
        let (address, key) = if lock {
            (self.address_write_protection_lock, 0x767B)
        } else {
            (self.address_write_protection_unlock, 0xDF0D)
        };
        write_address_bits(address, 0..=15, key);
    }

    /// True if system level configuration registers are locked.
    pub fn is_system_level_configuration_registers_locked(&self) -> bool {
        read_address_bit(self.address_write_protection_status, 0)
    }

    // TODO: PLL, clock control, etc.

    pub fn toggle_amba_clocks(&self, target: AmbaClockControl, enable: bool) {
        let index = target.as_u32();
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_amba_clock_control, index);
    }

    /// Enable or disable UART 0 reference clock.
    pub fn toggle_uart_0_reference_clock(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_uart_clock_control, 0);
    }

    /// Enable or disable UART 1 reference clock.
    pub fn toggle_uart_1_reference_clock(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_uart_clock_control, 1);
    }

    /*
    pub fn set_uart_pll_source(&self, source: PllSource) {}

    pub fn set_uart_pll_source_divisor(&self, divisor: u8) {
        // TODO: divisor is actually 6 bits
    }
    */

    // TODO: maybe rename better... also reset registers...
    pub fn reset(&self) -> &Reset {
        &self.reset
    }
}

const ADDRESS_BASE: u32 = 0xF800_0000;
const ADDRESS_BASE_RESET: u32 = ADDRESS_BASE + 0x200;

/// System level configuration registers.
pub static mut SLCR: Slcr = Slcr {
    address_secure_configuration_lock: (ADDRESS_BASE + 0x000) as *mut u32,
    address_write_protection_lock: (ADDRESS_BASE + 0x004) as *mut u32,
    address_write_protection_unlock: (ADDRESS_BASE + 0x008) as *mut u32,
    address_write_protection_status: (ADDRESS_BASE + 0x00C) as *mut u32,
    address_amba_clock_control: (ADDRESS_BASE + 0x12C) as *mut u32,
    address_uart_clock_control: (ADDRESS_BASE + 0x154) as *mut u32,
    reset: Reset {
        address_ps_reset_control: (ADDRESS_BASE_RESET + 0x00) as *mut u32,
        address_ddr_reset_control: (ADDRESS_BASE_RESET + 0x04) as *mut u32,
        address_central_interconnect_reset_control: (ADDRESS_BASE_RESET + 0x08) as *mut u32,
        address_dmac_reset_control: (ADDRESS_BASE_RESET + 0x0C) as *mut u32,
        address_usb_reset_control: (ADDRESS_BASE_RESET + 0x10) as *mut u32,
        address_ethernet_reset_control: (ADDRESS_BASE_RESET + 0x14) as *mut u32,
        address_sdio_reset_control: (ADDRESS_BASE_RESET + 0x18) as *mut u32,
        address_spi_reset_control: (ADDRESS_BASE_RESET + 0x1C) as *mut u32,
        address_can_reset_control: (ADDRESS_BASE_RESET + 0x20) as *mut u32,
        address_i2c_reset_control: (ADDRESS_BASE_RESET + 0x24) as *mut u32,
        address_uart_reset_control: (ADDRESS_BASE_RESET + 0x28) as *mut u32,
        address_gpio_reset_control: (ADDRESS_BASE_RESET + 0x2C) as *mut u32,
        address_quad_spi_reset_control: (ADDRESS_BASE_RESET + 0x30) as *mut u32,
        address_smc_reset_control: (ADDRESS_BASE_RESET + 0x34) as *mut u32,
        address_ocm_reset_control: (ADDRESS_BASE_RESET + 0x38) as *mut u32,
        // TODO: why offset 0x3C has no register?
        address_fpga_reset_control: (ADDRESS_BASE_RESET + 0x40) as *mut u32,
        address_cpu_reset_and_clock_control: (ADDRESS_BASE_RESET + 0x44) as *mut u32,
        address_watchdog_timer_reset_control: (ADDRESS_BASE_RESET + 0x4C) as *mut u32,
        // TODO: why these offsets have no registers?
        address_reboot_status: (ADDRESS_BASE_RESET + 0x58) as *mut u32,
        address_boot_mode: (ADDRESS_BASE_RESET + 0x5C) as *mut u32,
    },
};
