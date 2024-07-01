//! Support for I²C registers.

mod device_address;

pub use crate::i2c::device_address::{DeviceAddress, DeviceAddress10, DeviceAddress8};
use crate::sizes::R1;
use crate::{HardwareRegister, WritableHardwareRegister};

/// A readable I²C register.
pub trait I2CRegister<D, R>: HardwareRegister<R1>
where
    D: DeviceAddress,
    R: RegisterAddress,
{
    /// The factory-default device address.
    const DEFAULT_DEVICE_ADDRESS: D;

    /// The register address
    const REGISTER_ADDRESS: R;

    /// The backing type for the register, e.g. `u8` for an 8-bit register, or `u16` for a 10-bit register.
    type Backing;
}

/// A writable I²C register.
pub trait WritableI2CRegister<D, R>: I2CRegister<D, R> + WritableHardwareRegister<R1>
where
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Auto-implement [`WritableI2CRegister`] for any fitting writable register.
impl<I, D, R> WritableI2CRegister<D, R> for I
where
    I: I2CRegister<D, R> + WritableHardwareRegister<R1>,
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// A register address.
pub trait RegisterAddress {
    /// The number of bits of the address.
    const ADDR_BITS: usize;

    /// Returns the number of bits in the address.
    #[must_use]
    fn addr_bits(&self) -> usize {
        Self::ADDR_BITS
    }
}

/// Implements 8-bit register addresses.
impl RegisterAddress for u8 {
    const ADDR_BITS: usize = 8;
}

/// Implements 16-bit register addresses.
impl RegisterAddress for u16 {
    const ADDR_BITS: usize = 16;
}
