//! Support for I²C registers.

mod device_address;
mod register_address;

pub use crate::i2c::device_address::{DeviceAddress, DeviceAddress10, DeviceAddress7};
pub use crate::i2c::register_address::{RegisterAddress, RegisterAddress16, RegisterAddress8};
use crate::sizes::{RegisterSizeInformation, R1, R2};
use crate::{HardwareRegister, WritableHardwareRegister};

/// A readable I²C register.
pub trait I2CRegister<D, R, Size>: HardwareRegister<Size>
where
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
    /// The factory-default device address.
    const DEFAULT_DEVICE_ADDRESS: D;

    /// The register address
    const REGISTER_ADDRESS: R;

    /*
    /// The backing type for the register, e.g. `u8` for an 8-bit register, or `u16` for a 10-bit register.
    type Backing;
    */
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
pub trait I2CRegister8<D, R>: I2CRegister<D, R, R1> + HardwareRegister<R1>
where
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Marker trait for writable 8-bit I²C registers.
pub trait WritableI2CRegister8<D, R>:
    WritableI2CRegister<D, R, R1> + WritableHardwareRegister<R1>
where
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Marker trait for readable 16-bit I²C registers.
pub trait I2CRegister16<D, R>: I2CRegister<D, R, R2> + HardwareRegister<R2>
where
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Marker trait for writable 16-bit I²C registers.
pub trait WritableI2CRegister16<D, R>:
    WritableI2CRegister<D, R, R2> + WritableHardwareRegister<R2>
where
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Auto-implement [`I2CRegister8`] for any fitting register.
impl<I, D, R> I2CRegister8<D, R> for I
where
    I: I2CRegister<D, R, R1> + WritableHardwareRegister<R1>,
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Auto-implement [`WritableI2CRegister8`] for any fitting register.
impl<I, D, R> WritableI2CRegister8<D, R> for I
where
    I: WritableI2CRegister<D, R, R1> + WritableHardwareRegister<R1>,
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Auto-implement [`I2CRegister16`] for any fitting register.
impl<I, D, R> I2CRegister16<D, R> for I
where
    I: I2CRegister<D, R, R2> + WritableHardwareRegister<R2>,
    D: DeviceAddress,
    R: RegisterAddress,
{
}

/// Auto-implement [`WritableI2CRegister16`] for any fitting register.
impl<I, D, R> WritableI2CRegister16<D, R> for I
where
    I: WritableI2CRegister<D, R, R2> + WritableHardwareRegister<R2>,
    D: DeviceAddress,
    R: RegisterAddress,
{
}
