# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] <!-- release-date -->

## [0.2.0] - 2023-05-30

### BREAKING CHANGES

- Dependency `async-mailer` (`async-mailer-core`) renames type-erased, object safe `trait Mailer` to `trait DynMailer`.

### Added

- Add `impl` for strongly typed `Mailer` trait to be used as generic trait bound.
- Add `new_box` and `new_arc` to create new `BoxMailer` and `ArcMailer` type aliases for `dyn DynMailer` wrapped in smart pointers.
- Add a `CHANGELOG.md` and configure `cargo-release`.

## [0.1.2] - 2023-05-16

### Added

- Add crate link to documentation.

## [0.1.1] - 2023-05-16

### Changed

- Refactor `success?`, removing unnecessary binding.

## [0.1.0] - 2023-05-16

### Added

- Initial implementation.

<!-- next-url -->
[Unreleased]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-outlook-v0.2.0...HEAD
[0.2.0]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-outlook-v0.1.2...async-mailer-outlook-v0.2.0
[0.1.2]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-outlook-v0.1.1...async-mailer-outlook-v0.1.2
[0.1.1]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-outlook-v0.1.0...async-mailer-outlook-v0.1.1
[0.1.0]: https://github.com/LeoniePhiline/async-mailer/releases/tag/async-mailer-outlook-v0.1.0