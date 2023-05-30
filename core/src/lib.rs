//! Core trait for `async_mailer`. Use `async_mailer` instead.
use std::{fmt::Debug, sync::Arc};

pub use async_trait::async_trait;

pub use mail_send;
use mail_send::smtp::message::Message;

// == Mailer ==

#[async_trait]
pub trait Mailer: Debug + Send + Sync {
    type Error;

    async fn send_mail(&self, message: Message<'_>) -> Result<(), Self::Error>;
}

// == DynMailer ==

/// Type-erased mailer error, for use of [`DynMailer`] as trait object.
pub type DynMailerError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Object-safe [`DynMailer`] trait, usable as [`ArcMailer`] (`Arc<dyn DynMailer>`) or `BoxMailer` (`Box<dyn DynMailer>`).
#[async_trait]
pub trait DynMailer: Debug + Send + Sync {
    async fn send_mail(&self, message: Message<'_>) -> Result<(), DynMailerError>;
}

pub type BoxMailer = Box<dyn DynMailer>;
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
