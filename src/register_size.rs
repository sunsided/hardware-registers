use core::fmt::{Debug, Formatter, Write};
use core::num::NonZeroUsize;

/// A register size.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RegisterSize(NonZeroUsize);

impl RegisterSize {
    /// A register with a size of 8 bits (1 byte).
    pub const R1: RegisterSize = RegisterSize(NonZeroUsize::MIN);

    /// A register with a size of 16 bits (2 bytes).
    pub const R2: RegisterSize = Self::R1.saturating_add(1);

    /// A register with a size of 24 bits (3 bytes).
    pub const R3: RegisterSize = Self::R2.saturating_add(1); // e.g., DSPs

    /// A register with a size of 32 bits (4 bytes).
    pub const R4: RegisterSize = Self::R3.saturating_add(1);

    /// A register with a size of 64 bits (8 bytes).
    pub const R8: RegisterSize = Self::R4.saturating_add(Self::R4.bytes());

    /// A register with a size of 128 bits (16 bytes).
    pub const R16: RegisterSize = Self::R8.saturating_add(Self::R8.bytes());

    /// A register with a size of 256 bits (32 bytes).
    pub const R32: RegisterSize = Self::R16.saturating_add(Self::R16.bytes());

    /// A register with a size of 512 bits (64 bytes).
    ///
    /// ## Example
    ///
    /// ```rust
    /// # use hardware_registers::RegisterSize;
    /// assert_eq!(RegisterSize::R64.bytes(), 32);
    /// assert_eq!(RegisterSize::R64.bits(), 512);
    /// ```
    pub const R64: RegisterSize = Self::R32.saturating_add(Self::R32.bytes());

    /// Initializes a new instance of the [`RegisterSize`] type.
    #[must_use]
    pub const fn new(bytes: NonZeroUsize) -> Self {
        Self(bytes)
    }

    /// Consumes self and returns a `usize` indicating the register size
    /// in bytes.
    #[must_use]
    pub const fn into_usize(self) -> usize {
        self.0.get()
    }

    /// Consumes self and returns the inner [`NonZeroUsize`] indicating
    /// the register size in bytes.
    #[must_use]
    pub const fn into_inner(self) -> NonZeroUsize {
        self.0
    }

    /// Gets the register size in bytes.
    #[must_use]
    pub const fn bytes(&self) -> usize {
        self.into_usize()
    }

    /// Gets the register size in bits.
    #[must_use]
    pub const fn bits(&self) -> usize {
        self.bytes() * 8
    }

    /// Increments the register size by the specified number of bytes.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use hardware_registers::RegisterSize;
    /// let r3 = RegisterSize::R1.saturating_add(3);
    /// assert_eq!(r3.bytes(), 4);
    /// assert_eq!(r3.bits(), 16);
    /// ```
    #[must_use]
    pub const fn saturating_add(self, bytes: usize) -> RegisterSize {
        Self::new(self.0.saturating_add(bytes))
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for RegisterSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.bytes(), f)?;
        f.write_str(" byte")?;
        if self.bytes() > 1 {
            f.write_char('s')?;
        }
        f.write_str(" (")?;
        std::fmt::Display::fmt(&self.bits(), f)?;
        f.write_str(" bits)")
    }
}

impl Debug for RegisterSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.bytes(), f)?;
        f.write_str(" byte")?;
        if self.bytes() > 1 {
            f.write_char('s')?;
        }
        f.write_str(" (")?;
        Debug::fmt(&self.bits(), f)?;
        f.write_str(" bits)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn saturating_add() {
        let r3 = RegisterSize::R1.saturating_add(3);
        assert_eq!(r3.bytes(), 4);
        assert_eq!(r3.bits(), 32);
    }

    #[test]
    pub fn debug() {
        let reg = RegisterSize::R4;
        test_format::assert_debug_fmt!(reg, "4 bytes (32 bits)");
    }

    #[test]
    #[cfg(feature = "std")]
    pub fn display() {
        let reg = RegisterSize::R4;
        test_format::assert_display_fmt!(reg, "4 bytes (32 bits)");
    }
}
