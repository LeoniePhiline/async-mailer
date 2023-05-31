# async-mailer
A set of async generic [`Mailer`][Mailer] and dynamic [`dyn DynMailer`][DynMailer] traits with runtime-pluggable [Microsoft Outlook (Office365)][OutlookMailer] and [SMTP][SmtpMailer] implementations.

[![Crates.io](https://img.shields.io/crates/v/async-mailer)](https://crates.io/crates/async-mailer)
[![Documentation](https://docs.rs/async-mailer/badge.svg)][docs]
[![Dependency status](https://deps.rs/repo/github/LeoniePhiline/async-mailer/status.svg)](https://deps.rs/repo/github/LeoniePhiline/async-mailer)

## Installation

Add to your `Cargo.toml`:

```toml
async-mailer = "0.3.5"
```

You can control the re-exported mailer implementations,
as well as [`tracing`](https://docs.rs/crate/tracing) support,
via [crate feature toggles](https://docs.rs/crate/async-mailer/latest/features).

By default, features `smtp`, `outlook` and `tracing` are enabled.
Use `default-features = false` and `features = [...]` to select features individually.

## Examples:

Use `new` for a strongly typed mailer instance,
or `new_box` / `new_arc` for a type-erased dynamic mailer.

[Microsoft Outlook (Office365)][OutlookMailer] and [SMTP][SmtpMailer] variants are available.

# Using the strongly typed [`Mailer`][Mailer]:

```rust
// Both `OutlookMailer` and `SmtpMailer` implement `Mailer`
// and can be used with `impl Mailer` or `<M: Mailer>` bounds.

use async_mailer::{ IntoMessage, Mailer, OutlookMailer, SmtpMailer };

let mailer: OutlookMailer = OutlookMailer::new(
    "<Microsoft Identity service tenant>".into(),
    "<OAuth2 app GUID>".into(),
    async_mailer::Secret::new("<OAuth2 app secret>".into())
).await?;

// Alternative:

let mailer: SmtpMailer = SmtpMailer::new(
    "smtp.example.com".into(),
    465,
    async_mailer::SmtpInvalidCertsPolicy::Deny,
    "<username>".into(),
    async_mailer::Secret::new("<password>".into())
);

// Further alternative mailers can be implemented by third parties.

// Build a message using the re-exported `mail_builder::MessageBuilder'.
//
// For blazingly fast rendering of beautiful HTML mail,
// I recommend combining `askama` with `mrml`.

let message = async_mailer::MessageBuilder::new()
    .from(("From Name", "from@example.com"))
    .to("to@example.com")
    .subject("Subject")
    .text_body("Mail body")
    .into_message()?;

// Send the message using the strongly typed `Mailer`.

mailer.send_mail(message).await?;
```

# Using the dynamically typed [`dyn DynMailer`][DynMailer] / [`BoxMailer`][BoxMailer] / [`ArcMailer`][ArcMailer]:

```rust
use async_mailer::{ BoxMailer, IntoMessage, OutlookMailer, SmtpMailer };

// Both `OutlookMailer` and `SmtpMailer` implement `DynMailer` and can be used as trait objects.
// Here they are used as `BoxMailer`, which is an alias to `Box<dyn DynMailer>`.

let mailer: BoxMailer = OutlookMailer::new_box( // Or `OutlookMailer::new_arc()`.
    "<Microsoft Identity service tenant>".into(),
    "<OAuth2 app GUID>".into(),
    async_mailer::Secret::new("<OAuth2 app secret>".into())
).await?;

// Alternative:

let mailer: BoxMailer = SmtpMailer::new_box( // Or `SmtpMailer::new_arc()`.
    "smtp.example.com".into(),
    465,
    async_mailer::SmtpInvalidCertsPolicy::Deny,
    "<username>".into(),
    async_mailer::Secret::new("<password>".into())
);

// Further alternative mailers can be implemented by third parties.

// The trait object is `Send` and `Sync` and may be stored e.g. as part of your server state.

// Build a message using the re-exported `mail_builder::MessageBuilder'.
//
// For blazingly fast rendering of beautiful HTML mail,
// I recommend combining `askama` with `mrml`.

let message = async_mailer::MessageBuilder::new()
    .from(("From Name", "from@example.com"))
    .to("to@example.com")
    .subject("Subject")
    .text_body("Mail body")
    .into_message()?;

// Send the message using the implementation-agnostic `dyn DynMailer`.

mailer.send_mail(message).await?;
```

# Feature flags

- `outlook`: Enable [`OutlookMailer`][OutlookMailer].
- `smtp`: Enable [`SmtpMailer`][SmtpMailer].
- `tracing`: Enable debug and error logging using the [`tracing`](https://docs.rs/crate/tracing) crate.
- `clap`: Implement [`clap::ValueEnum`](https://docs.rs/clap/latest/clap/trait.ValueEnum.html)
  for [`SmtpInvalidCertsPolicy`][SmtpInvalidCertsPolicy].
  This allows for easily configured CLI options like `--invalid-certs <allow|deny>`.

Default: `outlook`, `smtp`, `tracing`.

## Roadmap

- DKIM support is planned to be implemented on the [`SmtpMailer`][SmtpMailer].
- Access token auto-refresh is planned to be implemented on the [`OutlookMailer`][OutlookMailer].

Further mailer implementations are possible.
Please open an issue and ideally provide a pull request to add your alternative mailer implementation!

[docs]: https://docs.rs/async-mailer
[Mailer]: https://docs.rs/async-mailer/latest/async_mailer/trait.Mailer.html
[DynMailer]: https://docs.rs/async-mailer/latest/async_mailer/trait.DynMailer.html
[BoxMailer]: https://docs.rs/async-mailer/latest/async_mailer/type.BoxMailer.html
[ArcMailer]: https://docs.rs/async-mailer/latest/async_mailer/type.ArcMailer.html
[OutlookMailer]: https://docs.rs/async-mailer/latest/async_mailer/struct.OutlookMailer.html
[SmtpMailer]: https://docs.rs/async-mailer/latest/async_mailer/struct.SmtpMailer.html
[SmtpInvalidCertsPolicy]: https://docs.rs/async-mailer/latest/async_mailer/enum.SmtpInvalidCertsPolicy.html
