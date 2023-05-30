# async-mailer
A set of async generic `Mailer` and dynamic `dyn DynMailer` traits with runtime-pluggable Outlook (Office365) and SMTP implementations.

## Installation

Add to your `Cargo.toml`:

```toml
async-mailer = "0.3.1"
```

By default, features `smtp`, `outlook` and `tracing` are enabled.
Use `default-features = false` and `features = [...]` to select features individually.

## Examples:

Use `new` for a strongly typed mailer instance,
or `new_box` / `new_arc` for a type-erased dynamic mailer.

Microsoft Outlook and SMTP mailer variants are available.

# Using the strongly typed `async_mailer::Mailer`:

```rust
// Create an `impl Mailer`.
//
// Alternative implementations can be used.

let mailer = async_mailer::OutlookMailer::new(
    "<Microsoft Identity service tenant>".into(),
    "<OAuth2 app GUID>".into(),
    async_mailer::Secret::new("<OAuth2 app secret>".into())
).await?;

// Alternative:
let mailer = async_mailer::SmtpMailer::new(
    "smtp.example.com".into(),
    465,
    async_mailer::SmtpInvalidCertsPolicy::Deny,
    "<username>".into(),
    async_mailer::Secret::new("<password>".into())
);

// Further alternative mailers can be implemented by third parties.

// Build a message using the re-exported `mail_builder::MessageBuilder'.
// For blazingly fast rendering of beautiful HTML mail, I recommend combining `askama` with `mrml`.
use async_mailer::IntoMessage;
let message = async_mailer::MessageBuilder::new()
    .from(("From Name", "from@example.com"))
    .to("to@example.com")
    .subject("Subject")
    .text_body("Mail body")
    .into_message()?;

// Send the message using the strongly typed `Mailer`.
use async_mailer::Mailer;
mailer.send_mail(message).await?;
```

# Using the dynamically typed `async_mailer::DynMailer`:

```rust
// Create a `BoxMailer`.
//
// Alternative implementations can be used.

use async_mailer::BoxMailer;
let mailer: BoxMailer = async_mailer::OutlookMailer::new_box( // Or `new_arc` to use in e.g. globally shared server state.
    "<Microsoft Identity service tenant>".into(),
    "<OAuth2 app GUID>".into(),
    async_mailer::Secret::new("<OAuth2 app secret>".into())
).await?;

// Alternative:
let mailer: BoxMailer = async_mailer::SmtpMailer::new_box( // Or `new_arc` to use in e.g. globally shared server state.
    "smtp.example.com".into(),
    465,
    async_mailer::SmtpInvalidCertsPolicy::Deny,
    "<username>".into(),
    async_mailer::Secret::new("<password>".into())
);

// Further alternative mailers can be implemented by third parties.

// The trait object is `Send` and `Sync` and may be stored e.g. as part of your server state.

// Build a message using the re-exported `mail_builder::MessageBuilder'.
// For blazingly fast rendering of beautiful HTML mail, I recommend combining `askama` with `mrml`.
use async_mailer::IntoMessage;
let message = async_mailer::MessageBuilder::new()
    .from(("From Name", "from@example.com"))
    .to("to@example.com")
    .subject("Subject")
    .text_body("Mail body")
    .into_message()?;

// Send the message using the implementation-agnostic `dyn DynMailer`.
mailer.send_mail(message).await?;
```

## Roadmap

- DKIM support is planned to be implemented on the `SmtpMailer`.
- Access token auto-refresh is planned to be implemented on the `OutlookMailer`.

Further mailer implementations are possible. Please open an issue and ideally provide a pull request to add your alternative mailer implementation!
