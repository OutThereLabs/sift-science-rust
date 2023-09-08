# Changelog

## [v0.5.0]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.4.3...v0.5.0

### Added

- Add `wallet_address` and `wallet_type` to payment methods (#19)
- Add `digital_orders` to order and transaction events (#19)
- Add `receiver_wallet_address` and `receiver_external_address` to transactions (#19)

### Changed

- Make `merchant_id` required in merchant profiles (#19)

## [v0.4.3]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.4.2...v0.4.3

### Fixed

- Fix: correct code sending to use string to avoid 0 padding issues (#17)

## [v0.4.2]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.4.1...v0.4.2

### Fixed

- Fix abuse score reason defaults (#15)

## [v0.4.1]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.4.0...v0.4.1

### Fixed

- Fix abuse score defaults (#13)

## [v0.4.0]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.3.0...v0.4.0

### Added

- Add webhook support (#7)
- Add decisions API support (#8)
- Add webhook signature verification (#9)
- Add support for `awc` v3 via feature (#12)

### Changed

- Impl `Clone` for `Client` and `Default` for `Image` (#6)

## [v0.3.0]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.2.0...v0.3.0

### Changed

- Add sift content integrity events (#5)
- Add extra field to types and events for additional custom attributes (#5)

## [v0.2.0]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.1.2...v0.2.0

### Changed

- Add changes from [sift October 2021 changelog](https://sift.com/developers/docs/curl/apis-overview/core-topics/changelog) (#4)

## [v0.1.2]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.1.1...v0.1.2

### Fixed

- Update payment method payload (#2)

## [v0.1.1]: https://github.com/OutThereLabs/sift-science-rust/compare/v0.1.0...v0.1.1

### Fixed

- Fix send verification payload (#1)

## [v0.1.0]: https://github.com/OutThereLabs/sift-science-rust/releases/tag/v0.1.0

Initial sift client implementation
