# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] <!-- release-date -->

### Added

- Vastly improve documentation.

## [0.2.1] - 2023-05-30

## [0.2.0] - 2023-05-30

### BREAKING CHANGES

- Rename type-erased, object safe `trait Mailer` to `trait DynMailer`.

### Added

- Add strongly typed `Mailer` trait to be used as generic trait bound.
- Add `BoxMailer` and `ArcMailer` type aliases for `dyn DynMailer` wrapped in smart pointers.
- Add a `CHANGELOG.md` and configure `cargo-release`.

## [0.1.2] - 2023-05-16

### Added

- Add crate link to documentation.

## [0.1.0] - 2023-05-16

### Added

- Initial implementation.

<!-- next-url -->
[Unreleased]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.2.1...HEAD
[0.2.1]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.2.0...async-mailer-core-v0.2.1
[0.2.0]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.1.2...async-mailer-core-v0.2.0
[0.1.2]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-core-v0.1.0...async-mailer-core-v0.1.2
[0.1.0]: https://github.com/LeoniePhiline/async-mailer/releases/tag/async-mailer-core-v0.1.0
