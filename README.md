# async-mailer
An async dyn Mailer trait with runtime-pluggable Outlook (Office365) and SMTP implementations.

## Example:

```rust
// Create a `Box<dyn Mailer>`.
//
// Alternative implementations can be used.

let mailer = OutlookMailer::new(
    "<Microsoft Identity service tenant>",
    "<OAuth2 app GUID>",
    "<OAuth2 app secret>"
).await?;

// Alternative:
let mailer = SmtpMailer::new(
    "smtp.example.com",
    465,
    SmtpInvalidCertsPolicy::Deny,
    "<username>",
    "<password>"
);

// Further alternative mailers can be implemented by third parties.

// The trait object is `Send` and `Sync` and may be stored e.g. as part of your server state.

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
