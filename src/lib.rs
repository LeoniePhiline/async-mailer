//! Create a dynamic mailer trait object depending on runtime mailer configuration.
//!
//! Microsoft Outlook and SMTP mailer variants are available.
//!
//! # Examples
//!
//! ## Using the strongly typed `Mailer`:
//!
//! ```no_run
//! # async fn test() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an `impl Mailer`.
//! //
//! // Alternative implementations can be used.
//!
//! let mailer = async_mailer::OutlookMailer::new(
//!     "<Microsoft Identity service tenant>".into(),
//!     "<OAuth2 app GUID>".into(),
//!     async_mailer::Secret::new("<OAuth2 app secret>".into())
//! ).await?;
//!
//! // Alternative:
//! let mailer = async_mailer::SmtpMailer::new(
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
//! // For blazingly fast rendering of beautiful HTML mail, I recommend combining `askama` with `mrml`.
//! use async_mailer::IntoMessage;
//! let message = async_mailer::MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body")
//!     .into_message()?;
//!
//! // Send the message using the strongly typed `Mailer`.
//! use async_mailer::Mailer;
//! mailer.send_mail(message).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Using the dynamically typed `DynMailer`:
//!
//! ```no_run
//! # async fn test() -> Result<(), async_mailer::DynMailerError> {
//! // Create a `BoxMailer`.
//! //
//! // Alternative implementations can be used.
//!
//! use async_mailer::BoxMailer;
//! let mailer: BoxMailer = async_mailer::OutlookMailer::new_box( // Or `new_arc` to use in e.g. globally shared server state.
//!     "<Microsoft Identity service tenant>".into(),
//!     "<OAuth2 app GUID>".into(),
//!     async_mailer::Secret::new("<OAuth2 app secret>".into())
//! ).await?;
//!
//! // Alternative:
//! let mailer: BoxMailer = async_mailer::SmtpMailer::new_box( // Or `new_arc` to use in e.g. globally shared server state.
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
//! // For blazingly fast rendering of beautiful HTML mail, I recommend combining `askama` with `mrml`.
//! use async_mailer::IntoMessage;
//! let message = async_mailer::MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body")
//!     .into_message()?;
//!
//! // Send the message using the implementation-agnostic `dyn DynMailer`.
//! mailer.send_mail(message).await?;
//! # Ok(())
//! # }
//! ```

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
