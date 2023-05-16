# async-mailer
An async dyn Mailer trait with runtime-pluggable Outlook (Office365) and SMTP implementations.

## Example:

```rust
// Outlook configuration, e.g. from command line arguments or environment variables.
let mailer_configuration = MailerConfiguration::Outlook {
    tenant: "<Microsoft Identity service tenant>",
    app_guid: "<OAuth2 app GUID>",
    secret: "<OAuth2 app secret>"
};

// Alternative: SMTP configuration, e.g. from command line arguments or environment variables.
let mailer_configuration = MailerConfiguration::Outlook {
    host: "smtp.example.com",
    port: 465,
    invalid_certs: SmtpInvalidCertsPolicy::Deny,
    user: "<username>",
    password: "<password>"
};

// Create a `Box<dyn Mailer>`.
// The implementation is `Send` and `Sync` and may be store e.g. as part of your server state.
let mailer = new_mailer(mailer_configuration).await?;

// Build a message using the re-exported `mail_builder::MessageBuilder'.
// For blazingly fast rendering of beautiful HTML mail, I recommend combining `askama` with `mrml`.
let message = MessageBuilder::new()
    .from(("From Name", "from@example.com"))
    .to("to@example.com")
    .subject("Subject")
    .text_body("Mail body");

// Send the message using the implementation-agnostic `dyn Mailer`.
mailer.send_mail(&message).await?;
```

## Roadmap

DKIM support is planned to be implemented on the `SmtpMailer`.

Further mailer implementations are possible. Please open an issue and ideally provide a pull request to add your alternative mailer implementation!

