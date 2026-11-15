//! Support for I²C registers.

mod device_address;

pub use crate::i2c::device_address::{DeviceAddress, DeviceAddress10, DeviceAddress7};
pub use crate::register_address::{RegisterAddress, RegisterAddress16, RegisterAddress8};
use crate::sizes::{RegisterBacking, RegisterSizeInformation, R1, R2};
use crate::{HardwareRegister, ToBits, TryFromBits, WritableHardwareRegister};

/// An I²C register: its addresses, size and backing type.
///
/// Access is expressed by [`ReadableI2CRegister`] and [`WritableI2CRegister`], which are
/// implemented automatically for registers implementing [`TryFromBits`] (or [`FromBits`](crate::FromBits))
/// and [`WritableHardwareRegister`] + [`ToBits`], respectively.
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

    /// The backing type for the register, e.g. `u8` for an 8-bit register, or `u16` for a 10-bit register.
    type Backing: RegisterBacking<Size>;
}

/// A readable I²C register.
pub trait ReadableI2CRegister<D, R, Size>:
    I2CRegister<D, R, Size> + TryFromBits<<Self as I2CRegister<D, R, Size>>::Backing>
where
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// A writable I²C register.
pub trait WritableI2CRegister<D, R, Size>:
    I2CRegister<D, R, Size>
    + WritableHardwareRegister<Size>
    + ToBits<Target = <Self as I2CRegister<D, R, Size>>::Backing>
where
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Auto-implement [`ReadableI2CRegister`] for any decodable register.
impl<I, D, R, Size> ReadableI2CRegister<D, R, Size> for I
where
    I: I2CRegister<D, R, Size> + TryFromBits<<I as I2CRegister<D, R, Size>>::Backing>,
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Auto-implement [`WritableI2CRegister`] for any fitting writable register.
impl<I, D, R, Size> WritableI2CRegister<D, R, Size> for I
where
    I: I2CRegister<D, R, Size>
        + WritableHardwareRegister<Size>
        + ToBits<Target = <I as I2CRegister<D, R, Size>>::Backing>,
    D: DeviceAddress,
    R: RegisterAddress,
    Size: RegisterSizeInformation,
{
}

/// Marker trait for 8-bit I²C registers.
pub trait I2CRegister8<D>: I2CRegister<D, RegisterAddress8, R1, Backing = u8> + Copy
where
    D: DeviceAddress,
{
}

/// Marker trait for readable 8-bit I²C registers.
///
/// Requires `TryFromBits<u8>` directly rather than via the generic readable trait:
/// the solver does not normalize `TryFromBits<Self::Backing>`, so generic code could
/// not name `<R as TryFromBits<u8>>::Error`. Every such register still implements it.
///
pub trait ReadableI2CRegister8<D>: I2CRegister8<D> + TryFromBits<u8>
where
    D: DeviceAddress,
{
}

/// Marker trait for writable 8-bit I²C registers.
pub trait WritableI2CRegister8<D>:
    I2CRegister8<D> + WritableI2CRegister<D, RegisterAddress8, R1>
where
    D: DeviceAddress,
{
}

/// Marker trait for 16-bit I²C registers.
pub trait I2CRegister16<D>: I2CRegister<D, RegisterAddress16, R2, Backing = u16> + Copy
where
    D: DeviceAddress,
{
}

/// Marker trait for readable 16-bit I²C registers.
///
/// Requires `TryFromBits<u16>` directly rather than via the generic readable trait:
/// the solver does not normalize `TryFromBits<Self::Backing>`, so generic code could
/// not name `<R as TryFromBits<u16>>::Error`. Every such register still implements it.
pub trait ReadableI2CRegister16<D>: I2CRegister16<D> + TryFromBits<u16>
where
    D: DeviceAddress,
{
}

/// Marker trait for writable 16-bit I²C registers.
pub trait WritableI2CRegister16<D>:
    I2CRegister16<D> + WritableI2CRegister<D, RegisterAddress16, R2>
where
    D: DeviceAddress,
{
}

