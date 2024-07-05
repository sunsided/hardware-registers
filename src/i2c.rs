//! Support for I²C registers.

mod device_address;

pub use crate::i2c::device_address::{DeviceAddress, DeviceAddress10, DeviceAddress7};
pub use crate::register_address::{RegisterAddress, RegisterAddress16, RegisterAddress8};
use crate::sizes::{RegisterSizeInformation, R1, R2};
use crate::{FromBits, HardwareRegister, ToBits, WritableHardwareRegister};

/// A readable I²C register.
pub trait I2CRegister<D, R, Size>:
    HardwareRegister<Size> + ToBits<Target = Self::Backing> + FromBits<Self::Backing>
where
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
    /// The factory-default device address.
    const DEFAULT_DEVICE_ADDRESS: D;

    /// The register address
    const REGISTER_ADDRESS: R;

    /// The backing type for the register, e.g. `u8` for an 8-bit register, or `u16` for a 10-bit register.
    type Backing;
}

/// A writable I²C register.
pub trait WritableI2CRegister<D, R, Size>:
    I2CRegister<D, R, Size> + WritableHardwareRegister<Size>
where
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Auto-implement [`WritableI2CRegister`] for any fitting writable register.
impl<I, D, R, Size> WritableI2CRegister<D, R, Size> for I
where
    I: I2CRegister<D, R, Size> + WritableHardwareRegister<Size>,
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Marker trait for readable 8-bit I²C registers.
pub trait I2CRegister8<D>:
    I2CRegister<D, RegisterAddress8, R1, Backing = u8>
    + HardwareRegister<R1>
    + Copy
    + Clone
    + ToBits<Target = u8>
    + FromBits<u8>
where
    D: DeviceAddress,
{
}

/// Marker trait for writable 8-bit I²C registers.
pub trait WritableI2CRegister8<D>:
    WritableI2CRegister<D, RegisterAddress8, R1> + WritableHardwareRegister<R1> + Copy + Clone
where
    D: DeviceAddress,
{
}

/// Marker trait for readable 16-bit I²C registers.
pub trait I2CRegister16<D>:
    I2CRegister<D, RegisterAddress16, R2> + HardwareRegister<R2> + Copy + Clone
where
    D: DeviceAddress,
{
}

/// Marker trait for writable 16-bit I²C registers.
pub trait WritableI2CRegister16<D>:
    WritableI2CRegister<D, RegisterAddress16, R2> + WritableHardwareRegister<R2> + Copy + Clone
where
    D: DeviceAddress,
{
}

/// Auto-implement [`I2CRegister8`] for any fitting register.
impl<I, D> I2CRegister8<D> for I
where
    I: I2CRegister<D, RegisterAddress8, R1, Backing = u8>
        + HardwareRegister<R1>
        + Copy
        + Clone
        + ToBits<Target = u8>
        + FromBits<u8>,
    D: DeviceAddress,
{
}

/// Auto-implement [`WritableI2CRegister8`] for any fitting register.
impl<I, D> WritableI2CRegister8<D> for I
where
    I: WritableI2CRegister<D, RegisterAddress8, R1> + WritableHardwareRegister<R1> + Copy + Clone,
    D: DeviceAddress,
{
}

/// Auto-implement [`I2CRegister16`] for any fitting register.
impl<I, D> I2CRegister16<D> for I
where
    I: I2CRegister<D, RegisterAddress16, R2> + HardwareRegister<R2> + Copy + Clone,
    D: DeviceAddress,
{
}

/// Auto-implement [`WritableI2CRegister16`] for any fitting register.
impl<I, D> WritableI2CRegister16<D> for I
where
    I: WritableI2CRegister<D, RegisterAddress16, R2> + WritableHardwareRegister<R2> + Copy + Clone,
    D: DeviceAddress,
{
}
