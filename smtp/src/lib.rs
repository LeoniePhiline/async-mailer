//! An SMTP mailer, usable either stand-alone or as either generic `Mailer` or dynamic `dyn DynMailer` using the `mail-send` crate.
//!
//! **Preferably, use [`async-mailer`](https://docs.rs/async-mailer), which re-exports from this crate,
//! rather than using `async-mailer-smtp` directly.**
//!
//! You can control the re-exported mailer implementations,
//! as well as [`tracing`](https://docs.rs/crate/tracing) support,
//! via [`async-mailer` feature toggles](https://docs.rs/crate/async-mailer/latest/features).
//!
//! **Note:**
//! If you are planning to always use `SmtpMailer` and do not need `async_mailer_outlook::OutlookMailer`
//! or `async_mailer::BoxMailer`, then consider using the [`mail-send`](https://docs.rs/mail-send) crate directly.
//!
//! # Examples
//!
//! ## Using the strongly typed `Mailer`:
//!
//! ```no_run
//! # async fn test() -> Result<(), Box<dyn std::error::Error>> {
//! // Both `async_mailer::OutlookMailer` and `async_mailer::SmtpMailer` implement `Mailer`
//! // and can be used with `impl Mailer` or `<M: Mailer>` bounds.
//!
//! # use async_mailer_smtp::{ SmtpMailer, SmtpInvalidCertsPolicy };
//! let mailer = SmtpMailer::new(
//!     "smtp.example.com".into(),
//!     465,
//!     SmtpInvalidCertsPolicy::Deny,
//!     "<username>".into(),
//!     secrecy::Secret::new("<password>".into())
//! );
//!
//! // An alternative `OutlookMailer` can be found at `async-mailer-outlook`.
//! // Further alternative mailers can be implemented by third parties.
//!
//! // Build a message using the re-exported `mail_builder::MessageBuilder'.
//! //
//! // For blazingly fast rendering of beautiful HTML mail,
//! // I recommend combining `askama` with `mrml`.
//!
//! # use async_mailer_core::mail_send::smtp::message::IntoMessage;
//! let message = async_mailer_core::mail_send::mail_builder::MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body")
//!     .into_message()?;
//!
//! // Send the message using the strongly typed `Mailer`.
//!
//! # use async_mailer_core::Mailer;
//! mailer.send_mail(message).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Using the dynamically typed `DynMailer`:
//!
//! ```no_run
//! # async fn test() -> Result<(), async_mailer_core::DynMailerError> {
//! // Both `async_mailer::OutlookMailer` and `async_mailer::SmtpMailer`
//! // implement `DynMailer` and can be used as trait objects.
//! //
//! // Here they are used as `BoxMailer`, which is an alias to `Box<dyn DynMailer>`.
//!
//! # use async_mailer_core::BoxMailer;
//! # use async_mailer_smtp::{ SmtpMailer, SmtpInvalidCertsPolicy };
//! let mailer: BoxMailer = SmtpMailer::new_box( // Or `SmtpMailer::new_arc()`.
//!     "smtp.example.com".into(),
//!     465,
//!     SmtpInvalidCertsPolicy::Deny,
//!     "<username>".into(),
//!     secrecy::Secret::new("<password>".into())
//! );
//!
//! // An alternative `OutlookMailer` can be found at `async-mailer-outlook`.
//! // Further alternative mailers can be implemented by third parties.
//!
//! // The trait object is `Send` and `Sync` and may be stored e.g. as part of your server state.
//!
//! // Build a message using the re-exported `mail_builder::MessageBuilder'.
//! //
//! // For blazingly fast rendering of beautiful HTML mail,
//! // I recommend combining `askama` with `mrml`.
//!
//! # use async_mailer_core::mail_send::smtp::message::IntoMessage;
//! let message = async_mailer_core::mail_send::mail_builder::MessageBuilder::new()
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
//! - `tracing`: Enable debug and error logging using the [`tracing`](https://docs.rs/crate/tracing) crate.
//!   All relevant functions are instrumented.
//! - `clap`: Implement [`clap::ValueEnum`](https://docs.rs/clap/latest/clap/trait.ValueEnum.html) for [`SmtpInvalidCertsPolicy`].
//!   This allows for easily configured CLI options like `--invalid-certs <allow|deny>`.
//!
//! Default: `tracing`.
//!
//! ## Roadmap
//!
//! DKIM support is planned to be implemented on the [`SmtpMailer`].

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

