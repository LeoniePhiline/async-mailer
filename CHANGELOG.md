# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] <!-- release-date -->

### Added

- SMTP mailer: Add `clap` feature flag, which implements [`clap::ValueEnum`](https://docs.rs/clap/latest/clap/trait.ValueEnum.html)
  for `SmtpInvalidCertsPolicy`.
- Implement `Default` for `SmtpInvalidCertsPolicy::Deny`.
- Vastly improve documentation.

## [0.3.3] - 2023-05-30

### Added

- Add links to crates.io and docs.rs in README.

### Changed

- Fine tune some documentation.

## [0.3.2] - 2023-05-30

### Fixed

- Re-export `DynMailerError` type definition from `async-mailer-core`.
- Fix crate usage examples.

## [0.3.1] - 2023-05-30

### Fixed

- Re-export traits and type definitions from `async-mailer-core`.

## [0.3.0] - 2023-05-30

### BREAKING CHANGES

- Rename type-erased, object safe `trait Mailer` to `trait DynMailer`.

### Added

- Add strongly typed `Mailer` trait to be used as generic trait bound.
- Add `BoxMailer` and `ArcMailer` type aliases for `dyn DynMailer` wrapped in smart pointers.
- Add a `CHANGELOG.md` and configure `cargo-release`.

## [0.2.1] - 2023-05-16

### Added

- Add crate link to documentation.

## [0.2.0] - 2023-05-16

### BREAKING CHANGES

- Remove inflexible `new_mailer` fn and `MailerConfiguration` enum.

## [0.1.2] - 2023-05-16

### Added

- Re-export mail_send and mail_builder for downstream use.

## [0.1.1] - 2023-05-16

### Fixed

- Remove residual generic from `new_mailer` function.

## [0.1.0] - 2023-05-16

### Added

- Initial implementation.

<!-- next-url -->
[Unreleased]: https://github.com/LeoniePhiline/async-mailer/compare/v0.3.3...HEAD
[0.3.3]: https://github.com/LeoniePhiline/async-mailer/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/LeoniePhiline/async-mailer/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/LeoniePhiline/async-mailer/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/LeoniePhiline/async-mailer/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/LeoniePhiline/async-mailer/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/LeoniePhiline/async-mailer/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/LeoniePhiline/async-mailer/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/LeoniePhiline/async-mailer/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/LeoniePhiline/async-mailer/releases/tag/v0.1.0
