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
pub trait I2CRegister8<D>: I2CRegister<D, RegisterAddress8, R1, Backing = u8> + Copy
where
    D: DeviceAddress,
{
}

/// Marker trait for writable 8-bit I²C registers.
pub trait WritableI2CRegister8<D>: I2CRegister8<D> + WritableHardwareRegister<R1>
where
    D: DeviceAddress,
{
}

/// Marker trait for readable 16-bit I²C registers.
pub trait I2CRegister16<D>: I2CRegister<D, RegisterAddress16, R2, Backing = u16> + Copy
where
    D: DeviceAddress,
{
}

/// Marker trait for writable 16-bit I²C registers.
pub trait WritableI2CRegister16<D>: I2CRegister16<D> + WritableHardwareRegister<R2>
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

/// Auto-implement [`WritableI2CRegister8`] for any fitting register.
impl<I, D> WritableI2CRegister8<D> for I
where
    I: I2CRegister8<D> + WritableHardwareRegister<R1>,
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

/// Auto-implement [`WritableI2CRegister16`] for any fitting register.
impl<I, D> WritableI2CRegister16<D> for I
where
    I: I2CRegister16<D> + WritableHardwareRegister<R2>,
    D: DeviceAddress,
{
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn round_trip<T: WritableI2CRegister8<DeviceAddress7>>(value: u8) -> u8 {
        T::from_bits(value).to_bits()
    }

    #[test]
    fn marker_traits_auto_implemented() {
        assert_eq!(round_trip::<Reg8>(0xAB), 0xAB);
        assert_eq!(Reg8::SIZE_BYTES, 1);
    }
}
