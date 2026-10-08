use crate::{
    ApiError, ApiResponse, Config, Error, RequestOptions, Result, WebSocketConnection, auth::Auth,
    config::validate_limits, error::invalid,
};
use reqwest::{
    Body, Method, Url,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::Serialize;
use serde_json::Value;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}
pub(crate) struct Inner {
    pub config: Config,
    http: reqwest::Client,
    auth: Auth,
    token_url: String,
}
impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish_non_exhaustive()
    }
}
impl Client {
    /// Create a pooled client. Redirects and implicit proxy settings are disabled.
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        let token_url = config
            .token_url
            .clone()
            .unwrap_or(config.endpoint("iam", "https://iam.basaltic.sh")? + "/v1/oauth/token");
        let http = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .build()?;
        let auth = Auth::new(config.credentials.clone())?;
        Ok(Self {
            inner: Arc::new(Inner {
                config,
                http,
                auth,
                token_url,
            }),
        })
    }
    pub fn from_env() -> Result<Self> {
        Self::new(Config::from_env()?)
    }
}

pub(crate) struct QueryEncoding {
    pub name: &'static str,
    pub style: &'static str,
    pub explode: bool,
}
pub(crate) struct Operation {
    pub id: &'static str,
    pub method: &'static str,
    pub path: &'static str,
    pub authenticated: bool,
    pub required_query: &'static [&'static str],
    pub required_headers: &'static [&'static str],
    pub content_type: &'static str,
    pub accept: &'static str,
    pub query_encoding: &'static [QueryEncoding],
}
pub(crate) enum Payload {
    Empty,
    Bytes(Vec<u8>),
    Stream(Option<Body>),
}
impl Payload {
    pub fn json<T: Serialize>(body: &T) -> Result<Self> {
        Ok(Self::Bytes(serde_json::to_vec(body)?))
    }
    pub fn form<T: Serialize>(body: &T) -> Result<Self> {
        Ok(Self::Bytes(
            encode_query(&serde_json::to_value(body)?, &[])?.into_bytes(),
        ))
    }
    pub fn binary(body: Body) -> Self {
        if let Some(bytes) = body.as_bytes() {
            Self::Bytes(bytes.to_vec())
        } else {
            Self::Stream(Some(body))
        }
    }
    fn try_clone(&self) -> Result<Self> {
        match self {
            Self::Empty => Ok(Self::Empty),
            Self::Bytes(v) => Ok(Self::Bytes(v.clone())),
            Self::Stream(_) => Err(invalid("Streaming bodies cannot be replayed")),
        }
    }
}

