# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- `ReadableI2CRegister8` / `16` and `ReadableSPIRegister8` / `16` now require
  `TryFromBits<u8>` / `TryFromBits<u16>` directly. Generic code bounded only by these
  markers can now name `<R as TryFromBits<u8>>::Error` in where-clauses, which failed
  with E0277 (`R: FromBits<u8>` not satisfied). The markers still imply the generic
  `ReadableI2CRegister` / `ReadableSPIRegister` traits through their blanket impls.

## [0.4.0] - 2026-11-15

[0.4.0]: https://github.com/sunsided/hardware-registers/releases/tag/v0.4.0

### Added

- `ByteOrder` and `HardwareRegister::BYTE_ORDER` (default `BigEndian`) describe the
  wire order of multi-byte registers.
- `TryFromBits` for registers with reserved or invalid bit patterns. Every `FromBits`
  type implements it automatically with an `Infallible` error.
- `sizes::RegisterBacking<Size>` marks types whose size matches a register of `Size`.
- Write-only registers: `ReadableI2CRegister` / `ReadableSPIRegister` (plus their
  8/16-bit markers) now express readability separately.

### Changed

- [**breaking**] `I2CRegister` / `SPIRegister` (and the `*8` / `*16` markers) no longer
  imply readability or `ToBits`/`FromBits`. Use `Readable*` for reads and `Writable*`
  for writes; `Writable*` now requires `ToBits`, `Readable*` requires `TryFromBits`.
  To give an existing register a fallible decoder, replace its `FromBits` impl with a
  `TryFromBits` impl: the blanket impl makes the two mutually exclusive (E0119).
- [**breaking**] `I2CRegister::Backing` / `SPIRegister::Backing` must implement
  `RegisterBacking<Size>`, rejecting backing types whose size differs from the register
  (e.g. `u8` or `u32` on `R2`). 24-bit registers use `[u8; 3]`.

## [0.3.0] - 2026-11-15

[0.3.0]: https://github.com/sunsided/hardware-registers/releases/tag/v0.3.0

### Fixed

- `DeviceAddress7::try_from(DeviceAddress10)` now rejects addresses above `0x7F`
  instead of producing an invalid 7-bit address.
- `Display` for register sizes (`R1` … `R64`) no longer requires the `std` feature.

### Changed

- [**breaking**] `I2CRegister16` / `SPIRegister16` now require `Backing = u16`.
- [**breaking**] The writable 8/16-bit marker traits now build on their readable
  counterparts (e.g. `WritableI2CRegister8: I2CRegister8 + WritableHardwareRegister<R1>`)
  and thereby require the matching `Backing` type.
- Redundant bounds (`Clone`, `HardwareRegister<_>`, `ToBits`/`FromBits`) removed from
  the marker traits; they are implied by the remaining bounds.

## [0.2.0] - 2024-07-05

[0.2.0]: https://github.com/sunsided/hardware-registers/releases/tag/v0.2.0

### Added

- Added SPI support via the `spi` crate feature.
- `Display` implementations are now available on `no_std`.

## [0.1.0] - 2024-07-01

[0.1.0]: https://github.com/sunsided/hardware-registers/releases/tag/v0.1.0

### Added

- Initial release providing the `HardwareRegister` / `WritableHardwareRegister`, `ToBits` / `FromBits` and I²C
  register support.
