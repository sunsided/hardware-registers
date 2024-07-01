//! Generic, embedded-friendly hardware registers support, including
//! traits and types for understanding I2C registers.

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
#[cfg_attr(docsrs, doc(cfg(feature = "isc")))]
pub mod i2c;
pub mod sizes;

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
