//! # Hardware Register Traits
//!
//! Generic, embedded-friendly hardware registers support, including
//! traits and types for understanding I²C registers.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(warnings, clippy::pedantic)]
#![warn(
    clippy::expect_used,
    clippy::missing_errors_doc,
    clippy::unwrap_used,
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused_qualifications
)]
// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

use crate::sizes::RegisterSizeInformation;

/// Implements an address newtype whose value is masked to `$bits` bits.
///
/// Must be defined before the modules using it (textual macro scoping).
macro_rules! impl_address {
    ($(#[$meta:meta])* $name:ident($inner:ty), $trait:ident, $bits:literal, $mask:expr) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
        #[allow(clippy::module_name_repetitions)]
        pub struct $name($inner);

        impl $name {
            #[doc = concat!("Constructs a new [`", stringify!($name), "`] from a specified value.")]
            ///
            #[doc = concat!("The value is masked to its lower ", stringify!($bits), " bits.")]
            #[must_use]
            pub const fn new(address: $inner) -> Self {
                Self(address & $mask)
            }

            /// Consumes self and returns the inner value.
            #[must_use]
            pub const fn into_inner(self) -> $inner {
                self.0
            }
        }

        impl $trait for $name {
            const ADDR_BITS: usize = $bits;
        }

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self::new(value)
            }
        }

        impl core::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "0x{v:0hex$X} ({v:0bin$b})", v = self.0, hex = ($bits + 3) / 4, bin = $bits)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{self:?}")
            }
        }
    };
}

#[cfg(feature = "i2c")]
#[cfg_attr(docsrs, doc(cfg(feature = "i2c")))]
pub mod i2c;
pub mod sizes;
#[cfg(feature = "spi")]
#[cfg_attr(docsrs, doc(cfg(feature = "spi")))]
pub mod spi;

pub mod register_address;

/// A generic hardware register of specified byte size.
pub trait HardwareRegister<Size>
where
    Size: RegisterSizeInformation,
{
    /// The size of the register in bytes.
    const SIZE_BYTES: usize = Size::BYTES;

    /// The size of the register in bits.
    const SIZE_BITS: usize = Size::BITS;

    /// The order in which the register's bytes are transferred.
    ///
    /// Irrelevant for single-byte registers. Defaults to [`ByteOrder::BigEndian`],
    /// the common order for I²C and SPI devices.
    const BYTE_ORDER: ByteOrder = ByteOrder::BigEndian;
}

/// The order in which the bytes of a multi-byte register are transferred.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ByteOrder {
    /// Most significant byte first.
    BigEndian,
    /// Least significant byte first.
    LittleEndian,
}

/// A writable hardware register of specified byte size.
pub trait WritableHardwareRegister<Size>: HardwareRegister<Size>
where
    Size: RegisterSizeInformation,
{
}

/// Conversion to bits/bytes.
pub trait ToBits {
    /// The target type.
    type Target;

    /// Converts the value into an underlying bit/byte representation
    fn to_bits(&self) -> Self::Target;
}

/// Conversion from bits/bytes.
pub trait FromBits<B>: Sized {
    /// Constructs the value from an underlying bit/byte representation
    #[inline]
    fn from_bits(value: B) -> Self {
        Self::from_bits_ref(&value)
    }

    /// Constructs the value from a reference to an underlying bit/byte representation
    fn from_bits_ref(value: &B) -> Self;
}

/// Fallible conversion from bits/bytes, e.g. for registers with reserved or invalid bit patterns.
///
/// Implemented for every [`FromBits`] type with an [`Infallible`](core::convert::Infallible) error.
/// Because of that blanket implementation, a type implements either [`FromBits`] or its own
/// [`TryFromBits`] for a given `B`, never both: to make an infallible register fallible,
/// replace its [`FromBits`] implementation.
pub trait TryFromBits<B>: Sized {
    /// The error returned if the bits do not form a valid value.
    type Error;

    /// Attempts to construct the value from an underlying bit/byte representation.
    ///
    /// # Errors
    /// Returns [`Self::Error`] if `value` is not a valid bit pattern for this type.
    #[inline]
    fn try_from_bits(value: B) -> Result<Self, Self::Error> {
        Self::try_from_bits_ref(&value)
    }

    /// Attempts to construct the value from a reference to an underlying bit/byte representation.
    ///
    /// # Errors
    /// Returns [`Self::Error`] if `value` is not a valid bit pattern for this type.
    fn try_from_bits_ref(value: &B) -> Result<Self, Self::Error>;
}

impl<T, B> TryFromBits<B> for T
where
    T: FromBits<B>,
{
    type Error = core::convert::Infallible;

    #[inline]
    fn try_from_bits_ref(value: &B) -> Result<Self, Self::Error> {
        Ok(Self::from_bits_ref(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sizes::R1;

    struct TestRegister;

    impl HardwareRegister<R1> for TestRegister {}

    #[test]
    fn constant_size_usable() {
        // Ensure that the constant can be used to do calculations.
        let _ = [0_u8; TestRegister::SIZE_BYTES * 2];
    }

    #[test]
    fn byte_order_defaults_to_big_endian() {
        assert_eq!(TestRegister::BYTE_ORDER, ByteOrder::BigEndian);
    }
}