#[cfg(feature = "clap")]
use clap;

use secrecy::{ExposeSecret, Secret};

#[cfg(feature = "tracing")]
use tracing::{error, info, instrument};

use async_mailer_core::mail_send::{self, smtp::message::Message, SmtpClientBuilder};
use async_mailer_core::{util, ArcMailer, BoxMailer, DynMailer, DynMailerError, Mailer};

/// Error returned by [`SmtpMailer::new`] and [`SmtpMailer::send_mail`].
#[derive(Debug, thiserror::Error)]
pub enum SmtpMailerError {
    /// Could not connect to SMTP host.
    #[error("could not connect to SMTP host")]
    Connect(mail_send::Error),

    /// Could not send SMTP mail.
    #[error("could not send SMTP mail")]
    Send(mail_send::Error),
}

/// Pass to [`SmtpMailer::new`] to either allow or deny invalid SMTP certificates.
///
/// This option allows to perform tests or local development work against
/// SMTP development servers like MailHog or MailPit, while using a self-signed certificate.
///
/// **Never use [`SmtpInvalidCertsPolicy::Allow`] in production!**
// TODO: derive Clap ValueEnum
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum SmtpInvalidCertsPolicy {
    /// Allow connecting to SMTP servers with invalid TLS certificates.
    ///
    /// **Do not use in production!**
    Allow,

    /// Deny connecting to SMTP servers with invalid TLS certificates.
    ///
    /// This variant is the [`Default`].
    #[default]
    Deny,
}

/// An SMTP mailer client, implementing the [`async_mailer_core::Mailer`] and [`async_mailer_core::DynMailer`] traits
/// to be used as generic mailer or runtime-pluggable trait object.
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

    /// Create a new SMTP mailer client as dynamic `async_mailer::BoxMailer`.
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn new_box(
        host: String,
        port: u16,
        invalid_certs: SmtpInvalidCertsPolicy,
        user: String,
        password: Secret<String>,
    ) -> BoxMailer {
        Box::new(Self::new(host, port, invalid_certs, user, password))
    }

    /// Create a new SMTP mailer client as dynamic `async_mailer::ArcMailer`.
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn new_arc(
        host: String,
        port: u16,
        invalid_certs: SmtpInvalidCertsPolicy,
        user: String,
        password: Secret<String>,
    ) -> ArcMailer {
        Arc::new(Self::new(host, port, invalid_certs, user, password))
    }
}

// == Mailer ==

#[async_trait]
impl Mailer for SmtpMailer {
    type Error = SmtpMailerError;

    /// Send the prepared MIME message via an SMTP connection, using the previously configured credentials.
    ///
    /// # Errors
    ///
    /// Returns an [`SmtpMailerError::Connect`] error if a connection to the SMTP server cannot be established.
    ///
    /// Returns an [`SmtpMailerError::Send`] error if the connection was established but sending the e-mail message failed.
    async fn send_mail(&self, message: Message<'_>) -> Result<(), Self::Error> {
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

// == DynMailer ==

#[async_trait]
impl DynMailer for SmtpMailer {
    /// Send the prepared MIME message via an SMTP connection, using the previously configured credentials.
    ///
    /// # Errors
    ///
    /// Returns a boxed, type-erased [`SmtpMailerError::Connect`] error if a connection to the SMTP server cannot be established.
    ///
    /// Returns a boxed, type-erased [`SmtpMailerError::Send`] error if the connection was established but sending the e-mail message failed.
    #[cfg_attr(feature = "tracing", instrument(skip(message)))]
    async fn send_mail(&self, message: Message<'_>) -> Result<(), DynMailerError> {
        Mailer::send_mail(self, message).await.map_err(Into::into)
    }
}
