# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] <!-- release-date -->

## [0.3.4] - 2024-10-22

### Changed

- Remove unused dependency `secrecy`.

## [0.3.3] - 2024-04-05

### Added

- Add missing crate metadata `categories` and `keywords`.

## [0.3.2] - 2024-04-05

### Changed

- Update dependencies.

## [0.3.1] - 2023-10-24

### Fixed

- Fix changelog after duplicate release due to missing crates.io API token.

## [0.3.0] - 2023-10-24

### Fixed

- Change wording "strongly" to "statically" typed.

### BREAKING CHANGES

- Update re-exported dependency `mail-send` to v0.4,
  updating transitively re-exported dependency
  [`mail-builder` to v0.3](https://github.com/stalwartlabs/mail-builder/compare/0.2.5...v0.3.1).

## [0.2.2] - 2023-05-31

### Added

- Vastly improve documentation.

## [0.2.1] - 2023-05-30

## [0.2.0] - 2023-05-30

### BREAKING CHANGES

- Rename type-erased, object safe `trait Mailer` to `trait DynMailer`.

### Added

- Add statically typed `Mailer` trait to be used as generic trait bound.
- Add `BoxMailer` and `ArcMailer` type aliases for `dyn DynMailer` wrapped in smart pointers.
- Add a `CHANGELOG.md` and configure `cargo-release`.

## [0.1.2] - 2023-05-16

### Added

- Add crate link to documentation.

## [0.1.0] - 2023-05-16

### Added

- Initial implementation.

<!-- next-url -->
[Unreleased]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.3.4...HEAD
[0.3.4]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.3.3...async-mailer-core-v0.3.4
[0.3.3]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.3.2...async-mailer-core-v0.3.3
[0.3.2]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.3.1...async-mailer-core-v0.3.2
[0.3.1]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.3.0...async-mailer-core-v0.3.1
[0.3.0]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.2.2...async-mailer-core-v0.3.0
[0.2.2]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.2.1...async-mailer-core-v0.2.2
[0.2.1]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.2.0...async-mailer-core-v0.2.1
[0.2.0]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.1.2...async-mailer-core-v0.2.0
[0.1.2]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.1.0...async-mailer-core-v0.1.2
[0.1.0]: https://github.com/LeoniePhiline/async-mailer/releases/tag/async-mailer-core-v0.1.0
