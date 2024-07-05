//! Support for SPI registers.

pub use crate::register_address::{RegisterAddress, RegisterAddress16, RegisterAddress8};
use crate::sizes::{RegisterSizeInformation, R1, R2};
use crate::{FromBits, HardwareRegister, ToBits, WritableHardwareRegister};

/// A readable SPI register.
pub trait SPIRegister<R, Size>:
    HardwareRegister<Size> + ToBits<Target = Self::Backing> + FromBits<Self::Backing>
where
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
    /// The register address
    const REGISTER_ADDRESS: R;

    /// The backing type for the register, e.g. `u8` for an 8-bit register, or `u16` for a 10-bit register.
    type Backing;
}

/// A writable SPI register.
pub trait WritableSPIRegister<R, Size>:
    SPIRegister<R, Size> + WritableHardwareRegister<Size>
where
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Auto-implement [`WritableSPIRegister`] for any fitting writable register.
impl<I, R, Size> WritableSPIRegister<R, Size> for I
where
    I: SPIRegister<R, Size> + WritableHardwareRegister<Size>,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Marker trait for readable 8-bit SPI registers.
pub trait SPIRegister8:
    SPIRegister<RegisterAddress8, R1, Backing = u8>
    + HardwareRegister<R1>
    + Copy
    + Clone
    + ToBits<Target = u8>
    + FromBits<u8>
{
}

/// Marker trait for writable 8-bit SPI registers.
pub trait WritableSPIRegister8:
    WritableSPIRegister<RegisterAddress8, R1> + WritableHardwareRegister<R1> + Copy + Clone
{
}

/// Marker trait for readable 16-bit SPI registers.
pub trait SPIRegister16:
    SPIRegister<RegisterAddress16, R2> + HardwareRegister<R2> + Copy + Clone
{
}

/// Marker trait for writable 16-bit SPI registers.
pub trait WritableSPIRegister16:
    WritableSPIRegister<RegisterAddress16, R2> + WritableHardwareRegister<R2> + Copy + Clone
{
}

/// Auto-implement [`SPIRegister8`] for any fitting register.
impl<I> SPIRegister8 for I where
    I: SPIRegister<RegisterAddress8, R1, Backing = u8>
        + HardwareRegister<R1>
        + Copy
        + Clone
        + ToBits<Target = u8>
        + FromBits<u8>
{
}

/// Auto-implement [`WritableSPIRegister8`] for any fitting register.
impl<I> WritableSPIRegister8 for I where
    I: WritableSPIRegister<RegisterAddress8, R1> + WritableHardwareRegister<R1> + Copy + Clone
{
}

/// Auto-implement [`SPIRegister16`] for any fitting register.
impl<I> SPIRegister16 for I where
    I: SPIRegister<RegisterAddress16, R2> + HardwareRegister<R2> + Copy + Clone
{
}

/// Auto-implement [`WritableSPIRegister16`] for any fitting register.
impl<I> WritableSPIRegister16 for I where
    I: WritableSPIRegister<RegisterAddress16, R2> + WritableHardwareRegister<R2> + Copy + Clone
{
}
