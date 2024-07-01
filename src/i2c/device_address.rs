use core::ops::Deref;

/// A device address.
pub trait DeviceAddress {
    /// The number of bits of the address.
    const ADDR_BITS: usize;

    /// Returns the number of bits in the address.
    #[must_use]
    fn addr_bits(&self) -> usize {
        Self::ADDR_BITS
    }
}

/// An 7-bit device address.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[allow(clippy::module_name_repetitions)]
pub struct DeviceAddress7(u8);

impl DeviceAddress7 {
    /// Constructs a new [`DeviceAddress7`] from a specified value.
    ///
    /// To ensure the provided value is a valid 7-bit address, the value is masked
    /// with `0b0111_1111`.
    #[must_use]
    pub const fn new(address: u8) -> Self {
        Self(address & 0b0111_1111_u8)
    }

    /// Consumes self and returns the inner value.
    #[must_use]
    pub const fn into_inner(self) -> u8 {
        self.0
    }
}

impl DeviceAddress for DeviceAddress7 {
    const ADDR_BITS: usize = 7;
}

impl From<u8> for DeviceAddress7 {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

/// A 10-bit device address.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[allow(clippy::module_name_repetitions)]
pub struct DeviceAddress10(u16);

impl DeviceAddress10 {
    /// Constructs a new [`DeviceAddress10`] from a specified value.
    ///
    /// To ensure the provided value is a valid 10-bit address, the value is masked
    /// with `0b1111111111`.
    #[must_use]
    pub const fn new(address: u16) -> Self {
        Self(address & 0b0000_0011_1111_1111_u16)
    }

    /// Consumes self and returns the inner value.
    #[must_use]
    pub const fn into_inner(self) -> u16 {
        self.0
    }
}

impl DeviceAddress for DeviceAddress10 {
    const ADDR_BITS: usize = 10;
}

impl From<u8> for DeviceAddress10 {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<u16> for DeviceAddress10 {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<DeviceAddress7> for DeviceAddress10 {
    fn from(value: DeviceAddress7) -> Self {
        Self(value.into_inner().into())
    }
}

impl TryFrom<DeviceAddress10> for DeviceAddress7 {
    type Error = <u16 as TryInto<u8>>::Error;

    fn try_from(value: DeviceAddress10) -> Result<Self, Self::Error> {
        let value = value.into_inner();
        let addr_8: u8 = value.try_into()?;
        Ok(Self(addr_8))
    }
}

impl Deref for DeviceAddress7 {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DeviceAddress10 {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Debug for DeviceAddress7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:02X}", self.0)?;
        f.write_str(" (")?;
        write!(f, "{:07b}", self.0)?;
        f.write_str(")")
    }
}

impl core::fmt::Debug for DeviceAddress10 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:03X}", self.0)?;
        f.write_str(" (")?;
        write!(f, "{:010b}", self.0)?;
        f.write_str(")")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::fmt::Display for DeviceAddress7 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::fmt::Display for DeviceAddress10 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dev7_from_u8() {
        let addr = DeviceAddress7::from(0b111_0000);
        assert_eq!(addr.into_inner(), 0b111_0000);
        assert_eq!(addr.addr_bits(), 7);
    }

    #[test]
    fn dev10_from_u8() {
        let addr = DeviceAddress10::from(0b0111_0000_u8);
        assert_eq!(addr.into_inner(), 0b111_0000);
        assert_eq!(addr.addr_bits(), 10);
    }

    #[test]
    fn dev7_from_u8_masked() {
        let addr = DeviceAddress7::from(0b1111_0000_u8);
        assert_eq!(addr.into_inner(), 0b111_0000);
    }

    #[test]
    fn dev10_from_u16() {
        let addr = DeviceAddress10::from(0b11_1111_0000_u16);
        assert_eq!(addr.into_inner(), 0b11_1111_0000);
    }

    #[test]
    fn dev10_from_u16_masked() {
        let addr = DeviceAddress10::from(0b1111_1111_1111_0000_u16);
        assert_eq!(addr.into_inner(), 0b0000_0011_1111_0000);
    }

    #[test]
    fn dev10_from_dev7() {
        let addr = DeviceAddress7::from(0b1111_0000_u8);
        let addr: DeviceAddress10 = addr.into();
        assert_eq!(addr.into_inner(), 0b0000_0000_1111_0000);
    }

    #[test]
    fn dev7_debug() {
        let addr = DeviceAddress7::new(0b111_0000);
        test_format::assert_debug_fmt!(addr, "0xF0 (01110000)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn dev7_display() {
        let addr = DeviceAddress7::new(0b111_0000);
        test_format::assert_display_fmt!(addr, "0xF0 (01110000)");
    }

    #[test]
    fn dev10_debug() {
        let addr = DeviceAddress10::new(0b01_1111_0100);
        test_format::assert_debug_fmt!(addr, "0x1F4 (0111110100)");
    }

    #[test]
    #[cfg(feature = "std")]
    fn dev10_display() {
        let addr = DeviceAddress10::new(0b01_1111_0100);
        test_format::assert_display_fmt!(addr, "0x1F4 (0111110100)");
    }
}
