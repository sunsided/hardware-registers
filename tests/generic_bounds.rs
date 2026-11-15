//! Downstream-crate checks: the sized marker traits alone must let generic code
//! name the concrete backing type, e.g. `<R as TryFromBits<u8>>` and `to_bits() -> u8`,
//! without extra `+ TryFromBits<u8>` bounds.

#![allow(dead_code)] // generic fns are type-checked even unused; compiling is the test

use hardware_registers::TryFromBits;

#[cfg(feature = "i2c")]
mod i2c {
    use super::*;
    use hardware_registers::i2c::*;

    pub fn read8<R: ReadableI2CRegister8<DeviceAddress7>>(
        b: u8,
    ) -> Result<R, <R as TryFromBits<u8>>::Error> {
        R::try_from_bits(b)
    }

    pub fn read16<R: ReadableI2CRegister16<DeviceAddress7>>(
        b: u16,
    ) -> Result<R, <R as TryFromBits<u16>>::Error> {
        R::try_from_bits(b)
    }

    pub fn write8<R: WritableI2CRegister8<DeviceAddress7>>(r: R) -> u8 {
        r.to_bits()
    }

    pub fn write16<R: WritableI2CRegister16<DeviceAddress7>>(r: R) -> u16 {
        r.to_bits()
    }
}

#[cfg(feature = "spi")]
mod spi {
    use super::*;
    use hardware_registers::spi::*;

    pub fn read8<R: ReadableSPIRegister8>(b: u8) -> Result<R, <R as TryFromBits<u8>>::Error> {
        R::try_from_bits(b)
    }

    pub fn read16<R: ReadableSPIRegister16>(b: u16) -> Result<R, <R as TryFromBits<u16>>::Error> {
        R::try_from_bits(b)
    }

    pub fn write8<R: WritableSPIRegister8>(r: R) -> u8 {
        r.to_bits()
    }

    pub fn write16<R: WritableSPIRegister16>(r: R) -> u16 {
        r.to_bits()
    }
}

#[cfg(feature = "i2c")]
mod layered {
    use super::*;
    use hardware_registers::i2c::*;

    pub trait Register: ReadableI2CRegister8<DeviceAddress7> {}

    pub fn read<R: Register>(b: u8) -> Result<R, <R as TryFromBits<u8>>::Error> {
        R::try_from_bits(b)
    }
}

/// The projection inside a where-clause is what failed in 0.4.0 (E0277 `R: FromBits<u8>`).
#[cfg(feature = "i2c")]
mod where_clause {
    use super::*;
    use hardware_registers::i2c::*;

    pub struct DriverError;

    pub fn read<R>(b: u8) -> Result<R, DriverError>
    where
        R: ReadableI2CRegister8<DeviceAddress7>,
        DriverError: From<<R as TryFromBits<u8>>::Error>,
    {
        Ok(R::try_from_bits(b)?)
    }
}

#[cfg(feature = "spi")]
mod where_clause_spi {
    use super::*;
    use hardware_registers::spi::*;

    pub struct DriverError;

    pub fn read8<R>(b: u8) -> Result<R, DriverError>
    where
        R: ReadableSPIRegister8,
        DriverError: From<<R as TryFromBits<u8>>::Error>,
    {
        Ok(R::try_from_bits(b)?)
    }

    pub fn read16<R>(b: u16) -> Result<R, DriverError>
    where
        R: ReadableSPIRegister16,
        DriverError: From<<R as TryFromBits<u16>>::Error>,
    {
        Ok(R::try_from_bits(b)?)
    }
}

/// The sized markers must still satisfy the generic readable traits.
#[cfg(feature = "i2c")]
mod implies_generic_i2c {
    use hardware_registers::i2c::*;
    use hardware_registers::sizes::{R1, R2};

    fn generic8<R: ReadableI2CRegister<DeviceAddress7, RegisterAddress8, R1>>() {}
    fn generic16<R: ReadableI2CRegister<DeviceAddress7, RegisterAddress16, R2>>() {}

    pub fn marker8<R: ReadableI2CRegister8<DeviceAddress7>>() {
        generic8::<R>();
    }

    pub fn marker16<R: ReadableI2CRegister16<DeviceAddress7>>() {
        generic16::<R>();
    }
}

#[cfg(feature = "spi")]
mod implies_generic_spi {
    use hardware_registers::sizes::{R1, R2};
    use hardware_registers::spi::*;

    fn generic8<R: ReadableSPIRegister<RegisterAddress8, R1>>() {}
    fn generic16<R: ReadableSPIRegister<RegisterAddress16, R2>>() {}

    pub fn marker8<R: ReadableSPIRegister8>() {
        generic8::<R>();
    }

    pub fn marker16<R: ReadableSPIRegister16>() {
        generic16::<R>();
    }
}
