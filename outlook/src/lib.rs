//! An Outlook mailer, usable either stand-alone or as either generic `Mailer` or dynamic `dyn DynMailer`.
//!
//! **Preferably, use [`async-mailer`](https://docs.rs/async-mailer), which re-exports from this crate,
//! rather than using `async-mailer-outlook` directly.**
//!
//! You can control the re-exported mailer implementations,
//! as well as [`tracing`](https://docs.rs/crate/tracing) support,
//! via [`async-mailer` feature toggles](https://docs.rs/crate/async-mailer/latest/features).
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
//! # use async_mailer_outlook::OutlookMailer;
//! let mailer = OutlookMailer::new(
//!     "<Microsoft Identity service tenant>".into(),
//!     "<OAuth2 app GUID>".into(),
//!     secrecy::Secret::new("<OAuth2 app secret>".into())
//! ).await?;
//!
//! // An alternative `SmtpMailer` can be found at `async-mailer-smtp`.
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
//! # use async_mailer_outlook::OutlookMailer;
//! let mailer: BoxMailer = OutlookMailer::new_box( // Or `OUtlookMailer::new_arc()`.
//!     "<Microsoft Identity service tenant>".into(),
//!     "<OAuth2 app GUID>".into(),
//!     secrecy::Secret::new("<OAuth2 app secret>".into())
//! ).await?;
//!
//! // An alternative `SmtpMailer` can be found at `async-mailer-smtp`.
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
//!
//! Default: `tracing`.
//!
//! ## Roadmap
//!
//! Access token auto-refresh is planned to be implemented on the [`OutlookMailer`].

use std::sync::Arc;

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as base64_engine, Engine as _};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[cfg(feature = "tracing")]
use tracing::{debug, error, info, instrument};

use async_mailer_core::mail_send::smtp::message::Message;
use async_mailer_core::{util, ArcMailer, BoxMailer, DynMailer, DynMailerError, Mailer};

/// Error returned by [`OutlookMailer::new`] and [`OutlookMailer::send_mail`].
#[derive(Debug, thiserror::Error)]
pub enum OutlookMailerError {
    /// Failed to retrieve Microsoft Graph API access token.
    #[error("failed to retrieve Microsoft Graph API access token")]
    RetrieveAccessToken(#[from] OutlookAccessTokenError),

    /// Failed request attempting to send Outlook MIME mail through Microsoft Graph API.
    #[error("failed request attempting to send Outlook MIME mail through Microsoft Graph API")]
    SendMailRequest(reqwest::Error),

    /// Failed sending Outlook MIME mail through Microsoft Graph API.
    #[error("failed sending Outlook MIME mail through Microsoft Graph API")]
    SendMailResponse(reqwest::Error),

    /// Failed retrieving response body from Microsoft Graph API.
    /// (Crate feature `tracing` only.)
    #[cfg(feature = "tracing")]
    #[error("failed retrieving response body from Microsoft Graph API")]
    SendMailResponseBody(reqwest::Error),
}

/// Error returned by [`OutlookMailer::new`] if an access token cannot be retrieved.
#[derive(Debug, thiserror::Error)]
pub enum OutlookAccessTokenError {
    /// Failed sending OAuth2 client credentials grant access token request to Microsoft Identity service.
    #[error("failed sending OAuth2 client credentials grant access token request to Microsoft Identity service")]
    SendRequest(reqwest::Error),

    /// Failed receiving OAuth2 client credentials grant access token response from Microsoft Identity service.
    #[error("failed receiving OAuth2 client credentials grant access token response from Microsoft Identity service")]
    ReceiveResponse(reqwest::Error),

    /// Failed to parse OAuth2 client credentials grant access token response from Microsoft Identity service.
    #[error("failed to parse OAuth2 client credentials grant access token response from Microsoft Identity service")]
    ParseResponse(serde_json::Error),
}

/// An Outlook mailer client, implementing the `async_mailer::Mailer` and `async_mailer::DynMailer` traits
/// to be used as generic mailer or runtime-pluggable trait object.
///
/// Sends mail authenticated by OAuth2 client credentials grant via the Microsoft Graph API.
#[derive(Clone, Debug)]
pub struct OutlookMailer {
    http_client: reqwest::Client,
    access_token: Secret<String>,
}

impl OutlookMailer {
    /// Create a new Outlook mailer client.
    ///
    /// # Errors
    ///
    /// Returns an [`OutlookMailerError::RetrieveAccessToken`] error
    /// when the attempt to retrieve an access token from the Microsoft Identity Service fails:
    ///
    /// - Wrapping an [`OutlookAccessTokenError::SendRequest`] error if sending the token request fails.
    /// - Wrapping an [`OutlookAccessTokenError::ReceiveResponse`] error if the response body cannot be received.
    /// - Wrapping an [`OutlookAccessTokenError::ParseResponse`] error if the response body bytes cannot be parsed as JSON.
    #[cfg_attr(feature = "tracing", instrument)]
    pub async fn new(
        tenant: String,
        app_guid: String,
        secret: Secret<String>,
    ) -> Result<Self, OutlookMailerError> {
        let http_client = reqwest::Client::new();

        let access_token = Self::get_access_token(&tenant, &app_guid, &secret, http_client.clone())
            .await
            .map_err(OutlookMailerError::RetrieveAccessToken)?;

        Ok(Self {
            http_client,
            access_token,
        })
    }

