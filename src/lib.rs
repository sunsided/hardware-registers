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

mod register_size;

pub use register_size::{RegisterSize, RegisterSizeInformation};

/// A generic hardware register of specified byte size.
pub trait HardwareRegister<Size>
where
    Size: RegisterSizeInformation,
{
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

    struct TestRegister;

    impl HardwareRegister<RegisterSize::R1> for TestRegister {}

    #[test]
    fn constant_size_usable() {
        // Ensure that the constant can be used to do calculations.
        let _ = [0_u8; TestRegister::SIZE * 2];
    }
}
