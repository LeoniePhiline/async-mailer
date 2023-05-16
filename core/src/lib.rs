//! Core trait for `async_mailer`. Use `async_mailer` instead.
use std::fmt::Debug;

pub use async_trait::async_trait;

pub use mail_send;
use mail_send::smtp::message::Message;

/// Type-erased mailer error, for use of [`Mailer`] as trait object.
pub type MailerError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Object-safe [`Mailer`] trait, usable as `Arc<dyn Mailer>` or `Box<dyn Mailer>`.
#[async_trait]
pub trait Mailer: Debug + Send + Sync {
    async fn send_mail(&self, message: Message<'_>) -> Result<(), MailerError>;
}

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
