use crate::{Credentials, Result, error::invalid};
use reqwest::{Url, header::HeaderMap};
use std::{collections::BTreeMap, time::Duration};

/// Explicit configuration. Environment variables are read only by `from_env`.
#[derive(Clone, Debug)]
pub struct Config {
    pub credentials: Credentials,
    pub region: String,
    pub account_id: String,
    pub domain: String,
    pub endpoints: BTreeMap<String, String>,
    pub token_url: Option<String>,
    pub timeout: Duration,
    pub max_attempts: u8,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            credentials: Credentials::Missing,
            region: String::new(),
            account_id: String::new(),
            domain: "basaltic.sh".into(),
            endpoints: BTreeMap::new(),
            token_url: None,
            timeout: Duration::from_secs(30),
            max_attempts: 4,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(20),
        }
    }
}

impl Config {
    /// Read BASALTIC_ACCESS_TOKEN or the access-key pair, region, account, domain,
    /// and `BASALTIC_ENDPOINT_URL_<SERVICE>` from the process environment.
    pub fn from_env() -> Result<Self> {
        let env = |key: &str| std::env::var(key).unwrap_or_default();
        let token = env("BASALTIC_ACCESS_TOKEN");
        let key = env("BASALTIC_ACCESS_KEY_ID");
        let secret = env("BASALTIC_SECRET_ACCESS_KEY");
        let credentials = if !token.is_empty() {
            Credentials::bearer(token)?
        } else if !key.is_empty() || !secret.is_empty() {
            Credentials::access_key(key, secret)?
        } else {
            Credentials::Missing
        };
        let domain = env("BASALTIC_DOMAIN");
        let value = Self {
            credentials,
            region: env("BASALTIC_REGION"),
            account_id: env("BASALTIC_ACCOUNT_ID"),
            domain: if domain.is_empty() {
                "basaltic.sh".into()
            } else {
                domain
            },
            endpoints: std::env::vars()
                .filter_map(|(k, v)| {
                    k.strip_prefix("BASALTIC_ENDPOINT_URL_")
                        .filter(|_| !v.is_empty())
                        .map(|s| (s.to_ascii_lowercase(), v))
                })
                .collect(),
            ..Self::default()
        };
        value.validate()?;
        Ok(value)
    }

    pub(crate) fn validate(&self) -> Result<()> {
        if matches!(self.credentials, Credentials::Missing) {
            return Err(invalid("Provide credentials or explicit anonymous access"));
        }
        if self.domain.is_empty()
            || !self.domain.split('.').all(|s| {
                !s.is_empty()
                    && !s.starts_with('-')
                    && !s.ends_with('-')
                    && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            })
        {
            return Err(invalid("Invalid API domain"));
        }
        validate_limits(self.timeout, self.max_attempts)?;
        for endpoint in self.endpoints.values() {
            validate_url(endpoint)?;
        }
        if let Some(url) = &self.token_url {
            validate_url(url)?;
        }
        Ok(())
    }

    pub(crate) fn endpoint(&self, service: &str, template: &str) -> Result<String> {
        if let Some(url) = self.endpoints.get(service) {
            return Ok(url.trim_end_matches('/').to_string());
        }
        if template.contains("{region}")
            && (self.region.is_empty()
                || !self.region.split('-').all(|p| {
                    !p.is_empty()
                        && p.bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
                }))
        {
            return Err(invalid("A valid region is required for this service"));
        }
        let url = template
            .replace("basaltic.sh", &self.domain)
            .replace("{region}", &self.region);
        validate_url(&url)?;
        Ok(url.trim_end_matches('/').to_string())
    }
}

#[derive(Clone, Debug, Default)]
pub struct RequestOptions {
    pub account_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub headers: HeaderMap,
    pub timeout: Option<Duration>,
    pub max_attempts: Option<u8>,
}

pub(crate) fn validate_limits(timeout: Duration, attempts: u8) -> Result<()> {
    if timeout.is_zero() || !(1..=10).contains(&attempts) {
        return Err(invalid(
            "Timeout must be positive and max_attempts must be between 1 and 10",
        ));
    }
    Ok(())
}

pub(crate) fn validate_url(value: &str) -> Result<Url> {
    let url = Url::parse(value).map_err(|_| invalid("Endpoint must be an absolute HTTP(S) URL"))?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.port() == Some(0)
        || value.chars().any(char::is_whitespace)
        || value.chars().any(char::is_control)
    {
        return Err(invalid(
            "Endpoint must not contain credentials, a query, fragment or whitespace",
        ));
    }
    Ok(url)
}
