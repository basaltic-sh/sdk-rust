use crate::{Error, Result, error::invalid};
use std::{
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

pub type TokenFuture<'a> = Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;

/// Supply short-lived credentials from another identity source.
pub trait TokenProvider: Send + Sync {
    fn token(&self) -> TokenFuture<'_>;
    fn invalidate<'a>(
        &'a self,
        _rejected: &'a str,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async {})
    }
}

#[derive(Clone)]
pub enum Credentials {
    /// Rejected by Client::new; credentials must be explicit.
    Missing,
    /// Allows only operations marked as unauthenticated by the API specification.
    Anonymous,
    #[doc(hidden)]
    Bearer(String),
    #[doc(hidden)]
    AccessKey {
        id: String,
        secret: String,
    },
    Provider(Arc<dyn TokenProvider>),
}
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Missing => "Missing",
            Self::Anonymous => "Anonymous",
            _ => "Credentials([REDACTED])",
        })
    }
}
impl Credentials {
    pub fn bearer(token: impl Into<String>) -> Result<Self> {
        let token = token.into();
        validate_token(&token)?;
        Ok(Self::Bearer(token))
    }
    pub fn access_key(id: impl Into<String>, secret: impl Into<String>) -> Result<Self> {
        let (id, secret) = (id.into(), secret.into());
        if id.is_empty() || id.contains(':') || secret.is_empty() {
            return Err(invalid("A valid access key ID and secret are required"));
        }
        Ok(Self::AccessKey { id, secret })
    }
}
pub(crate) fn validate_token(value: &str) -> Result<()> {
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_graphic()) {
        return Err(invalid(
            "Bearer tokens must be nonempty ASCII without whitespace",
        ));
    }
    Ok(())
}

pub(crate) struct Auth {
    credentials: Credentials,
    cache: Mutex<Option<(String, Instant)>>,
}
impl Auth {
    pub fn new(credentials: Credentials) -> Result<Self> {
        match &credentials {
            Credentials::Bearer(t) => validate_token(t)?,
            Credentials::AccessKey { id, secret }
                if id.is_empty() || id.contains(':') || secret.is_empty() =>
            {
                return Err(invalid("A valid access key pair is required"));
            }
            _ => (),
        }
        Ok(Self {
            credentials,
            cache: Mutex::new(None),
        })
    }
    pub async fn token(
        &self,
        client: &reqwest::Client,
        url: &str,
        timeout: Duration,
    ) -> Result<String> {
        match &self.credentials {
            Credentials::Missing | Credentials::Anonymous => Err(invalid(
                "Anonymous access cannot call an authenticated operation",
            )),
            Credentials::Bearer(t) => Ok(t.clone()),
            Credentials::Provider(provider) => {
                let token = provider.token().await?;
                validate_token(&token)?;
                Ok(token)
            }
            Credentials::AccessKey { id, secret } => {
                // Holding this async lock gives one refresh at a time. Cancellation
                // releases it without installing a partial credential.
                let mut cache = self.cache.lock().await;
                if let Some((token, until)) = &*cache
                    && Instant::now() < *until
                {
                    return Ok(token.clone());
                }
                let started = Instant::now();
                let mut response = client
                    .post(url)
                    .basic_auth(id, Some(secret))
                    .header("accept", "application/json")
                    .form(&[("grant_type", "client_credentials")])
                    .timeout(timeout)
                    .send()
                    .await
                    .map_err(|_| Error::Authentication { status: None })?;
                let status = response.status().as_u16();
                let fail = || Error::Authentication {
                    status: Some(status),
                };
                if status != 200 {
                    return Err(fail());
                }
                let mut raw = Vec::new();
                while let Some(chunk) = response.chunk().await.map_err(|_| fail())? {
                    if raw.len() + chunk.len() > 1_048_576 {
                        return Err(fail());
                    }
                    raw.extend_from_slice(&chunk);
                }
                let data: serde_json::Value = serde_json::from_slice(&raw).map_err(|_| fail())?;
                let token = data
                    .get("access_token")
                    .and_then(|v| v.as_str())
                    .ok_or_else(fail)?;
                validate_token(token).map_err(|_| fail())?;
                if !data
                    .get("token_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Bearer")
                    .eq_ignore_ascii_case("bearer")
                {
                    return Err(fail());
                }
                let seconds = data
                    .get("expires_in")
                    .map(|v| v.as_f64())
                    .unwrap_or(Some(900.0))
                    .ok_or_else(fail)?;
                if !seconds.is_finite() || seconds <= 0.0 {
                    return Err(fail());
                }
                let ttl = Duration::try_from_secs_f64(seconds - (seconds * 0.1).min(300.0))
                    .map_err(|_| fail())?;
                let until = started.checked_add(ttl).ok_or_else(fail)?;
                *cache = Some((token.to_owned(), until));
                Ok(token.to_owned())
            }
        }
    }
    pub fn refreshable(&self) -> bool {
        matches!(
            self.credentials,
            Credentials::AccessKey { .. } | Credentials::Provider(_)
        )
    }
    pub async fn invalidate(&self, rejected: &str) {
        match &self.credentials {
            Credentials::Provider(p) => p.invalidate(rejected).await,
            Credentials::AccessKey { .. } => {
                let mut cache = self.cache.lock().await;
                if cache.as_ref().is_some_and(|(token, _)| token == rejected) {
                    *cache = None;
                }
            }
            _ => (),
        }
    }
}
