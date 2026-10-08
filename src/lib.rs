//! Typed asynchronous clients for the Basaltic API.
//!
//! Service methods return request builders. Call `.send().await` to execute them;
//! use `.options(...)` for account selection, idempotency keys and custom headers.
//! See the README and generated API reference for examples.
#![doc = include_str!("../README.md")]

mod auth;
mod config;
mod error;
pub mod models;
mod request;
mod response;
pub mod services;
mod transport;

pub use auth::{Credentials, TokenFuture, TokenProvider};
pub use config::{Config, RequestOptions};
pub use error::{ApiError, Error, Result};
pub use request::{BinaryRequest, EmptyRequest, PagedRequest, Request, WebSocketRequest};
pub use reqwest::{Body, Response as BinaryResponse};
pub use response::{ApiResponse, Nullable, Page, WebSocketConnection};
pub use transport::Client;

/// Generate once per logical mutation and reuse when retrying that mutation.
pub fn new_idempotency_key() -> String {
    uuid::Uuid::new_v4().to_string()
}