pub(crate) struct Core {
    pub client: Client,
    pub service: &'static str,
    pub endpoint: &'static str,
    pub op: &'static Operation,
    pub path: Vec<(String, String)>,
    pub body: Payload,
    pub query: Value,
    pub options: RequestOptions,
}
impl Core {
    pub fn new(
        client: Client,
        service: &'static str,
        endpoint: &'static str,
        op: &'static Operation,
        path: &[(&str, &str)],
        body: Result<Payload>,
        query: Result<Value>,
    ) -> Result<Self> {
        Ok(Self {
            client,
            service,
            endpoint,
            op,
            path: path
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            body: body?,
            query: query?,
            options: RequestOptions::default(),
        })
    }
    pub fn try_clone(&self) -> Result<Self> {
        Ok(Self {
            client: self.client.clone(),
            service: self.service,
            endpoint: self.endpoint,
            op: self.op,
            path: self.path.clone(),
            body: self.body.try_clone()?,
            query: self.query.clone(),
            options: self.options.clone(),
        })
    }
    fn prepare(&self) -> Result<(Url, HeaderMap, Duration, u8)> {
        let config = &self.client.inner.config;
        let options = &self.options;
        let timeout = options.timeout.unwrap_or(config.timeout);
        let attempts = if matches!(self.body, Payload::Stream(_)) {
            1
        } else {
            options.max_attempts.unwrap_or(config.max_attempts)
        };
        validate_limits(timeout, options.max_attempts.unwrap_or(config.max_attempts))?;
        let mut route = self.op.path.to_owned();
        for (key, value) in &self.path {
            if value.is_empty() || value == "." || value == ".." {
                return Err(invalid(
                    "Path parameters must be nonempty and cannot be dot segments",
                ));
            }
            route = route.replace(&format!("{{{key}}}"), &percent(value));
        }
        if route.contains('{') {
            return Err(invalid("Missing path parameter"));
        }
        for key in self.op.required_query {
            if self.query.get(*key).is_none_or(Value::is_null) {
                return Err(invalid("Missing required query parameter"));
            }
        }
        let query = encode_query(&self.query, self.op.query_encoding)?;
        let url = Url::parse(
            &(config.endpoint(self.service, self.endpoint)?
                + &route
                + if query.is_empty() {
                    String::new()
                } else {
                    "?".to_string() + &query
                }
                .as_str()),
        )
        .map_err(|_| invalid("Invalid request URL"))?;
        let mut headers = HeaderMap::new();
        header(&mut headers, "accept", self.op.accept)?;
        header(
            &mut headers,
            "user-agent",
            concat!("basaltic-rust/", env!("CARGO_PKG_VERSION")),
        )?;
        if !matches!(self.body, Payload::Empty) && !self.op.content_type.is_empty() {
            header(&mut headers, "content-type", self.op.content_type)?;
        }
        let account = options.account_id.as_ref().unwrap_or(&config.account_id);
        if !account.is_empty() {
            header(&mut headers, "x-account-id", account)?;
        }
        if let Some(key) = &options.idempotency_key {
            if key.trim().is_empty() {
                return Err(invalid("Idempotency key cannot be empty"));
            }
            header(&mut headers, "idempotency-key", key)?;
        }
        for (key, value) in &options.headers {
            if matches!(
                key.as_str(),
                "authorization"
                    | "host"
                    | "cookie"
                    | "content-length"
                    | "content-type"
                    | "accept"
                    | "user-agent"
                    | "x-account-id"
                    | "idempotency-key"
                    | "transfer-encoding"
            ) {
                return Err(invalid("Cannot override a managed HTTP header"));
            }
            if value.as_bytes().iter().any(|b| b.is_ascii_control()) {
                return Err(invalid("HTTP header contains a control character"));
            }
            headers.insert(key.clone(), value.clone());
        }
        for key in self.op.required_headers {
            if headers.get(*key).is_none_or(|v| v.is_empty()) {
                return Err(invalid("Missing required header"));
            }
        }
        Ok((url, headers, timeout, attempts))
    }
    async fn token(&self) -> Result<String> {
        if !self.op.authenticated {
            return Ok(String::new());
        }
        let inner = &self.client.inner;
        inner
            .auth
            .token(
                &inner.http,
                &inner.token_url,
                self.options.timeout.unwrap_or(inner.config.timeout),
            )
            .await
    }
    pub async fn websocket(&self) -> Result<WebSocketConnection> {
        let (mut url, mut headers, _, _) = self.prepare()?;
        let scheme = if url.scheme() == "https" { "wss" } else { "ws" };
        url.set_scheme(scheme)
            .map_err(|_| invalid("Invalid websocket URL"))?;
        let token = self.token().await?;
        if !token.is_empty() {
            header(&mut headers, "authorization", &format!("Bearer {token}"))?;
        }
        Ok(WebSocketConnection {
            url: url.into(),
            headers,
        })
    }
    pub async fn send(mut self) -> Result<reqwest::Response> {
        let (url, headers, timeout, attempts) = self.prepare()?;
        let repeatable = matches!(
            self.op.method,
            "GET" | "HEAD" | "OPTIONS" | "PUT" | "DELETE"
        ) || self.options.idempotency_key.is_some();
        let method = Method::from_bytes(self.op.method.as_bytes())
            .map_err(|_| invalid("Invalid operation method"))?;
        let mut refreshed = false;
        for attempt in 1..=attempts {
            let token = self.token().await?;
            let inner = &self.client.inner;
            let mut request = inner
                .http
                .request(method.clone(), url.clone())
                .headers(headers.clone())
                .timeout(timeout);
            if !token.is_empty() {
                request = request.bearer_auth(&token);
            }
            request = match &mut self.body {
                Payload::Empty => request,
                Payload::Bytes(v) => request.body(v.clone()),
                Payload::Stream(body) => request.body(
                    body.take()
                        .ok_or_else(|| invalid("Streaming body was already consumed"))?,
                ),
            };
            match request.send().await {
                Ok(response) if response.status().is_success() => return Ok(response),
                Ok(response) => {
                    let status = response.status().as_u16();
                    if status == 401
                        && self.op.authenticated
                        && !refreshed
                        && attempt < attempts
                        && inner.auth.refreshable()
                    {
                        drop(response);
                        inner.auth.invalidate(&token).await;
                        refreshed = true;
                        continue;
                    }
                    if repeatable
                        && attempt < attempts
                        && matches!(status, 429 | 500 | 502 | 503 | 504)
                        && let Some(delay) = retry_delay(
                            &inner.config,
                            attempt,
                            response
                                .headers()
                                .get("retry-after")
                                .and_then(|v| v.to_str().ok()),
                        )
                    {
                        drop(response);
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err(api_error(response, self.op.id).await);
                }
                Err(e) => {
                    let failure = Error::from(e);
                    if !repeatable || attempt == attempts {
                        return Err(failure);
                    }
                    tokio::time::sleep(backoff(&inner.config, attempt)).await;
                }
            }
        }
        Err(Error::Transport)
    }
}

fn header(headers: &mut HeaderMap, key: &str, value: &str) -> Result<()> {
    if value.bytes().any(|b| b.is_ascii_control()) {
        return Err(invalid("HTTP header contains a control character"));
    }
    let name =
        HeaderName::from_bytes(key.as_bytes()).map_err(|_| invalid("Invalid HTTP header name"))?;
    let mut value =
        HeaderValue::from_str(value).map_err(|_| invalid("Invalid HTTP header value"))?;
    if name == "authorization" {
        value.set_sensitive(true);
    }
    headers.insert(name, value);
    Ok(())
}
fn percent(value: &str) -> String {
    let mut encoded = String::new();
    for b in value.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            encoded.push(b as char);
        } else {
            use std::fmt::Write;
            let _ = write!(encoded, "%{b:02X}");
        }
    }
    encoded
}
fn scalar(value: &Value) -> Result<String> {
    match value {
        Value::String(v) => Ok(v.clone()),
        Value::Number(v) => Ok(v.to_string()),
        Value::Bool(v) => Ok(v.to_string()),
        _ => Err(invalid("Query values must be scalars or arrays of scalars")),
    }
}
pub(crate) fn encode_query(value: &Value, encodings: &[QueryEncoding]) -> Result<String> {
    if value.is_null() {
        return Ok(String::new());
    }
    let values = value
        .as_object()
        .ok_or_else(|| invalid("Query or form body must be an object"))?;
    let mut pairs = Vec::new();
    for (key, value) in values {
        if value.is_null() {
            continue;
        }
        let prefix = percent(key) + "=";
        if let Some(values) = value.as_array() {
            let values = values.iter().map(scalar).collect::<Result<Vec<_>>>()?;
            let encoding = encodings.iter().find(|e| e.name == key);
            if encoding.is_none_or(|e| e.explode) {
                pairs.extend(values.iter().map(|v| prefix.clone() + &percent(v)));
            } else {
                let separator = match encoding.map(|e| e.style) {
                    Some("spaceDelimited") => " ",
                    Some("pipeDelimited") => "|",
                    _ => ",",
                };
                pairs.push(prefix + &percent(&values.join(separator)));
            }
        } else {
            pairs.push(prefix + &percent(&scalar(value)?));
        }
    }
    Ok(pairs.join("&"))
}
fn backoff(config: &Config, attempt: u8) -> Duration {
    config
        .base_delay
        .saturating_mul(1u32 << (attempt - 1))
        .min(config.max_delay)
        .mul_f64(fastrand::f64())
}
fn retry_delay(config: &Config, attempt: u8, value: Option<&str>) -> Option<Duration> {
    let Some(value) = value else {
        return Some(backoff(config, attempt));
    };
    let parsed = if value.bytes().all(|b| b.is_ascii_digit()) && !value.is_empty() {
        Some(Duration::from_secs(value.parse::<u64>().ok()?))
    } else {
        httpdate::parse_http_date(value)
            .ok()
            .map(|date| date.duration_since(SystemTime::now()).unwrap_or_default())
    };
    match parsed {
        Some(delay) if delay > config.max_delay => None,
        Some(delay) => Some(delay),
        None => Some(backoff(config, attempt)),
    }
}
async fn api_error(mut response: reqwest::Response, operation: &str) -> Error {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let mut raw = Vec::new();
    while let Ok(Some(chunk)) = response.chunk().await {
        let count = chunk.len().min(1_048_576 - raw.len());
        raw.extend_from_slice(&chunk[..count]);
        if raw.len() == 1_048_576 {
            break;
        }
    }
    let data: Value = serde_json::from_slice(&raw).unwrap_or(Value::Null);
    let get = |key: &str| {
        data.get("error")
            .and_then(|v| v.get(key))
            .and_then(Value::as_str)
    };
    Error::Api(Box::new(ApiError {
        status,
        code: get("code").unwrap_or("").into(),
        message: get("message").unwrap_or("Unexpected API response").into(),
        request_id: get("request_id")
            .unwrap_or_else(|| {
                headers
                    .get("x-request-id")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
            })
            .into(),
        operation: operation.into(),
        headers,
    }))
}
pub(crate) async fn json_response(response: reqwest::Response) -> Result<ApiResponse<Value>> {
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let bytes = response.bytes().await?;
    let data: Value = if bytes.iter().all(u8::is_ascii_whitespace) {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&bytes)?
    };
    if !data.is_object() && !data.is_array() {
        return Err(Error::Protocol("Expected a JSON object or array".into()));
    }
    Ok(ApiResponse {
        data,
        status,
        headers,
    })
}
