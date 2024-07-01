//! Support for I²C registers.

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

/// A device address.
pub trait DeviceAddress {
    /// The number of bits of the address.
    const ADDR_BITS: usize;

    /// Returns the number of bits in the address.
    #[must_use]
    fn addr_bits(&self) -> usize {
        Self::ADDR_BITS
    }
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

/// Implements 8-bit device addresses.
impl DeviceAddress for u8 {
    const ADDR_BITS: usize = 8;
}

/// Implements 8-bit register addresses.
impl RegisterAddress for u8 {
    const ADDR_BITS: usize = 8;
}

/// Implements 16-bit register addresses.
impl RegisterAddress for u16 {
    const ADDR_BITS: usize = 16;
}

/// An 8-bit device address.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceAddress8(u8);

impl DeviceAddress8 {
    /// Constructs a new [`DeviceAddress8`] from a specified value.
    #[must_use]
    pub const fn new(address: u8) -> Self {
        Self(address)
    }

    /// Consumes self and returns the inner value.
    #[must_use]
    pub const fn into_inner(self) -> u8 {
        self.0
    }
}

impl DeviceAddress for DeviceAddress8 {
    const ADDR_BITS: usize = 8;
}

impl From<u8> for DeviceAddress8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

/// A 10-bit device address.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceAddress10(u16);

impl DeviceAddress10 {
    /// Constructs a new [`DeviceAddress10`] from a specified value.
    ///
    /// To ensure the provided value is a valid 10-bit address, the value is masked
    /// with `0b1111111111`.
    #[must_use]
    pub const fn new(address: u16) -> Self {
        Self(address & 0b0000_0011_1111_1111_u16)
    }

    /// Consumes self and returns the inner value.
    #[must_use]
    pub const fn into_inner(self) -> u16 {
        self.0
    }
}

impl DeviceAddress for DeviceAddress10 {
    const ADDR_BITS: usize = 10;
}

impl From<u8> for DeviceAddress10 {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<u16> for DeviceAddress10 {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<DeviceAddress8> for DeviceAddress10 {
    fn from(value: DeviceAddress8) -> Self {
        Self(value.into_inner().into())
    }
}

impl TryFrom<DeviceAddress10> for DeviceAddress8 {
    type Error = <u16 as TryInto<u8>>::Error;

    fn try_from(value: DeviceAddress10) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        let addr_8: u8 = value.try_into()?;
        Ok(Self(addr_8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev8_from_u8() {
        let addr = DeviceAddress8::from(0b1111_0000);
        assert_eq!(addr.into_inner(), 0b1111_0000);
        assert_eq!(addr.addr_bits(), 8);
    }

    #[test]
    fn dev10_from_u8() {
        let addr = DeviceAddress10::from(0b1111_0000_u8);
        assert_eq!(addr.into_inner(), 0b1111_0000);
        assert_eq!(addr.addr_bits(), 10);
    }

    #[test]
    fn dev10_from_u16() {
        let addr = DeviceAddress10::from(0b11_1111_0000_u16);
        assert_eq!(addr.into_inner(), 0b11_1111_0000);
    }

    #[test]
    fn dev10_from_u16_masked() {
        let addr = DeviceAddress10::from(0b1111_1111_1111_0000_u16);
        assert_eq!(addr.into_inner(), 0b0000_0011_1111_0000);
    }

    #[test]
    fn dev10_from_dev8() {
        let addr = DeviceAddress8::from(0b1111_0000_u8);
        let addr: DeviceAddress10 = addr.into();
        assert_eq!(addr.into_inner(), 0b0000_0000_1111_0000);
    }
}
