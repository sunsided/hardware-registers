//! Provides traits and types to identify register sizes.

/// Accesses register size information.
#[allow(clippy::module_name_repetitions)]
pub trait RegisterSizeInformation {
    /// Gets the register size in bytes.
    const BYTES: usize;

    /// Gets the register size in bits.
    const BITS: usize;

    /// Gets the register size in bytes.
    #[inline]
    #[must_use]
    fn bytes(&self) -> usize {
        Self::BYTES
    }

    /// Gets the register size in bits.
    #[inline]
    #[must_use]
    fn bits(&self) -> usize {
        Self::BITS
    }
}

macro_rules! impl_register_size {
    ($ident: ident, $bytes: literal, $doc: literal) => {
        #[doc = $doc]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct $ident;

        impl $ident {
            /// The size of the register in bytes.
            pub const BYTES: usize = $bytes;

            /// The size of the register in bits.
            pub const BITS: usize = ($bytes) * 8;

            /// Gets the register size in bytes.
            #[must_use]
            pub const fn bytes(&self) -> usize {
                Self::BYTES
            }

            /// Gets the register size in bits.
            #[must_use]
            pub const fn bits(&self) -> usize {
                Self::BITS
            }
        }

        impl $crate::sizes::RegisterSizeInformation for $ident {
            const BYTES: usize = Self::BYTES;
            const BITS: usize = Self::BITS;
        }

        impl core::fmt::Display for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{self:?}")
            }
        }

        impl core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use core::fmt::Write;
                core::fmt::Debug::fmt(&self.bytes(), f)?;
                f.write_str(" byte")?;
                if self.bytes() > 1 {
                    f.write_char('s')?;
                }
                f.write_str(" (")?;
                core::fmt::Debug::fmt(&self.bits(), f)?;
                f.write_str(" bits)")
            }
        }
    };
}

impl_register_size!(R1, 1, "A register with a size of 8 bits (1 byte).");
impl_register_size!(R2, 2, "A register with a size of 16 bits (2 bytes).");
impl_register_size!(R3, 3, "A register with a size of 24 bits (3 bytes).");
impl_register_size!(R4, 4, "A register with a size of 32 bits (4 bytes).");
impl_register_size!(R8, 8, "A register with a size of 64 bits (8 bytes).");
impl_register_size!(R16, 16, "A register with a size of 128 bits (16 bytes).");
impl_register_size!(R32, 32, "A register with a size of 256 bits (32 bytes).");
impl_register_size!(R64, 64, "A register with a size of 512 bits (64 bytes).");

/// A type whose size matches a register of size `Size`.
///
/// Implemented for the primitive integers of exactly `Size::BYTES` bytes and for byte arrays
/// of that length. `R3`, `R32` and `R64` have no matching primitive and use `[u8; N]`.
/// Implement it for custom backing types as needed.
pub trait RegisterBacking<Size: RegisterSizeInformation> {}

macro_rules! impl_register_backing {
    ($($size:ident => $($ty:ty),+;)+) => {
        $($(impl RegisterBacking<$size> for $ty {})+)+
    };
}

impl_register_backing! {
    R1 => u8, i8, [u8; 1];
    R2 => u16, i16, [u8; 2];
    R3 => [u8; 3];
    R4 => u32, i32, [u8; 4];
    R8 => u64, i64, [u8; 8];
    R16 => u128, i128, [u8; 16];
    R32 => [u8; 32];
    R64 => [u8; 64];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn debug() {
        test_format::assert_debug_fmt!(R4, "4 bytes (32 bits)");
    }

    #[test]
    pub fn display() {
        test_format::assert_display_fmt!(R4, "4 bytes (32 bits)");
    }
}
