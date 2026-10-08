use reqwest::header::HeaderMap;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// Structured API failure, including its request ID for support.
#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: u16,
    pub code: String,
    pub message: String,
    pub request_id: String,
    pub operation: String,
    pub headers: HeaderMap,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    Configuration(String),
    Protocol(String),
    Authentication { status: Option<u16> },
    Api(Box<ApiError>),
    Transport,
    Timeout,
    AmbiguousReference,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(s) => write!(f, "Invalid configuration: {s}"),
            Self::Protocol(s) => write!(f, "Invalid API response: {s}"),
            Self::Authentication { status } => {
                write!(f, "Authentication failed (status {status:?})")
            }
            Self::Api(e) => write!(
                f,
                "{}: HTTP {} {}: {} (request {})",
                e.operation, e.status, e.code, e.message, e.request_id
            ),
            Self::Transport => f.write_str("HTTP transport failed"),
            Self::Timeout => f.write_str("HTTP request timed out"),
            Self::AmbiguousReference => f.write_str("More than one resource matches the reference"),
        }
    }
}
impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Self::Timeout
        } else {
            Self::Transport
        }
    }
}
impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::Protocol("JSON does not match the expected schema".into())
    }
}

pub(crate) fn invalid(message: &str) -> Error {
    Error::Configuration(message.into())
}
