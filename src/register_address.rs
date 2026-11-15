//! Traits and types for register addresses.

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

impl_address!(
    /// A 6-bit register address.
    RegisterAddress6(u8),
    RegisterAddress,
    6,
    0b0011_1111
);

impl_address!(
    /// An 8-bit register address.
    RegisterAddress8(u8),
    RegisterAddress,
    8,
    u8::MAX
);

impl_address!(
    /// A 16-bit register address.
    RegisterAddress16(u16),
    RegisterAddress,
    16,
    u16::MAX
);

impl From<u8> for RegisterAddress16 {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<RegisterAddress8> for RegisterAddress16 {
    fn from(value: RegisterAddress8) -> Self {
        Self(value.into_inner().into())
    }
}

impl TryFrom<RegisterAddress16> for RegisterAddress8 {
    type Error = <u16 as TryInto<u8>>::Error;

    fn try_from(value: RegisterAddress16) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        let addr_8: u8 = value.try_into()?;
        Ok(Self(addr_8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reg6_from_u8_masked() {
        let addr = RegisterAddress6::from(0xFF);
        assert_eq!(addr.into_inner(), 0b0011_1111);
        assert_eq!(addr.addr_bits(), 6);
    }

    #[test]
    fn reg6_debug() {
        let addr = RegisterAddress6::new(0b10_0101);
        test_format::assert_debug_fmt!(addr, "0x25 (100101)");
    }

    #[test]
    fn reg8_from_u8() {
        let addr = RegisterAddress8::from(0b1111_0000);
        assert_eq!(addr.into_inner(), 0b1111_0000);
        assert_eq!(addr.addr_bits(), 8);
    }

    #[test]
    fn reg16_from_u8() {
        let addr = RegisterAddress16::from(0b1111_0000_u8);
        assert_eq!(addr.into_inner(), 0b1111_0000);
        assert_eq!(addr.addr_bits(), 16);
    }

    #[test]
    fn reg16_from_u16() {
        let addr = RegisterAddress16::from(0b11_1111_0000_u16);
        assert_eq!(addr.into_inner(), 0b11_1111_0000);
    }

    #[test]
    fn reg16_from_u16_masked() {
        let addr = RegisterAddress16::from(0b1111_1111_1111_0000_u16);
        assert_eq!(addr.into_inner(), 0b1111_1111_1111_0000_u16);
    }

    #[test]
    fn reg16_from_reg8() {
        let addr = RegisterAddress8::from(0b1111_0000_u8);
        let addr: RegisterAddress16 = addr.into();
        assert_eq!(addr.into_inner(), 0b0000_0000_1111_0000);
    }

    #[test]
    fn reg8_try_from_reg16() {
        assert_eq!(
            RegisterAddress8::try_from(RegisterAddress16::new(0xFF))
                .map(RegisterAddress8::into_inner),
            Ok(0xFF)
        );
        assert!(RegisterAddress8::try_from(RegisterAddress16::new(0x100)).is_err());
    }

    #[test]
    fn reg8_debug() {
        let addr = RegisterAddress8::new(0b0011_0011);
        test_format::assert_debug_fmt!(addr, "0x33 (00110011)");
    }

    #[test]
    fn reg8_display() {
        let addr = RegisterAddress8::new(0b1111_0000);
        test_format::assert_display_fmt!(addr, "0xF0 (11110000)");
    }

    #[test]
    fn reg16_debug() {
        let addr = RegisterAddress16::new(0b1111_1111_0000_0000);
        test_format::assert_debug_fmt!(addr, "0xFF00 (1111111100000000)");
    }

    #[test]
    fn reg16_display() {
        let addr = RegisterAddress16::new(0b1111_1111_0000_0000);
        test_format::assert_display_fmt!(addr, "0xFF00 (1111111100000000)");
    }
}
