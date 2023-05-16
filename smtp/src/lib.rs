//! An SMTP mailer, usable either stand-alone or as `dyn Mailer` using the `async_mailer` crate.
//!
//! Note:
//! If you are planning to always use `SmtpMailer` and do not need `async_mailer_outlook::OutlookMailer`
//! or `Box<dyn async_mailer::Mailer`, then consider using the `mail_send` crate directly.
//!
//! Example:
//! ```no_run
//! let mailer = SmtpMailer::new(
//!     "smtp.example.com",
//!     465,
//!     SmtpInvalidCertsPolicy::Deny,
//!     "<username>",
//!     "<password>"
//! ).await?;
//!
//! let message = MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body");
//!
//! mailer.send_mail(&message).await?;
//! ```

use std::time::Duration;

use async_trait::async_trait;
use secrecy::{ExposeSecret, Secret};

#[cfg(feature = "tracing")]
use tracing::{error, info, instrument};

use async_mailer_core::mail_send::{self, smtp::message::Message, SmtpClientBuilder};
pub use async_mailer_core::Mailer;
use async_mailer_core::{util, MailerError};

/// Error returned by [`SmtpMailer::new`] and [`SmtpMailer::send_mail`].
#[derive(Debug, thiserror::Error)]
pub enum SmtpMailerError {
    #[error("could not connect to SMTP host")]
    Connect(mail_send::Error),

    #[error("could not send SMTP mail")]
    Send(mail_send::Error),
}

/// Pass to [`SmtpMailer::new`] to either allow or deny invalid SMTP certificates.
///
/// This option allows to perform tests or local development work against
/// SMTP development servers like MailHog or MailPit, while using a self-signed certificate.
///
/// Never use [`SmtpInvalidCertsPolicy::Allow`] in production!
#[derive(Clone, Debug)]
pub enum SmtpInvalidCertsPolicy {
    Allow,
    Deny,
}

/// An SMTP mailer client, implementing the `async_mailer::Mailer` trait to be used as runtime-pluggable trait object.
///
/// An abstraction over `mail_send`, sending mail via an SMTP connection.
///
/// Self-signed certificates can optionally be accepted, to use the SMTP mailer in development while using the Outlook mailer in production.
#[derive(Clone)]
pub struct SmtpMailer {
    inner: SmtpClientBuilder<String>,
}

impl std::fmt::Debug for SmtpMailer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client (SMTP)").finish()
    }
}

impl SmtpMailer {
    /// Create a new SMTP mailer client.
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn new(
        host: String,
        port: u16,
        invalid_certs: SmtpInvalidCertsPolicy,
        user: String,
        password: Secret<String>,
    ) -> Self {
        let mut smtp_client = SmtpClientBuilder::new(host, port)
            .credentials((user, password.expose_secret().into()))
            .timeout(Duration::from_secs(30));

        if matches!(invalid_certs, SmtpInvalidCertsPolicy::Allow) {
            smtp_client = smtp_client.allow_invalid_certs();
        }

        Self { inner: smtp_client }
    }
}

#[async_trait]
impl Mailer for SmtpMailer {
    /// Send the prepared MIME message via an SMTP connection.
    #[cfg_attr(feature = "tracing", instrument(skip(message)))]
    async fn send_mail(&self, message: Message<'_>) -> Result<(), MailerError> {
        #[cfg(feature = "tracing")]
        // Extract recipient addresses for tracing log output.
        let recipient_addresses = util::format_recipient_addresses(&message);

        info!("Sending SMTP mail to {recipient_addresses}...");

        let connection = self.inner.connect().await;

        #[cfg(feature = "tracing")]
        match &connection {
            Ok(_) => {}
            Err(error) => error!(
                ?error,
                "Failed to connect to SMTP host for mail to {recipient_addresses}"
            ),
        }

        let response = connection
            .map_err(SmtpMailerError::Connect)?
            .send(message)
            .await;

        #[cfg(feature = "tracing")]
        match &response {
            Ok(_) => {
                info!("Sent SMTP mail to {recipient_addresses}");
            }
            Err(error) => {
                error!(?error, "Failed to send SMTP mail to {recipient_addresses}");
            }
        }

        Ok(response.map_err(SmtpMailerError::Send)?)
    }
}
