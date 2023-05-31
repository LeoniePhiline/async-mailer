//! # async-mailer
//! A set of async generic `Mailer` and dynamic `dyn DynMailer` traits with runtime-pluggable Outlook (Office365) and SMTP implementations.
//!
//! [![Crates.io](https://img.shields.io/crates/v/async-mailer)](https://crates.io/crates/async-mailer)
//! [![Documentation](https://docs.rs/async-mailer/badge.svg)][docs]
//! [![Dependency status](https://deps.rs/repo/github/LeoniePhiline/async-mailer/status.svg)](https://deps.rs/repo/github/LeoniePhiline/async-mailer)
//!
//! ## Installation
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! async-mailer = "0.3.5"
//! ```
//!
//! You can control the re-exported mailer implementations,
//! as well as [`tracing`](https://docs.rs/crate/tracing) support,
//! via [crate feature toggles](https://docs.rs/crate/async-mailer/latest/features).
//!
//! By default, features `smtp`, `outlook` and `tracing` are enabled.
//! Use `default-features = false` and `features = [...]` to select features individually.
//!
//! # Examples
//!
//! Use `new` for a strongly typed mailer instance,
//! or `new_box` / `new_arc` for a type-erased dynamic mailer.
//!
//! Microsoft Outlook and SMTP mailer variants are available.
//!
//! ## Using the strongly typed `Mailer`:
//!
//! ```no_run
//! # async fn test() -> Result<(), Box<dyn std::error::Error>> {
//! // Both `OutlookMailer` and `SmtpMailer` implement `Mailer`
//! // and can be used with `impl Mailer` or `<M: Mailer>` bounds.
//!
//! use async_mailer::{ IntoMessage, Mailer, OutlookMailer, SmtpMailer };
//!
//! let mailer: OutlookMailer = OutlookMailer::new(
//!     "<Microsoft Identity service tenant>".into(),
//!     "<OAuth2 app GUID>".into(),
//!     async_mailer::Secret::new("<OAuth2 app secret>".into())
//! ).await?;
//!
//! // Alternative:
//!
//! let mailer: SmtpMailer = SmtpMailer::new(
//!     "smtp.example.com".into(),
//!     465,
//!     async_mailer::SmtpInvalidCertsPolicy::Deny,
//!     "<username>".into(),
//!     async_mailer::Secret::new("<password>".into())
//! );
//!
//! // Further alternative mailers can be implemented by third parties.
//!
//! // Build a message using the re-exported `mail_builder::MessageBuilder'.
//! //
//! // For blazingly fast rendering of beautiful HTML mail,
//! // I recommend combining `askama` with `mrml`.
//!
//! let message = async_mailer::MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body")
//!     .into_message()?;
//!
//! // Send the message using the strongly typed `Mailer`.
//!
//! mailer.send_mail(message).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Using the dynamically typed `DynMailer`:
//!
//! ```no_run
//! # async fn test() -> Result<(), async_mailer::DynMailerError> {
//! use async_mailer::{ BoxMailer, IntoMessage, OutlookMailer, SmtpMailer };
//!
//! // Both `OutlookMailer` and `SmtpMailer` implement `DynMailer` and can be used as trait objects.
//! // Here they are used as `BoxMailer`, which is an alias to `Box<dyn DynMailer>`.
//!
//! let mailer: BoxMailer = OutlookMailer::new_box( // Or `OutlookMailer::new_arc()`.
//!     "<Microsoft Identity service tenant>".into(),
//!     "<OAuth2 app GUID>".into(),
//!     async_mailer::Secret::new("<OAuth2 app secret>".into())
//! ).await?;
//!
//! // Alternative:
//!
//! let mailer: BoxMailer = SmtpMailer::new_box( // Or `SmtpMailer::new_arc()`.
//!     "smtp.example.com".into(),
//!     465,
//!     async_mailer::SmtpInvalidCertsPolicy::Deny,
//!     "<username>".into(),
//!     async_mailer::Secret::new("<password>".into())
//! );
//!
//! // Further alternative mailers can be implemented by third parties.
//!
//! // The trait object is `Send` and `Sync` and may be stored e.g. as part of your server state.
//!
//! // Build a message using the re-exported `mail_builder::MessageBuilder'.
//! //
//! // For blazingly fast rendering of beautiful HTML mail,
//! // I recommend combining `askama` with `mrml`.
//!
//! let message = async_mailer::MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body")
//!     .into_message()?;
//!
//! // Send the message using the implementation-agnostic `dyn DynMailer`.
//!
//! mailer.send_mail(message).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Feature flags
//!
//! - `outlook`: Enable [`OutlookMailer`].
//! - `smtp`: Enable [`SmtpMailer`].
//! - `tracing`: Enable debug and error logging using the [`tracing`](https://docs.rs/crate/tracing) crate.
//!   All relevant functions are instrumented.
//! - `clap`: Implement [`clap::ValueEnum`](https://docs.rs/clap/latest/clap/trait.ValueEnum.html) for [`SmtpInvalidCertsPolicy`].
//!   This allows for easily configured CLI options like `--invalid-certs <allow|deny>`.
//!
//! Default: `outlook`, `smtp`, `tracing`.
//!
//! ## Roadmap
//!
//! - DKIM support is planned to be implemented on the [`SmtpMailer`].
//! - Access token auto-refresh is planned to be implemented on the [`OutlookMailer`].
//!
//! Further mailer implementations are possible.
//! Please open an issue and ideally provide a pull request to add your alternative mailer implementation!
//!
//! [docs]: https://docs.rs/async-mailer

pub use secrecy::Secret;

pub use async_mailer_core::mail_send;
pub use async_mailer_core::mail_send::mail_builder;

pub use async_mailer_core::mail_send::mail_builder::MessageBuilder;
pub use async_mailer_core::mail_send::smtp::message::{IntoMessage, Message};

// == Mailer ==
pub use async_mailer_core::Mailer;

// == DynMailer ==
pub use async_mailer_core::{ArcMailer, BoxMailer, DynMailer, DynMailerError};

#[cfg(feature = "outlook")]
pub use async_mailer_outlook::*;

#[cfg(feature = "smtp")]
pub use async_mailer_smtp::*;
