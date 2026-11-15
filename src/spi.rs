//! Support for SPI registers.

pub use crate::register_address::{RegisterAddress, RegisterAddress16, RegisterAddress8};
use crate::sizes::{RegisterBacking, RegisterSizeInformation, R1, R2};
use crate::{HardwareRegister, ToBits, TryFromBits, WritableHardwareRegister};

/// An SPI register: its address, size and backing type.
///
/// Access is expressed by [`ReadableSPIRegister`] and [`WritableSPIRegister`], which are
/// implemented automatically for registers implementing [`TryFromBits`] (or [`FromBits`](crate::FromBits))
/// and [`WritableHardwareRegister`] + [`ToBits`], respectively.
pub trait SPIRegister<R, Size>: HardwareRegister<Size>
where
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
    /// The register address
    const REGISTER_ADDRESS: R;

    /// The backing type for the register, e.g. `u8` for an 8-bit register, or `u16` for a 10-bit register.
    type Backing: RegisterBacking<Size>;
}

/// A readable SPI register.
pub trait ReadableSPIRegister<R, Size>:
    SPIRegister<R, Size> + TryFromBits<<Self as SPIRegister<R, Size>>::Backing>
where
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// A writable SPI register.
pub trait WritableSPIRegister<R, Size>:
    SPIRegister<R, Size>
    + WritableHardwareRegister<Size>
    + ToBits<Target = <Self as SPIRegister<R, Size>>::Backing>
where
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Auto-implement [`ReadableSPIRegister`] for any decodable register.
impl<I, R, Size> ReadableSPIRegister<R, Size> for I
where
    I: SPIRegister<R, Size> + TryFromBits<<I as SPIRegister<R, Size>>::Backing>,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Auto-implement [`WritableSPIRegister`] for any fitting writable register.
impl<I, R, Size> WritableSPIRegister<R, Size> for I
where
    I: SPIRegister<R, Size>
        + WritableHardwareRegister<Size>
        + ToBits<Target = <I as SPIRegister<R, Size>>::Backing>,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Marker trait for 8-bit SPI registers.
pub trait SPIRegister8: SPIRegister<RegisterAddress8, R1, Backing = u8> + Copy {}

/// Marker trait for readable 8-bit SPI registers.
///
/// Requires `TryFromBits<u8>` directly rather than via the generic readable trait:
/// the solver does not normalize `TryFromBits<Self::Backing>`, so generic code could
/// not name `<R as TryFromBits<u8>>::Error`. Every such register still implements it.
///
pub trait ReadableSPIRegister8: SPIRegister8 + TryFromBits<u8> {}

/// Marker trait for writable 8-bit SPI registers.
pub trait WritableSPIRegister8: SPIRegister8 + WritableSPIRegister<RegisterAddress8, R1> {}

/// Marker trait for 16-bit SPI registers.
pub trait SPIRegister16: SPIRegister<RegisterAddress16, R2, Backing = u16> + Copy {}

/// Marker trait for readable 16-bit SPI registers.
///
/// Requires `TryFromBits<u16>` directly rather than via the generic readable trait:
/// the solver does not normalize `TryFromBits<Self::Backing>`, so generic code could
/// not name `<R as TryFromBits<u16>>::Error`. Every such register still implements it.
pub trait ReadableSPIRegister16: SPIRegister16 + TryFromBits<u16> {}

/// Marker trait for writable 16-bit SPI registers.
pub trait WritableSPIRegister16:
    SPIRegister16 + WritableSPIRegister<RegisterAddress16, R2>
{
}

/// Auto-implement [`SPIRegister8`] for any fitting register.
impl<I> SPIRegister8 for I where I: SPIRegister<RegisterAddress8, R1, Backing = u8> + Copy {}

/// Auto-implement [`ReadableSPIRegister8`] for any fitting register.
impl<I> ReadableSPIRegister8 for I where I: SPIRegister8 + TryFromBits<u8> {}

/// Auto-implement [`WritableSPIRegister8`] for any fitting register.
impl<I> WritableSPIRegister8 for I where I: SPIRegister8 + WritableSPIRegister<RegisterAddress8, R1> {}

/// Auto-implement [`SPIRegister16`] for any fitting register.
impl<I> SPIRegister16 for I where I: SPIRegister<RegisterAddress16, R2, Backing = u16> + Copy {}

/// Auto-implement [`ReadableSPIRegister16`] for any fitting register.
impl<I> ReadableSPIRegister16 for I where I: SPIRegister16 + TryFromBits<u16> {}

/// Auto-implement [`WritableSPIRegister16`] for any fitting register.
impl<I> WritableSPIRegister16 for I where
    I: SPIRegister16 + WritableSPIRegister<RegisterAddress16, R2>
{
}
