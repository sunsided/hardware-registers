# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
