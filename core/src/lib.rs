//! Core trait for `async-mailer`. Use [`async-mailer`](https://docs.rs/async-mailer/latest/async_mailer/) instead.
use std::{fmt::Debug, sync::Arc};

pub use async_trait::async_trait;

pub use mail_send;
use mail_send::smtp::message::Message;

// == Mailer ==

/// Statically typed [`Mailer`], to be used in `impl Mailer` or `<M: Mailer>` bounds.
///
/// The `async-mailer` crate exports Microsoft Outlook and SMTP mailers implementing the [`Mailer`] and [`DynMailer`] traits.
#[async_trait]
pub trait Mailer: Debug + Send + Sync {
    type Error;

    /// Send a [`Message`] using the [`Mailer`] implementation.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] in case sending the mail fails.
    ///
    /// Concrete errors vary by [`Mailer`] trait implementation.
    /// ([Outlook](https://docs.rs/async-mailer/latest/async_mailer/struct.OutlookMailer.html#impl-Mailer-for-OutlookMailer),
    /// [SMTP](https://docs.rs/async-mailer/latest/async_mailer/struct.SmtpMailer.html#impl-Mailer-for-SmtpMailer))
    async fn send_mail(&self, message: Message<'_>) -> Result<(), Self::Error>;
}

// == DynMailer ==

/// Type-erased mailer error, for use of [`DynMailer`] as trait object.
pub type DynMailerError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Object-safe [`DynMailer`] trait, usable as `&DynMailer`, [`ArcMailer`] (`Arc<dyn DynMailer>`) or [`BoxMailer`] (`Box<dyn DynMailer>`).
///
/// The `async-mailer` crate exports Microsoft Outlook and SMTP mailers implementing the [`DynMailer`] and [`Mailer`] traits.
#[async_trait]
pub trait DynMailer: Debug + Send + Sync {
    /// Send a [`Message`] using the [`DynMailer`] implementation.
    ///
    /// # Errors
    ///
    /// Returns a boxed, type-erased [`DynMailerError`] in case sending the mail fails.
    ///
    /// Concrete errors vary by [`DynMailer`] trait implementation.
    /// ([Outlook](https://docs.rs/async-mailer/latest/async_mailer/struct.OutlookMailer.html#impl-DynMailer-for-OutlookMailer),
    /// [SMTP](https://docs.rs/async-mailer/latest/async_mailer/struct.SmtpMailer.html#impl-DynMailer-for-SmtpMailer))
    async fn send_mail(&self, message: Message<'_>) -> Result<(), DynMailerError>;
}

/// Boxed dyn [`DynMailer`]
pub type BoxMailer = Box<dyn DynMailer>;

/// Arc-wrapped dyn [`DynMailer`]
pub type ArcMailer = Arc<dyn DynMailer>;

pub mod util {
    use super::Message;

    #[cfg(feature = "tracing")]
    /// Extract recipient addresses for tracing log output.
    pub fn format_recipient_addresses(message: &Message<'_>) -> String {
        let recipient_addresses = message
            .rcpt_to
            .iter()
            .map(|address| address.email.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        recipient_addresses
    }
}
