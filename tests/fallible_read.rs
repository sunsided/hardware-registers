//! Downstream-crate checks: a manual `TryFromBits` impl must coexist with the
//! blanket impl for `FromBits` types, and work through the readable marker traits.

#![cfg(feature = "spi")]

use hardware_registers::sizes::R1;
use hardware_registers::spi::{ReadableSPIRegister8, RegisterAddress8, SPIRegister};
use hardware_registers::{HardwareRegister, TryFromBits};

/// Register whose upper six bits are reserved and must read as zero.
#[derive(Copy, Clone, Debug, PartialEq)]
enum Mode {
    Off,
    On,
    Auto,
}

impl HardwareRegister<R1> for Mode {}

impl TryFromBits<u8> for Mode {
    type Error = u8;

    fn try_from_bits(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Off),
            1 => Ok(Self::On),
            2 => Ok(Self::Auto),
            invalid => Err(invalid),
        }
    }
}

impl SPIRegister<RegisterAddress8, R1> for Mode {
    const REGISTER_ADDRESS: RegisterAddress8 = RegisterAddress8::new(0x0F);
    type Backing = u8;
}

fn read<T: ReadableSPIRegister8>(raw: u8) -> Result<T, T::Error> {
    T::try_from_bits(raw)
}

#[test]
fn fallible_read_through_marker_trait() {
    assert_eq!(read::<Mode>(2), Ok(Mode::Auto));
    assert_eq!(read::<Mode>(3), Err(3));
}
