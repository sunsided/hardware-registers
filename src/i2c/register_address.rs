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

/// An 8-bit register address.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[allow(clippy::module_name_repetitions)]
pub struct RegisterAddress8(u8);

impl RegisterAddress8 {
    /// Constructs a new [`RegisterAddress8`] from a specified value.
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

impl RegisterAddress for RegisterAddress8 {
    const ADDR_BITS: usize = 8;
}

impl From<u8> for RegisterAddress8 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

/// A 16-bit register address.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[allow(clippy::module_name_repetitions)]
pub struct RegisterAddress16(u16);

impl RegisterAddress16 {
    /// Constructs a new [`RegisterAddress16`] from a specified value.
    #[must_use]
    pub const fn new(address: u16) -> Self {
        Self(address)
    }

    /// Consumes self and returns the inner value.
    #[must_use]
    pub const fn into_inner(self) -> u16 {
        self.0
    }
}

impl RegisterAddress for RegisterAddress16 {
    const ADDR_BITS: usize = 16;
}

impl From<u8> for RegisterAddress16 {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<u16> for RegisterAddress16 {
    fn from(value: u16) -> Self {
        Self::new(value)
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

impl core::fmt::Debug for RegisterAddress8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:02X}", self.0)?;
        f.write_str(" (")?;
        write!(f, "{:08b}", self.0)?;
        f.write_str(")")
    }
}

impl core::fmt::Debug for RegisterAddress16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:04X}", self.0)?;
        f.write_str(" (")?;
        write!(f, "{:016b}", self.0)?;
        f.write_str(")")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::fmt::Display for RegisterAddress8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::fmt::Display for RegisterAddress16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(addr.into_inner(), 0b0000_0011_1111_0000);
    }

    #[test]
    fn reg16_from_reg8() {
        let addr = RegisterAddress8::from(0b1111_0000_u8);
        let addr: RegisterAddress16 = addr.into();
        assert_eq!(addr.into_inner(), 0b0000_0000_1111_0000);
    }

    #[test]
    fn reg8_debug() {
        let addr = RegisterAddress8::new(0b0011_0011);
        test_format::assert_debug_fmt!(addr, "0xF0 (00110011)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn reg8_display() {
        let addr = RegisterAddress8::new(0b1111_0000);
        test_format::assert_display_fmt!(addr, "0xF0 (11116000)");
    }

    #[test]
    fn reg16_debug() {
        let addr = RegisterAddress16::new(0b1111_1111_0000_0000);
        test_format::assert_debug_fmt!(addr, "0x1F4 (0b1111111100000000)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn reg16_display() {
        let addr = RegisterAddress16::new(0b1111_1111_0000_0000);
        test_format::assert_debug_fmt!(addr, "0x1F4 (0b1111111100000000)");
    }
}