/// Auto-implement [`I2CRegister8`] for any fitting register.
impl<I, D> I2CRegister8<D> for I
where
    I: I2CRegister<D, RegisterAddress8, R1, Backing = u8> + Copy,
    D: DeviceAddress,
{
}

/// Auto-implement [`ReadableI2CRegister8`] for any fitting register.
impl<I, D> ReadableI2CRegister8<D> for I
where
    I: I2CRegister8<D> + TryFromBits<u8>,
    D: DeviceAddress,
{
}

/// Auto-implement [`WritableI2CRegister8`] for any fitting register.
impl<I, D> WritableI2CRegister8<D> for I
where
    I: I2CRegister8<D> + WritableI2CRegister<D, RegisterAddress8, R1>,
    D: DeviceAddress,
{
}

/// Auto-implement [`I2CRegister16`] for any fitting register.
impl<I, D> I2CRegister16<D> for I
where
    I: I2CRegister<D, RegisterAddress16, R2, Backing = u16> + Copy,
    D: DeviceAddress,
{
}

/// Auto-implement [`ReadableI2CRegister16`] for any fitting register.
impl<I, D> ReadableI2CRegister16<D> for I
where
    I: I2CRegister16<D> + TryFromBits<u16>,
    D: DeviceAddress,
{
}

/// Auto-implement [`WritableI2CRegister16`] for any fitting register.
impl<I, D> WritableI2CRegister16<D> for I
where
    I: I2CRegister16<D> + WritableI2CRegister<D, RegisterAddress16, R2>,
    D: DeviceAddress,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FromBits;

    #[derive(Copy, Clone)]
    struct Reg8(u8);

    impl HardwareRegister<R1> for Reg8 {}
    impl WritableHardwareRegister<R1> for Reg8 {}

    impl ToBits for Reg8 {
        type Target = u8;

        fn to_bits(&self) -> u8 {
            self.0
        }
    }

    impl FromBits<u8> for Reg8 {
        fn from_bits_ref(value: &u8) -> Self {
            Self(*value)
        }
    }

    impl I2CRegister<DeviceAddress7, RegisterAddress8, R1> for Reg8 {
        const DEFAULT_DEVICE_ADDRESS: DeviceAddress7 = DeviceAddress7::new(0x42);
        const REGISTER_ADDRESS: RegisterAddress8 = RegisterAddress8::new(0x10);
        type Backing = u8;
    }

    /// Generic driver code must see `u8` through the marker trait alone.
    fn round_trip<T>(value: u8) -> u8
    where
        T: ReadableI2CRegister8<DeviceAddress7, Error = core::convert::Infallible>
            + WritableI2CRegister8<DeviceAddress7>,
    {
        match T::try_from_bits(value) {
            Ok(reg) => reg.to_bits(),
            Err(e) => match e {},
        }
    }

    /// Write-only register: no `FromBits` needed.
    #[derive(Copy, Clone)]
    struct Command;

    impl HardwareRegister<R1> for Command {}
    impl WritableHardwareRegister<R1> for Command {}

    impl ToBits for Command {
        type Target = u8;

        fn to_bits(&self) -> u8 {
            0xA5
        }
    }

    impl I2CRegister<DeviceAddress7, RegisterAddress8, R1> for Command {
        const DEFAULT_DEVICE_ADDRESS: DeviceAddress7 = DeviceAddress7::new(0x42);
        const REGISTER_ADDRESS: RegisterAddress8 = RegisterAddress8::new(0x7E);
        type Backing = u8;
    }

    fn write<T: WritableI2CRegister8<DeviceAddress7>>(reg: T) -> u8 {
        reg.to_bits()
    }

    #[test]
    fn write_only_register() {
        assert_eq!(write(Command), 0xA5);
    }

    #[test]
    fn marker_traits_auto_implemented() {
        assert_eq!(round_trip::<Reg8>(0xAB), 0xAB);
        assert_eq!(Reg8::SIZE_BYTES, 1);
    }
}
