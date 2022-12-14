//! System control coprocessor.

/// Auxiliary control register.
pub mod actlr {}

/// Auxiliary data fault status register.
pub mod adfsr {}

/// Auxiliary instruction fault status register.
pub mod aifsr {}

/// Auxiliary ID register.
pub mod aidr {}

/// Auxiliary memory attribute indirection register 0.
pub mod amair0 {}

/// Auxiliary memory attribute indirection register 1.
pub mod amair1 {}

// TODO

/// Coprocessor access control register.
pub mod cpacr {
    use core::{arch::asm, ops::Mul};

    use crate::common::bitman::WriteBitwise;

    /// Coprocessor.
    #[derive(Clone, Copy)]
    pub enum CoProcessor {
        /// Coprocessor 0.
        Cp0,

        /// Coprocessor 1.
        Cp1,

        /// Coprocessor 2.
        Cp2,

        /// Coprocessor 3.
        Cp3,

        /// Coprocessor 4.
        Cp4,

        /// Coprocessor 5.
        Cp5,

        /// Coprocessor 6.
        Cp6,

        /// Coprocessor 7.
        Cp7,

        /// Coprocessor 8.
        Cp8,

        /// Coprocessor 9.
        Cp9,

        /// Coprocessor 10.
        Cp10,

        /// Coprocessor 11.
        Cp11,

        /// Coprocessor 12.
        Cp12,

        /// Coprocessor 13.
        Cp13,
    }

    impl CoProcessor {
        /// Transform to unsigned 32-bit integer.
        #[inline]
        #[must_use]
        pub const fn as_u32(self) -> u32 {
            match self {
                Self::Cp0 => 0,
                Self::Cp1 => 1,
                Self::Cp2 => 2,
                Self::Cp3 => 3,
                Self::Cp4 => 4,
                Self::Cp5 => 5,
                Self::Cp6 => 6,
                Self::Cp7 => 7,
                Self::Cp8 => 8,
                Self::Cp9 => 9,
                Self::Cp10 => 10,
                Self::Cp11 => 11,
                Self::Cp12 => 12,
                Self::Cp13 => 13,
            }
        }
    }

    /// Access right for privilege levels 0 and 1.
    #[derive(Clone, Copy)]
    pub enum Access {
        /// Access to coprocessor generates undefined instruction exception.
        Denied,

        /// Access to coprocessor from privilege level 0 generates undefined instruction exception.
        Pl1,

        /// Access is granted for privilege levels 0 and 1.
        Full,
    }

    impl Access {
        /// Transform to unsigned 32-bit integer.
        #[inline]
        #[must_use]
        pub const fn as_u32(self) -> u32 {
            match self {
                Self::Denied => 0b00,
                Self::Pl1 => 0b01,
                Self::Full => 0b11,
            }
        }
    }

    /// Configure access right to coprocessor.
    #[inline]
    pub fn set_coprocessor_access(coprocessor: CoProcessor, access: Access) {
        let mut old: u32;
        // SAFETY:
        // This is valid ARMv7-A assembly.
        unsafe {
            asm!(
                // Read from coprocessor to register.
                "mrc p15, 0, {old}, c1, c0, 2",
                old = out(reg) old,
            );
        }
        let offset = coprocessor.as_u32().mul(2);
        let value = access.as_u32();
        let new = old.write_bits(offset, value, 2);
        // SAFETY:
        // This is valid ARMv7-A assembly.
        unsafe {
            asm!(
                // Write from register to coprocessor.
                "mcr p15, 0, {new}, c1, c0, 2",
                new = in(reg) new,
            );
        }
    }
}

// TODO

/// Main ID register.
pub mod midr {}

// TODO

/// System control register.
pub mod sctlr {}

// TODO
