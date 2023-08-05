use crate::common::memman::clear_address_bit;
use crate::common::memman::read_address_bit;
use crate::common::memman::set_address_bit;
use crate::common::memman::write_to_address;

// TODO: add error strings

#[derive(Clone, Copy)]
pub enum PinDirection {
    /// Pin is configured as output.
    Output,

    /// Pin is configured as input.
    Input,
}

impl PinDirection {
    pub fn from_bool(value: bool) -> Self {
        if value {
            Self::Input
        } else {
            Self::Output
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            Self::Output => false,
            Self::Input => true,
        }
    }

    pub fn as_str<'a>(self) -> &'a str {
        match self {
            Self::Output => "output",
            Self::Input => "input",
        }
    }
}

impl core::fmt::Display for PinDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Channels {
    /// Device has a single channel.
    Single,

    /// Device has two channels.
    Dual,
}

impl Channels {
    pub fn as_str<'a>(self) -> &'a str {
        match self {
            Self::Single => "single",
            Self::Dual => "dual",
        }
    }
}

impl core::fmt::Display for Channels {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct Channel {
    address_data: *mut u32,
    address_control: *mut u32,
    width: u32,
}

impl Channel {
    pub fn new(address_data: *mut u32, address_control: *mut u32, width: u32) -> Self {
        Self {
            address_data,
            address_control,
            width,
        }
    }

    pub fn address_data(&self) -> *mut u32 {
        self.address_data
    }

    pub fn address_control(&self) -> *mut u32 {
        self.address_control
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn pin_direction(&self, index: u32) -> Result<PinDirection, ()> {
        if (0..self.width).contains(&index) {
            let value = read_address_bit(self.address_control, index);
            let direction = PinDirection::from_bool(value);
            Ok(direction)
        } else {
            Err(())
        }
    }

    pub fn read_pin(&self, index: u32) -> Result<bool, ()> {
        if (0..self.width).contains(&index) {
            match self.pin_direction(index).unwrap() {
                PinDirection::Output => Err(()),
                PinDirection::Input => Ok(read_address_bit(self.address_data, index)),
            }
        } else {
            Err(())
        }
    }

    pub fn write_pin(&self, index: u32, value: bool) -> Result<(), &'static str> {
        if (0..self.width).contains(&index) {
            match self.pin_direction(index).unwrap() {
                PinDirection::Output => {
                    let action = if value {
                        set_address_bit
                    } else {
                        clear_address_bit
                    };
                    action(self.address_data, index);
                    Ok(())
                }
                PinDirection::Input => Err("can not write to input pin"),
            }
        } else {
            Err("given pin does not exist")
        }
    }

