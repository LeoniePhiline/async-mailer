# async-mailer
A set of async generic `Mailer` and dynamic `dyn DynMailer` traits with runtime-pluggable Outlook (Office365) and SMTP implementations.

## Installation

Add to your `Cargo.toml`:

```toml
async-mailer = "0.3.0"
```

By default, features `smtp`, `outlook` and `tracing` are enabled.
Use `default-features = false` and `features = [...]` to select features individually.

## Example:

```rust
// Use `new` for a strongly typed mailer instance,
// or `new_box` / `new_arc` for a type-erased dynamic mailer.

// Create a `BoxMailer` - alias for `Box<dyn DynMailer>`.
let mailer: BoxMailer = OutlookMailer::new_box(
    "<Microsoft Identity service tenant>",
    "<OAuth2 app GUID>",
    "<OAuth2 app secret>"
).await?;

// Alternative implementations can be used.

// Alternative:
let mailer: BoxMailer = SmtpMailer::new_box(
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

// Send the message using the implementation-agnostic `dyn DynMailer`.
mailer.send_mail(&message).await?;
```

## Roadmap

- DKIM support is planned to be implemented on the `SmtpMailer`.
- Access token auto-refresh is planned to be implemented on the `OutlookMailer`.

Further mailer implementations are possible. Please open an issue and ideally provide a pull request to add your alternative mailer implementation!
