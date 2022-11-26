//! Bit manipulation.

use core::ops::{BitAnd, BitOr, Not, Shl, Shr};

/// Can set single bit high.
pub trait SetBitwise {
    /// My type.
    type Type;

    /// Set single bit high.
    fn set_bit(&self, index: Self::Type) -> Self::Type;
}

/// Implement `SetBitwise` for given type.
macro_rules! ImplementSetBitwise {
    ($type:ty) => {
        impl SetBitwise for $type {
            type Type = Self;
            fn set_bit(&self, index: Self) -> Self {
                assert!((index as u32) < Self::BITS, "Invalid index.");

                // Move high bit to target index.
                let mask = (0b1 as Self).shl(index);

                // Set bit at target index to high.
                self.bitor(mask)
            }
        }
    };
}

ImplementSetBitwise!(u8);
ImplementSetBitwise!(u32);

/// Can set single bit low.
pub trait ClearBitwise {
    /// My type.
    type Type;

    /// Set single bit low.
    fn clear_bit(&self, index: Self::Type) -> Self::Type;
}

/// Implement `ClearBitwise` for given type.
macro_rules! ImplementClearBitwise {
    ($type:ty) => {
        impl ClearBitwise for $type {
            type Type = Self;
            fn clear_bit(&self, index: Self) -> Self {
                assert!((index as u32) < Self::BITS, "Invalid index.");

                // Move low bit to target index.
                let mask = (0b1 as Self).shl(index);

                // Make other indices high and target index low.
                let mask = mask.not();

                // Set bit at target index to low.
                self.bitand(mask)
            }
        }
    };
}

ImplementClearBitwise!(u8);
ImplementClearBitwise!(u32);

/// Can read single bit value.
pub trait ReadBitwise {
    /// My type.
    type Type;

    /// Read single bit.
    fn read_bit(&self, index: Self::Type) -> bool;
}

/// Implement `ReadBitwise` for given type.
macro_rules! ImplementReadBitwise {
    ($type:ty) => {
        impl ReadBitwise for $type {
            type Type = Self;
            fn read_bit(&self, index: Self) -> bool {
                assert!((index as u32) < Self::BITS, "Invalid index.");

                // Move target bit to index 0.
                let temporary_1 = self.shr(index);

                // Clear all bits except index 0.
                let temporary_2 = 0b1.bitand(temporary_1);

                // If byte value is one, then the bit at given index is high.
                temporary_2 == 1
            }
        }
    };
}

ImplementReadBitwise!(u8);
ImplementReadBitwise!(u32);
