# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] <!-- release-date -->

### Fixed

- Fix link in crate documentation.

## [0.2.2] - 2023-05-31

### Added

- Add `clap` feature flag, which implements [`clap::ValueEnum`](https://docs.rs/clap/latest/clap/trait.ValueEnum.html)
  for `SmtpInvalidCertsPolicy`.
- Implement `Default` for `SmtpInvalidCertsPolicy::Deny`.
- Vastly improve documentation.

## [0.2.1] - 2023-05-30

### Fixed

- Corrected diff links in changelog.
- Fix lib doctests.

## [0.2.0] - 2023-05-30

### BREAKING CHANGES

- Dependency `async-mailer` (`async-mailer-core`) renames type-erased, object safe `trait Mailer` to `trait DynMailer`.

### Added

- Add `impl` for strongly typed `Mailer` trait to be used as generic trait bound.
- Add `new_box` and `new_arc` to create new `BoxMailer` and `ArcMailer` type aliases for `dyn DynMailer` wrapped in smart pointers.
- Add a `CHANGELOG.md` and configure `cargo-release`.

## [0.1.1] - 2023-05-16

### Added

- Add crate link to documentation.

## [0.1.0] - 2023-05-16

### Added

- Initial implementation.

<!-- next-url -->
[Unreleased]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-smtp-v0.2.2...HEAD
[0.2.2]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-smtp-v0.2.1...async-mailer-smtp-v0.2.2
[0.2.1]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-smtp-v0.2.0...async-mailer-smtp-v0.2.1
[0.2.0]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-smtp-v0.1.2...async-mailer-smtp-v0.2.0
[0.1.2]: https://github.com/LeoniePhiline/async-mailer/compare/async-mailer-smtp-v0.1.0...async-mailer-smtp-v0.1.2
[0.1.0]: https://github.com/LeoniePhiline/async-mailer/releases/tag/async-mailer-smtp-v0.1.0
