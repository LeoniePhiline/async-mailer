//! Create a dynamic mailer trait object depending on runtime mailer configuration.
//!
//! Microsoft Outlook and SMTP mailer variants are available.
//!
//! Example:
//!
//! ```no_run
//! // Outlook configuration, e.g. from command line arguments or environment variables.
//! let mailer_configuration = MailerConfiguration::Outlook {
//!     tenant: "<Microsoft Identity service tenant>",
//!     app_guid: "<OAuth2 app GUID>",
//!     secret: "<OAuth2 app secret>"
//! };
//!
//! // Alternative: SMTP configuration, e.g. from command line arguments or environment variables.
//! let mailer_configuration = MailerConfiguration::Outlook {
//!     host: "smtp.example.com",
//!     port: 465,
//!     invalid_certs: SmtpInvalidCertsPolicy::Deny,
//!     user: "<username>",
//!     password: "<password>"
//! };
//!
//! // Create a `Box<dyn Mailer>`.
//! // The implementation is `Send` and `Sync` and may be store e.g. as part of your server state.
//! let mailer = new_mailer(mailer_configuration).await?;
//!
//! // Build a message using the re-exported `mail_builder::MessageBuilder'.
//! // For blazingly fast rendering of beautiful HTML mail, I recommend combining `askama` with `mrml`.
//! let message = MessageBuilder::new()
//!     .from(("From Name", "from@example.com"))
//!     .to("to@example.com")
//!     .subject("Subject")
//!     .text_body("Mail body");
//!
//! // Send the message using the implementation-agnostic `dyn Mailer`.
//! mailer.send_mail(&message).await?;
//! ```

use std::sync::Arc;

pub use secrecy::Secret;

#[cfg(feature = "tracing")]
use tracing::instrument;

pub use async_mailer_core::mail_send::mail_builder::MessageBuilder;
pub use async_mailer_core::mail_send::smtp::message::Message;
pub use async_mailer_core::Mailer;

#[cfg(feature = "outlook")]
pub use async_mailer_outlook::*;

#[cfg(feature = "smtp")]
pub use async_mailer_smtp::*;

/// Mailer configuration helper, which can be used with clap.
#[derive(Clone, Debug)]
pub enum MailerConfiguration {
    #[cfg(feature = "outlook")]
    Outlook {
        /// Microsoft Active Directory tenant
        tenant: String,

        /// Outlook OAuth 2.0 application ID
        app_guid: String,

        /// Outlook OAuth 2.0 client secret
        secret: Secret<String>,
    },

    #[cfg(feature = "smtp")]
    Smtp {
        /// SMTP host
        host: String,

        /// SMTP port
        port: u16,

        /// Allow or deny invalid (self-signed) certificates. Set to 'deny' except on local test environments
        invalid_certs: SmtpInvalidCertsPolicy,

        /// SMTP user
        user: String,

        /// SMTP password
        password: Secret<String>,
    },
}

/// Create a new dynamic mailer trait object depending on the provided [`MailerConfiguration`].
#[cfg_attr(feature = "tracing", instrument)]
pub async fn new_mailer<M: Mailer>(
    mailer_configuration: MailerConfiguration,
) -> Result<Arc<dyn Mailer>, MailerError> {
    match mailer_configuration {
        // Outlook mailer - used in production
        #[cfg(feature = "outlook")]
        MailerConfiguration::Outlook {
            tenant,
            app_guid,
            secret,
        } => Ok(Arc::new(
            OutlookMailer::new(tenant, app_guid, secret).await?,
        )),

        // SMTP mailer - used during development and testing
        #[cfg(feature = "smtp")]
        MailerConfiguration::Smtp {
            host,
            port,
            invalid_certs,
            user,
            password,
        } => Ok(Arc::new(SmtpMailer::new(
            host,
            port,
            invalid_certs,
            user,
            password,
        ))),
    }
}
