# Basaltic Rust SDK

[![Checks](https://github.com/basaltic-sh/sdk-rust/actions/workflows/public-checks.yml/badge.svg)](https://github.com/basaltic-sh/sdk-rust/actions/workflows/public-checks.yml)

Official typed, asynchronous Rust client for Basaltic. Covers 401 operations across
15 services: audit, billing, catalog, certificate, compute, DNS, IAM, KMS, load balancer,
network, quota, secrets, storage, telemetry, and workspace.

Requires Rust 1.89 or newer and a Tokio runtime. HTTP connections use reqwest with
Rustls. Clone a client to share its connection pool and cached credentials.

## Install

```toml
[dependencies]
basaltic = { package = "basaltic-sdk-rust", version = "0.3" }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
futures-util = "0.3"
```

## Quick start

Set `BASALTIC_ACCESS_KEY_ID`, `BASALTIC_SECRET_ACCESS_KEY`, and `BASALTIC_REGION` in
your environment, then:

```rust,no_run
use basaltic::{Client, models::compute::ListImagesQuery};
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> basaltic::Result<()> {
    let client = Client::from_env()?;
    let mut images = client.compute().list_images(&ListImagesQuery {
        name: Some("ubuntu".into()),
        ..Default::default()
    }).items();
    while let Some(image) = images.try_next().await? {
        println!("{} {}", image.id, image.name);
    }
    Ok(())
}
```

## Configuration and authentication

`Config::from_env()` reads `BASALTIC_ACCESS_TOKEN` (if present, it takes precedence
over the access-key pair), `BASALTIC_REGION`, `BASALTIC_ACCOUNT_ID`, `BASALTIC_DOMAIN`,
and `BASALTIC_ENDPOINT_URL_<SERVICE>`. Explicit `Config` values do not read the environment.

```rust,no_run
use basaltic::{Client, Config, Credentials};

let client = Client::new(Config {
    credentials: Credentials::access_key("access-key-id", "secret-access-key")?,
    region: "region-name".into(),
    account_id: "account-id".into(),
    ..Config::default()
})?;
# Ok::<(), basaltic::Error>(())
```

Use `Credentials::bearer(token)?` for an existing bearer token, or
`Credentials::Provider(Arc::new(provider))` with your own `TokenProvider` implementation.
Access-key credentials exchange via OAuth client credentials, cache the token, and
refresh it before expiry. Concurrent requests share one refresh; cancellation cannot
leave the cache locked. A rejected refreshable token is invalidated once per request.
`Credentials::Anonymous` permits only API operations declared unauthenticated.

The default timeout is 30 seconds and the maximum is four attempts. Endpoints must
be HTTP(S) URLs without credentials, query strings, or fragments. HTTPS is used by
default. Redirects, ambient proxies and cookie storage are disabled.

## Typed requests and responses

Methods return owned request builders and perform no HTTP request until `.send().await`.
Path parameters are strings, JSON/form bodies use service-specific models, and query
parameters use the operation's `*Query` model. Models with required request fields
provide `::new(...)`; optional fields start omitted. Queries with no required fields
implement `Default`. Field names use Rust snake_case and retain their API names on the wire.

JSON calls return `ApiResponse<T>` containing the typed API envelope, status and
headers; `.request_id()` returns the request ID. String enums retain future values
as `Unknown(String)`. Optional nullable fields use `Option<Nullable<T>>`: `None`
omits a field, `Some(Nullable::Null)` sends JSON null, and `Some(Nullable::Value(x))`
sends a value. Credential and token fields are redacted in model `Debug` output.

Use request options for account overrides, custom headers, timeouts and idempotency:

```rust,no_run
# use basaltic::{Client, RequestOptions};
# async fn example(client: &Client) -> basaltic::Result<()> {
let key = basaltic::new_idempotency_key();
client.compute().start_instance("instance-id").options(RequestOptions {
    idempotency_key: Some(key),
    ..Default::default()
}).send().await?;
# Ok(()) }
```

Reuse the same idempotency key for retries of one logical mutation. Transient failures
retry GET, HEAD, OPTIONS, PUT and DELETE, and other methods only with an idempotency
key. Backoff uses jitter and respects bounded `Retry-After`; a delay above the configured
maximum is returned as an error instead of retried early. Streaming uploads run once.
Managed headers, including Authorization and Idempotency-Key, cannot be overridden
through the custom header map. Supply API-required headers such as If-Match there.

## Pagination and references

Paged requests support `.send().await` for one `Page<T, Item>`, `.pages()` for a lazy
stream of pages, and `.items()` for a lazy stream of resources. Pages retain their
typed response envelope alongside items and pagination metadata. Invalid or repeated
markers produce `Error::Protocol` rather than silently truncating a collection.

Supported resource getters offer `*_by_reference(reference, scope)` for UUIDs, CRNs
or unambiguous names. Scope models retain service-specific filters. Missing resources
return an API error with status 404; multiple matches return `Error::AmbiguousReference`.

## Binary data and WebSockets

Storage uploads accept `basaltic::Body` (buffered bytes or a stream). Binary downloads
and HEAD calls return `BinaryResponse`, exposing headers, `.chunk().await`, `.bytes().await`
and `.bytes_stream()`. Consume or drop the response to release it. Dropping a request
future cancels that request.

WebSocket operations expose `.prepare().await`, returning a URL and authenticated
headers for your chosen WebSocket library. The SDK does not open a WebSocket itself.
Keep those headers private and disable redirects when connecting.

## Errors

Match `Error::Api(error)` for status, API code, message, operation, request ID and
response headers. Authentication, transport, timeout, configuration, protocol and
ambiguous-reference failures have separate variants. Raw HTTP errors and authentication
response bodies are not retained in errors.

See the [API reference](https://github.com/basaltic-sh/sdk-rust/blob/main/docs/api.md) for every operation and the
[Basaltic documentation](https://docs.basaltic.sh) for API concepts.

## Development and security

This repository is a release snapshot for reference. Development happens internally;
external pull requests and contributions are not accepted. Public GitHub Actions run
formatting, lint, tests, documentation and package verification on the released source.
Report vulnerabilities privately to **security@basaltic.sh**; see [SECURITY.md](https://github.com/basaltic-sh/sdk-rust/blob/main/SECURITY.md).

Licensed under Apache-2.0.