    /// Create a new Outlook mailer client as dynamic `async_mailer::BoxMailer`.
    ///
    /// # Errors
    ///
    /// Returns an [`OutlookMailerError::RetrieveAccessToken`] error
    /// when the attempt to retrieve an access token from the Microsoft Identity Service fails:
    ///
    /// - Wrapping an [`OutlookAccessTokenError::SendRequest`] error if sending the token request fails.
    /// - Wrapping an [`OutlookAccessTokenError::ReceiveResponse`] error if the response body cannot be received.
    /// - Wrapping an [`OutlookAccessTokenError::ParseResponse`] error if the response body bytes cannot be parsed as JSON.
    #[cfg_attr(feature = "tracing", instrument)]
    pub async fn new_box(
        tenant: String,
        app_guid: String,
        secret: Secret<String>,
    ) -> Result<BoxMailer, OutlookMailerError> {
        Ok(Box::new(Self::new(tenant, app_guid, secret).await?))
    }

    /// Create a new Outlook mailer client as dynamic `async_mailer::ArcMailer`.
    ///
    /// # Errors
    ///
    /// Returns an [`OutlookMailerError::RetrieveAccessToken`] error
    /// when the attempt to retrieve an access token from the Microsoft Identity Service fails:
    ///
    /// - Wrapping an [`OutlookAccessTokenError::SendRequest`] error if sending the token request fails.
    /// - Wrapping an [`OutlookAccessTokenError::ReceiveResponse`] error if the response body cannot be received.
    /// - Wrapping an [`OutlookAccessTokenError::ParseResponse`] error if the response body bytes cannot be parsed as JSON.
    #[cfg_attr(feature = "tracing", instrument)]
    pub async fn new_arc(
        tenant: String,
        app_guid: String,
        secret: Secret<String>,
    ) -> Result<ArcMailer, OutlookMailerError> {
        Ok(Arc::new(Self::new(tenant, app_guid, secret).await?))
    }

    /// Retrieve an OAuth2 client credentials grant access token from the Microsoft Identity service.
    ///
    /// # Errors
    ///
    /// Returns an [`OutlookAccessTokenError::SendRequest`] error if sending the token request fails.
    ///
    /// Returns an [`OutlookAccessTokenError::ReceiveResponse`] error if the response body cannot be received.
    ///
    /// Returns an [`OutlookAccessTokenError::ParseResponse`] error if the response body bytes cannot be parsed as JSON.
    #[cfg_attr(feature = "tracing", instrument)]
    async fn get_access_token(
        tenant_id: &str,
        client_id: &str,
        client_secret: &Secret<String>,
        http_client: reqwest::Client,
    ) -> Result<Secret<String>, OutlookAccessTokenError> {
        let token_url = format!("https://login.microsoftonline.com/{tenant_id}/oauth2/v2.0/token");

        let form_data = [
            ("client_id", client_id),
            ("client_secret", client_secret.expose_secret()),
            ("grant_type", "client_credentials"),
            ("scope", &["https://graph.microsoft.com/.default"].join(" ")),
        ];

        let response = http_client
            .post(&token_url)
            .form(&form_data)
            .send()
            .await
            .map_err(OutlookAccessTokenError::SendRequest)?;

        let response_data = response
            .bytes()
            .await
            .map_err(OutlookAccessTokenError::ReceiveResponse)?;

        let token_response: TokenResponse = serde_json::from_slice(&response_data)
            .map_err(OutlookAccessTokenError::ParseResponse)?;

        Ok(Secret::from(token_response.access_token))
    }
}

// == Mailer ==

#[async_trait]
impl Mailer for OutlookMailer {
    type Error = OutlookMailerError;

