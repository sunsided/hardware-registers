# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
