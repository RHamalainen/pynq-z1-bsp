//! Bit manipulation.

use core::ops::{BitAnd, BitOr, Not, RangeInclusive, Shl, Shr};

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
            #[inline]
            #[must_use]
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
            #[inline]
            #[must_use]
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
            #[inline]
            #[must_use]
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

/// Can read values of multiple bits.
pub trait ReadBitwiseRange {
    /// My type.
    type Type;

    /// Read multiple bits.
    fn read_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement `ReadBitwiseRange` for given type.
macro_rules! ImplementReadBitwiseRange {
    ($type:ty) => {
        impl ReadBitwiseRange for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn read_bits(&self, range: RangeInclusive<Self>) -> Self {
                let bits = Self::BITS as Self;
                let start = *range.start();
                let end = *range.end();

                assert!(range.is_empty().not(), "Can not read empty range of bits.");
                assert!(
                    end < bits,
                    "Range end {end} must be less than type's bitwidth {bits}.",
                );

                // Clear bits lower than range start.
                let temporary_1 = self.shr(start);
                let temporary_2 = temporary_1.shl(start);

                // Clear bits higher than range end.
                let amount = bits - end - 1;
                let temporary_3 = temporary_2.shl(amount);
                let temporary_4 = temporary_3.shr(amount);

                // Move bit range to index 0.
                let temporary_5 = temporary_4.shr(start);

                temporary_5
            }
        }
    };
}

ImplementReadBitwiseRange!(u8);
ImplementReadBitwiseRange!(u32);

/// Can set multiple bits.
pub trait SetBitwiseRange {
    /// My type.
    type Type;

    /// Set multiple bits.
    fn set_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement `SetBitwiseRange` for given type.
macro_rules! ImplementSetBitwiseRange {
    ($type:ty) => {
        impl SetBitwiseRange for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn set_bits(&self, range: RangeInclusive<Self>) -> Self {
                let bits = Self::BITS as Self;
                let start = *range.start();
                let end = *range.end();

                assert!(range.is_empty().not(), "Can not set empty range of bits.");
                assert!(
                    end < bits,
                    "Range end {} must be less than type's bitwidth {}.",
                    end,
                    bits,
                );

                let mask: Self = 0;
                let mask = mask.not();

                // Clear bits lower than range start.
                let mask = mask.shr(start);
                let mask = mask.shl(start);

                // Clear bits higher than range end.
                let amount = bits - end - 1;
                let mask = mask.shl(amount);
                let mask = mask.shr(amount);

                // Set masked bits.
                let value = self.bitor(mask);
                value
            }
        }
    };
}

ImplementSetBitwiseRange!(u8);
ImplementSetBitwiseRange!(u32);

/// Can write multiple bits.
pub trait WriteBitwise {
    /// My type.
    type Type;

    /// Write multiple bits.
    fn write_bits(&self, start: Self::Type, value: Self::Type, length: Self::Type) -> Self::Type;
}

/// Implement `WriteBitwise` for given type.
macro_rules! ImplementWriteBitwise {
    ($type:ty) => {
        impl WriteBitwise for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn write_bits(&self, start: Self, value: Self, length: Self) -> Self {
                let bits = Self::BITS as Self;

                assert!(
                    start < bits,
                    "Start {} must be less than type's bitwidth {}.",
                    start,
                    bits
                );
                assert!(0 < length, "Length {} must be greater than zero.", length);
                assert!(
                    length <= bits,
                    "Length {} must be less or equal to type's bitwidth {}.",
                    length,
                    bits,
                );

                let mask1: Self = 0;
                let mask1 = mask1.set_bits(0..=length - 1);
                let mask1 = mask1.shl(start);
                let mask1 = mask1.not();

                let mask2 = value;
                let mask2 = mask2.shl(start);

                let value = self.bitand(mask1);
                let value = value.bitor(mask2);

                value
            }
        }
    };
}

ImplementWriteBitwise!(u8);
ImplementWriteBitwise!(u32);