    /// Send the prepared MIME message via the Microsoft Graph API.
    /// Strongly typed [`Mailer`] implementation for direct
    /// or generic (`impl Mailer` / `<M: Mailer>`) invocation without vtable dispatch.
    ///
    /// # Errors
    ///
    /// Returns an [`OutlookMailerError::SendMailRequest`] error if sending the mailing request to the
    /// Microsoft Graph API fails.
    ///
    /// Returns an [`OutlookMailerError::SendMailResponse`] error if the Microsoft Graph API responds
    /// with a non-success HTTP status code.
    ///
    /// Returns an [`OutlookMailerError::SendMailResponseBody`] error if the Microsoft Graph API reponse body
    /// cannot be received.
    /// (Crate feature `tracing` only: The response body is only received for logging.)
    async fn send_mail(&self, message: Message<'_>) -> Result<(), Self::Error> {
        // TODO: Token auto-refresh.

        // Extract sender address necessary for Microsoft Graph API call.
        let from_address = message.mail_from.email.to_string();

        #[cfg(feature = "tracing")]
        // Extract recipient addresses for tracing log output.
        let recipient_addresses = {
            let recipient_addresses = util::format_recipient_addresses(&message);

            info!("Sending Outlook mail to {recipient_addresses}...");
            recipient_addresses
        };

        // Encode the message body according to the MIME-mail API endpoint documentation:
        // https://learn.microsoft.com/en-us/graph/api/user-sendmail?view=graph-rest-1.0&tabs=http#example-4-send-a-new-message-using-mime-format
        // See also https://learn.microsoft.com/en-us/graph/outlook-send-mime-message
        let message_base64 = base64_engine.encode(&message.body);

        // Prepare the authorization header with OAuth 2.0 client credentials grant bearer token.
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.access_token.expose_secret())
                .parse()
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, "text/plain".parse().unwrap());

        // Send the mail via Graph API.
        let response = self
            .http_client
            .post(format!(
                "https://graph.microsoft.com/v1.0/users/{from_address}/sendMail",
            ))
            .headers(headers)
            .body(message_base64)
            .send()
            .await
            .map_err(OutlookMailerError::SendMailRequest)?;

        {
            // Get result with empty ok or status code error
            // before moving `response` to consume the body.
            let success = response
                .error_for_status_ref()
                // Un-reference `response`, so we can move out of it with `response.text()`.
                .map(|_| {});

            #[cfg(feature = "tracing")]
            {
                match success {
                    Ok(()) => {
                        info!("Sent Outlook mail to {recipient_addresses}");
                        debug!(?response);
                    }

                    Err(ref error) => {
                        error!(
                            ?error,
                            "Failed to send Outlook mail to {recipient_addresses}"
                        );
                        error!(?response);
                    }
                };

                // Log the response JSON as plain text.
                let response_text = response
                    .text()
                    .await
                    .map_err(OutlookMailerError::SendMailResponseBody)?;
                match &success {
                    Ok(_) => debug!(response_text),
                    Err(_) => error!(response_text),
                }
            }

            success
        }
        .map_err(OutlookMailerError::SendMailResponse)?;

        Ok(())
    }
}

// == DynMailer ==

#[async_trait]
impl DynMailer for OutlookMailer {
    /// Send the prepared MIME message via the Microsoft Graph API.
    /// Dynamically typed [`DynMailer`] implementation for trait object invocation via vtable dispatch.
    ///
    /// # Errors
    ///
    /// Returns a boxed, type-erased [`OutlookMailerError::SendMailRequest`] error if sending the mailing request to the
    /// Microsoft Graph API fails.
    ///
    /// Returns a boxed, type-erased  [`OutlookMailerError::SendMailResponse`] error if the Microsoft Graph API responds
    /// with a non-success HTTP status code.
    ///
    /// Returns a boxed, type-erased [`OutlookMailerError::SendMailResponseBody`] error if the Microsoft Graph API reponse body
    /// cannot be received.
    /// (Crate feature `tracing` only: The response body is only received for logging.)
    #[cfg_attr(feature = "tracing", instrument(skip(message)))]
    async fn send_mail(&self, message: Message<'_>) -> Result<(), DynMailerError> {
        Mailer::send_mail(self, message).await.map_err(Into::into)
    }
}

/// The Microsoft Identity Service access token request JSON success response.
#[derive(Debug, Deserialize)]
struct TokenResponse {
    // token_type: String,
    // expires_in: i32,
    // ext_expires_in: i32,
    access_token: String,
}