    pub fn set_pin_direction(&self, index: u32, direction: PinDirection) -> Result<(), ()> {
        if (0..self.width).contains(&index) {
            let action = if direction.as_bool() {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_control, index);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn reset(&self) {
        write_to_address(self.address_control, 0);
        write_to_address(self.address_control, 0);
    }
}

impl core::fmt::Display for Channel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value_data = unsafe { core::ptr::read_volatile(self.address_data) };
        let value_control = unsafe { core::ptr::read_volatile(self.address_control) };
        write!(
            f,
            "data @ 0x{:0>8X}=0b{:0>32b}, control @ 0x{:0>8X}=0b{:0>32b}, width={}",
            self.address_data as u32,
            value_data,
            self.address_control as u32,
            value_control,
            self.width
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChannelIndex {
    Channel1,
    Channel2,
}

impl ChannelIndex {
    fn as_u32(self) -> u32 {
        match self {
            Self::Channel1 => 0,
            Self::Channel2 => 1,
        }
    }

    fn as_str<'a>(self) -> &'a str {
        match self {
            Self::Channel1 => "channel 1",
            Self::Channel2 => "channel 2",
        }
    }
}

impl core::fmt::Display for ChannelIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct InterruptMechanism {
    address_global_interrupt_enable: *mut u32,
    address_ip_interrupt_enable: *mut u32,
    address_ip_interrupt_status: *mut u32,
    channels: Channels,
}

impl InterruptMechanism {
    pub fn new(
        address_global_interrupt_enable: *mut u32,
        address_ip_interrupt_enable: *mut u32,
        address_ip_interrupt_status: *mut u32,
        channels: Channels,
    ) -> Self {
        Self {
            address_global_interrupt_enable,
            address_ip_interrupt_enable,
            address_ip_interrupt_status,
            channels,
        }
    }

    /// True if interrupts are enabled.
    pub fn is_interrupts_enabled(&self) -> bool {
        read_address_bit(self.address_global_interrupt_enable, 31)
    }

    /// Enable or disable interrupts.
    pub fn toggle_interrupts(&self, enable: bool) {
        let action = if enable {
            set_address_bit
        } else {
            clear_address_bit
        };
        action(self.address_global_interrupt_enable, 31);
    }

    /// Check that the device is configured with dual channels if channel 2 is used.
    fn check_channel_configuration(&self, channel: ChannelIndex) -> Result<(), ()> {
        if channel == ChannelIndex::Channel2 {
            if self.channels != Channels::Dual {
                return Err(());
            }
        }
        Ok(())
    }

    /// True if channel interrupts are enabled.
    pub fn is_channel_interrupts_enabled(&self, channel: ChannelIndex) -> Result<bool, ()> {
        if let Err(_) = self.check_channel_configuration(channel) {
            Err(())
        } else {
            let index = channel.as_u32();
            Ok(read_address_bit(self.address_ip_interrupt_enable, index))
        }
    }

    /// Enable or disable channel interrupts.
    pub fn toggle_channel_interrupts(&self, channel: ChannelIndex, enable: bool) -> Result<(), ()> {
        if let Err(_) = self.check_channel_configuration(channel) {
            Err(())
        } else {
            let index = channel.as_u32();
            let action = if enable {
                set_address_bit
            } else {
                clear_address_bit
            };
            action(self.address_ip_interrupt_enable, index);
            Ok(())
        }
    }

    /// True if channel interrupt is signaled.
    pub fn read_channel_interrupt_status(&self, channel: ChannelIndex) -> Result<bool, ()> {
        if let Err(_) = self.check_channel_configuration(channel) {
            Err(())
        } else {
            let index = channel.as_u32();
            let status = read_address_bit(self.address_ip_interrupt_status, index);
            Ok(status)
        }
    }

    pub fn clear_channel_interrupt(&self, channel: ChannelIndex) -> Result<(), ()> {
        if let Err(_) = self.check_channel_configuration(channel) {
            Err(())
        } else {
            let index = channel.as_u32();
            set_address_bit(self.address_ip_interrupt_status, index);
            Ok(())
        }
    }

    pub fn reset(&self) {
        write_to_address(self.address_global_interrupt_enable, 0);
        write_to_address(self.address_ip_interrupt_enable, 0);
        write_to_address(self.address_ip_interrupt_status, 0xFFFF_FFFF);
    }
}

impl core::fmt::Display for InterruptMechanism {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value_gie = unsafe { core::ptr::read_volatile(self.address_global_interrupt_enable) };
        let value_iie = unsafe { core::ptr::read_volatile(self.address_ip_interrupt_enable) };
        let value_iis = unsafe { core::ptr::read_volatile(self.address_ip_interrupt_status) };

        write!(
            f,
            "gie @ 0x{:X}=0b{:b}, iie @ 0x{:X}=0b{:b}, iis @ 0x{:X}=0b{:b}, channels={}",
            self.address_global_interrupt_enable as u32,
            value_gie,
            self.address_ip_interrupt_enable as u32,
            value_iie,
            self.address_ip_interrupt_status as u32,
            value_iis,
            self.channels,
        )
    }
}

fn solve_address(address: *mut u32, offset: u32) -> *mut u32 {
    (address as u32 + offset) as *mut u32
}

pub struct AxiGpio {
    address_base: *mut u32,
    channel_1: Channel,
    channel_2: Option<Channel>,
    interrupt_mechanism: Option<InterruptMechanism>,
}

impl AxiGpio {
    pub fn new(address_base: *mut u32, channels: Channels, interrupts: bool, width: u32) -> Self {
        let address_data_1 = solve_address(address_base, 0x000);
        let address_control_1 = solve_address(address_base, 0x004);
        let channel_1 = Channel::new(address_data_1, address_control_1, width);
        let channel_2 = match channels {
            Channels::Single => None,
            Channels::Dual => {
                let address_data_2 = solve_address(address_base, 0x008);
                let address_control_2 = solve_address(address_base, 0x00C);
                Some(Channel::new(address_data_2, address_control_2, width))
            }
        };
        let interrupt_mechanism = if interrupts {
            let address_global_interrupt_enable = solve_address(address_base, 0x11C);
            let address_ip_interrupt_enable = solve_address(address_base, 0x128);
            let address_ip_interrupt_status = solve_address(address_base, 0x120);
            Some(InterruptMechanism::new(
                address_global_interrupt_enable,
                address_ip_interrupt_enable,
                address_ip_interrupt_status,
                channels,
            ))
        } else {
            None
        };
        Self {
            address_base,
            channel_1,
            channel_2,
            interrupt_mechanism,
        }
    }

    pub fn address_base(&self) -> *mut u32 {
        self.address_base
    }

    pub fn channel_1(&self) -> &Channel {
        &self.channel_1
    }

    pub fn channel_2(&self) -> &Option<Channel> {
        &self.channel_2
    }

    pub fn interrupt_mechanism(&self) -> &Option<InterruptMechanism> {
        &self.interrupt_mechanism
    }

    pub fn reset(&self) {
        self.channel_1.reset();
        if let Some(channel2) = &self.channel_2 {
            channel2.reset();
        }
        if let Some(interrupt_mechanism) = &self.interrupt_mechanism {
            interrupt_mechanism.reset();
        }
    }
}

impl core::fmt::Display for AxiGpio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.channel_2 {
            Some(channel_2) => match &self.interrupt_mechanism {
                Some(im) => {
                    write!(
                        f,
                        "dual channel axi gpio with interrupts @ 0x{:X}, channel 1: {}, channel 2: {}, interrupts: {}",
                        self.address_base as u32,
                        self.channel_1,
                        channel_2,
                        im,
                    )
                }
                None => {
                    write!(
                        f,
                        "dual channel axi gpio without interrupts @ 0x{:X}, channel 1: {}, channel 2: {}",
                        self.address_base as u32,
                        self.channel_1,
                        channel_2,
                    )
                }
            },
            None => match &self.interrupt_mechanism {
                Some(im) => {
                    write!(
                        f,
                        "single channel axi gpio with interrupts @ 0x{:X}, channel 1: {}, interrupts: {}",
                        self.address_base as u32,
                        self.channel_1,
                        im,
                    )
                }
                None => {
                    write!(
                        f,
                        "single channel axi gpio without interrupts @ 0x{:X}, channel 1: {}",
                        self.address_base as u32, self.channel_1,
                    )
                }
            },
        }
    }
}
