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

impl_address!(
    /// A 7-bit device address.
    DeviceAddress7(u8),
    DeviceAddress,
    7,
    0b0111_1111
);

impl_address!(
    /// A 10-bit device address.
    DeviceAddress10(u16),
    DeviceAddress,
    10,
    0b0000_0011_1111_1111
);

impl From<u8> for DeviceAddress10 {
    fn from(value: u8) -> Self {
        Self(value.into())
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
        // i8 accepts exactly 0..=127, the valid 7-bit range, and yields the same error type.
        let addr: i8 = value.into_inner().try_into()?;
        Ok(Self(addr.unsigned_abs()))
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
        assert_eq!(addr.into_inner(), 0b0000_0000_0111_0000);
    }

    #[test]
    fn dev7_try_from_dev10() {
        assert_eq!(
            DeviceAddress7::try_from(DeviceAddress10::new(0x7F)).map(DeviceAddress7::into_inner),
            Ok(0x7F)
        );
        assert!(DeviceAddress7::try_from(DeviceAddress10::new(0x80)).is_err());
        assert!(DeviceAddress7::try_from(DeviceAddress10::new(0xFF)).is_err());
    }

    #[test]
    fn dev7_debug() {
        let addr = DeviceAddress7::new(0b111_0000);
        test_format::assert_debug_fmt!(addr, "0x70 (1110000)");
    }

    #[test]
    fn dev7_display() {
        let addr = DeviceAddress7::new(0b111_0000);
        test_format::assert_display_fmt!(addr, "0x70 (1110000)");
    }

    #[test]
    fn dev10_debug() {
        let addr = DeviceAddress10::new(0b01_1111_0100);
        test_format::assert_debug_fmt!(addr, "0x1F4 (0111110100)");
    }

    #[test]
    fn dev10_display() {
        let addr = DeviceAddress10::new(0b01_1111_0100);
        test_format::assert_display_fmt!(addr, "0x1F4 (0111110100)");
    }
}
