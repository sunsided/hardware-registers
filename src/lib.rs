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
    /// Converts the value into an underlying bit/byte representation
    #[inline]
    fn from_bits(value: B) -> Self {
        Self::from_bits_ref(&value)
    }

    /// Converts the value into an underlying bit/byte representation
    fn from_bits_ref(value: &B) -> Self;
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
}
