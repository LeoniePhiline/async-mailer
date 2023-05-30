//! Create a dynamic mailer trait object depending on runtime mailer configuration.
//!
//! Microsoft Outlook and SMTP mailer variants are available.
//!
//! Example:
//!
//! ```no_run
//! // Create a `BoxMailer`.
//! //
//! // Alternative implementations can be used.
//!
//! let mailer = OutlookMailer::new(
//!     "<Microsoft Identity service tenant>",
//!     "<OAuth2 app GUID>",
//!     "<OAuth2 app secret>"
//! ).await?;
//!
//! // Alternative:
//! let mailer = SmtpMailer::new(
//!     "smtp.example.com",
//!     465,
//!     SmtpInvalidCertsPolicy::Deny,
//!     "<username>",
//!     "<password>"
//! );
//!
//! // The implementation is `Send` and `Sync` and may be stored e.g. as part of your server state.
//!
//! // Build a message using the re-exported `mail_builder::MessageBuilder'.
//! // For blazingly fast rendering of beautiful HTML mail, I recommend combining `askama` with `mrml`.
//! let message = MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body");
//!
//! // Send the message using the implementation-agnostic `dyn DynMailer`.
//! mailer.send_mail(&message).await?;
//! ```

pub use secrecy::Secret;

pub use async_mailer_core::mail_send;
pub use async_mailer_core::mail_send::mail_builder;

pub use async_mailer_core::mail_send::mail_builder::MessageBuilder;
pub use async_mailer_core::mail_send::smtp::message::Message;

pub use async_mailer_core::DynMailer;

#[cfg(feature = "outlook")]
pub use async_mailer_outlook::*;

#[cfg(feature = "smtp")]
pub use async_mailer_smtp::*;
