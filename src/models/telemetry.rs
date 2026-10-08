//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateLogGroupRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// 1..3650, or omit for never expire
    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub retention_days: Option<crate::Nullable<i64>>,
    /// KMS key UUID, CRN or exact name in the authenticated account and serving region. The resolved UUID is pinned; deleting a key and reusing its name never retargets existing data. An empty string selects plaintext.
    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl CreateLogGroupRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            retention_days: None,
            kms_key: None,
            tags: None,
        }
    }
}

pub type CreateLogGroupBody = CreateLogGroupRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LogGroupResponse {
    #[serde(rename = "log_group", default, skip_serializing_if = "Option::is_none")]
    pub log_group: Option<LogGroup>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LogGroup {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// CRN of the log group (used as the IAM resource ARN)
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Owning account — the tenant fence for this group
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,
    /// 1..512 chars, \[A-Za-z0-9_./#-\]. Leading "/" not allowed. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// 1..3650 days, or null for never expire
    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub retention_days: Option<crate::Nullable<i64>>,
    /// True when the bound key has been deleted. The CRN is omitted, but the binding remains encrypted under its original key identity; this does not select platform encryption or plaintext.
    #[serde(
        rename = "kms_key_unavailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_key_unavailable: Option<bool>,
    /// KMS key CRN. Omitted for plaintext or when kms_key_unavailable is true.
    #[serde(
        rename = "kms_key_crn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_key_crn: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,

    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

pub type CreateLogGroupResponse = LogGroupResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LogResponse {
    #[serde(rename = "log", default, skip_serializing_if = "Option::is_none")]
    pub log: Option<LogRecord>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LogRecord {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Producer-stamped event time
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(
        rename = "organization_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_id: Option<String>,
    /// Owning account, empty for org-scoped events
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,

    #[serde(
        rename = "log_group_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_group_id: Option<String>,
    /// Name of the parent log group
    #[serde(rename = "log_group", default, skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
    /// Within-group stream name (typically the producer host)
    #[serde(
        rename = "log_stream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_stream: Option<String>,

    #[serde(rename = "severity", default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<LogRecordSeverity>,
    /// OTel numeric severity band (1..24)
    #[serde(
        rename = "severity_number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub severity_number: Option<i64>,

    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "resource", default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "trace_id", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(rename = "span_id", default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogRecordSeverity {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
    Unknown(String),
}
impl LogRecordSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TRACE => "TRACE",
            Self::DEBUG => "DEBUG",
            Self::INFO => "INFO",
            Self::WARN => "WARN",
            Self::ERROR => "ERROR",
            Self::FATAL => "FATAL",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LogRecordSeverity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LogRecordSeverity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "TRACE" => Self::TRACE,
            "DEBUG" => Self::DEBUG,
            "INFO" => Self::INFO,
            "WARN" => Self::WARN,
            "ERROR" => Self::ERROR,
            "FATAL" => Self::FATAL,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetLogResponse = LogResponse;

pub type GetLogGroupResponse = LogGroupResponse;

pub type GetLogGroupResource = LogGroup;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetLogGroupScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetRetainedTelemetryPresenceResult {
    #[serde(rename = "has_resources")]
    pub has_resources: bool,
}

pub type GetRetainedTelemetryPresenceResponse = GetRetainedTelemetryPresenceResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TraceResponse {
    #[serde(rename = "trace_id", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(rename = "spans", default, skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<Span>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Span {
    #[serde(rename = "trace_id", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(rename = "span_id", default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,

    #[serde(
        rename = "parent_span_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_span_id: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<SpanKind>,

    #[serde(
        rename = "service_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_name: Option<String>,

    #[serde(
        rename = "start_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time: Option<String>,

    #[serde(rename = "end_time", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    #[serde(
        rename = "duration_ms",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_ms: Option<f64>,

    #[serde(
        rename = "status_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_code: Option<SpanStatusCode>,

    #[serde(
        rename = "status_message",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "resource", default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<SpanEvent>>,

    #[serde(rename = "links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SpanLink>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpanKind {
    INTERNAL,
    SERVER,
    CLIENT,
    PRODUCER,
    CONSUMER,
    Unknown(String),
}
impl SpanKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::INTERNAL => "INTERNAL",
            Self::SERVER => "SERVER",
            Self::CLIENT => "CLIENT",
            Self::PRODUCER => "PRODUCER",
            Self::CONSUMER => "CONSUMER",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SpanKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SpanKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "INTERNAL" => Self::INTERNAL,
            "SERVER" => Self::SERVER,
            "CLIENT" => Self::CLIENT,
            "PRODUCER" => Self::PRODUCER,
            "CONSUMER" => Self::CONSUMER,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpanStatusCode {
    UNSET,
    OK,
    ERROR,
    Unknown(String),
}
impl SpanStatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::UNSET => "UNSET",
            Self::OK => "OK",
            Self::ERROR => "ERROR",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SpanStatusCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SpanStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "UNSET" => Self::UNSET,
            "OK" => Self::OK,
            "ERROR" => Self::ERROR,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SpanEvent {
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SpanLink {
    #[serde(rename = "trace_id")]
    pub trace_id: String,

    #[serde(rename = "span_id")]
    pub span_id: String,
}

pub type GetTraceResponse = TraceResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TraceSettingsResponse {
    #[serde(
        rename = "trace_settings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trace_settings: Option<TraceSettings>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TraceSettings {
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,
    /// 1..3650, or null for never-expire
    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub retention_days: Option<crate::Nullable<i64>>,
    /// True when the bound key has been deleted. The CRN is omitted, but the binding remains encrypted under its original key identity; this does not select platform encryption or plaintext.
    #[serde(
        rename = "kms_key_unavailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_key_unavailable: Option<bool>,
    /// KMS key CRN. Omitted for plaintext or when kms_key_unavailable is true.
    #[serde(
        rename = "kms_key_crn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_key_crn: Option<String>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,

    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

pub type GetTraceSettingsResponse = TraceSettingsResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IngestRequestInput {
    #[serde(rename = "logs")]
    pub logs: Vec<IngestRecordInput>,
}
impl IngestRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(logs: Vec<IngestRecordInput>) -> Self {
        Self { logs }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IngestRecordInput {
    /// Optional — defaults to ingest time
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Reference to an existing log group by UUID, CRN or exact name in the authenticated account. CRNs must identify telemetry/log-group in the serving region and account handle. Classification is by syntax with no lookup fallback; slash-bearing names are preserved.
    #[serde(rename = "log_group")]
    pub log_group: String,
    /// Within-group stream name (free-form, customer choice)
    #[serde(rename = "log_stream")]
    pub log_stream: String,
    /// One of TRACE/DEBUG/INFO/WARN/ERROR/FATAL (defaults to INFO)
    #[serde(rename = "severity", default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,

    #[serde(rename = "body")]
    pub body: String,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "resource", default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "trace_id", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(rename = "span_id", default, skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}
impl IngestRecordInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(log_group: String, log_stream: String, body: String) -> Self {
        Self {
            timestamp: None,
            log_group,
            log_stream,
            severity: None,
            body,
            attributes: None,
            resource: None,
            trace_id: None,
            span_id: None,
        }
    }
}

pub type IngestLogsBody = IngestRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct IngestResult {
    #[serde(rename = "accepted", default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<i64>,

    #[serde(rename = "rejected", default, skip_serializing_if = "Option::is_none")]
    pub rejected: Option<i64>,

    #[serde(rename = "errors", default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

pub type IngestLogsResponse = IngestResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IngestSpansRequestInput {
    #[serde(rename = "spans")]
    pub spans: Vec<SpanIngestRecordInput>,
}
impl IngestSpansRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(spans: Vec<SpanIngestRecordInput>) -> Self {
        Self { spans }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SpanIngestRecordInput {
    /// 32 lower-hex chars (OTel TraceId)
    #[serde(rename = "trace_id")]
    pub trace_id: String,
    /// 16 lower-hex chars (OTel SpanId)
    #[serde(rename = "span_id")]
    pub span_id: String,
    /// 16 lower-hex chars or empty for root
    #[serde(
        rename = "parent_span_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_span_id: Option<String>,
    /// Operation name (e.g. "GET /api/users")
    #[serde(rename = "name")]
    pub name: String,
    /// Defaults to INTERNAL
    #[serde(rename = "kind", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<SpanIngestRecordInputKind>,
    /// Defaults to resource\[service.name\]
    #[serde(
        rename = "service_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_name: Option<String>,

    #[serde(rename = "start_time")]
    pub start_time: String,

    #[serde(rename = "end_time")]
    pub end_time: String,

    #[serde(
        rename = "status_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_code: Option<SpanIngestRecordInputStatusCode>,

    #[serde(
        rename = "status_message",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_message: Option<String>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "resource", default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "events", default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<SpanEventInput>>,

    #[serde(rename = "links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SpanLinkInput>>,
}
impl SpanIngestRecordInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trace_id: String,
        span_id: String,
        name: String,
        start_time: String,
        end_time: String,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id: None,
            name,
            kind: None,
            service_name: None,
            start_time,
            end_time,
            status_code: None,
            status_message: None,
            attributes: None,
            resource: None,
            events: None,
            links: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpanIngestRecordInputKind {
    INTERNAL,
    SERVER,
    CLIENT,
    PRODUCER,
    CONSUMER,
    Unknown(String),
}
impl SpanIngestRecordInputKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::INTERNAL => "INTERNAL",
            Self::SERVER => "SERVER",
            Self::CLIENT => "CLIENT",
            Self::PRODUCER => "PRODUCER",
            Self::CONSUMER => "CONSUMER",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SpanIngestRecordInputKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SpanIngestRecordInputKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "INTERNAL" => Self::INTERNAL,
            "SERVER" => Self::SERVER,
            "CLIENT" => Self::CLIENT,
            "PRODUCER" => Self::PRODUCER,
            "CONSUMER" => Self::CONSUMER,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpanIngestRecordInputStatusCode {
    UNSET,
    OK,
    ERROR,
    Unknown(String),
}
impl SpanIngestRecordInputStatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::UNSET => "UNSET",
            Self::OK => "OK",
            Self::ERROR => "ERROR",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SpanIngestRecordInputStatusCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SpanIngestRecordInputStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "UNSET" => Self::UNSET,
            "OK" => Self::OK,
            "ERROR" => Self::ERROR,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SpanEventInput {
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
}
impl SpanEventInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            timestamp: None,
            name,
            attributes: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SpanLinkInput {
    #[serde(rename = "trace_id")]
    pub trace_id: String,

    #[serde(rename = "span_id")]
    pub span_id: String,
}
impl SpanLinkInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(trace_id: String, span_id: String) -> Self {
        Self { trace_id, span_id }
    }
}

pub type IngestSpansBody = IngestSpansRequestInput;

pub type IngestSpansResponse = IngestResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListLogGroupsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListLogGroupsQuery = ListLogGroupsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LogGroupListResponse {
    #[serde(
        rename = "log_groups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_groups: Option<Vec<LogGroup>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PaginationMeta {
    /// Total number of items
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Number of items per page
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor for the next page. Pass it back as the `marker` query parameter; treat it as a token, not a value to parse.
    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// Whether there are more items
    #[serde(rename = "has_more", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

pub type ListLogGroupsResponse = LogGroupListResponse;

pub type ListLogGroupsItem = LogGroup;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricNamesParameters {
    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,
}
impl ListMetricNamesParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }
}

pub type ListMetricNamesQuery = ListMetricNamesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListMetricNamesResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
}

pub type ListMetricNamesResponse = ListMetricNamesResult;

pub type ListMetricNamesItem = String;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricNamesPostRequest {
    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,
}
impl ListMetricNamesPostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }
}

pub type ListMetricNamesPostBody = ListMetricNamesPostRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListMetricNamesPostResult {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
}

pub type ListMetricNamesPostResponse = ListMetricNamesPostResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricSeriesParameters {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,
}
impl ListMetricSeriesParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, start: String, end: String) -> Self {
        Self { metric, start, end }
    }
}

pub type ListMetricSeriesQuery = ListMetricSeriesParameters;

pub type ListMetricSeriesResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricSeriesPostRequest {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,
}
impl ListMetricSeriesPostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, start: String, end: String) -> Self {
        Self { metric, start, end }
    }
}

pub type ListMetricSeriesPostBody = ListMetricSeriesPostRequest;

pub type ListMetricSeriesPostResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateTraceSettingsRequestInput {
    /// 1..3650; pass clear_retention=true to switch to never-expire
    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub retention_days: Option<crate::Nullable<i64>>,
    /// When true, sets retention to never-expire (ignores retention_days)
    #[serde(
        rename = "clear_retention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clear_retention: Option<bool>,
    /// KMS key UUID, CRN or exact name in the authenticated account and serving region. PUT replaces settings; omission or an empty string selects plaintext for future spans. Historical ciphertext retains its original key UUID.
    #[serde(
        rename = "kms_key",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub kms_key: Option<crate::Nullable<String>>,
}

pub type PutTraceSettingsBody = UpdateTraceSettingsRequestInput;

pub type PutTraceSettingsResponse = TraceSettingsResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsInstantParameters {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: QueryMetricsInstantParametersAgg,

    #[serde(rename = "match[]", default, skip_serializing_if = "Option::is_none")]
    pub match__: Option<Vec<String>>,

    #[serde(rename = "by[]", default, skip_serializing_if = "Option::is_none")]
    pub by__: Option<Vec<String>>,

    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,

    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl QueryMetricsInstantParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, agg: QueryMetricsInstantParametersAgg) -> Self {
        Self {
            metric,
            agg,
            match__: None,
            by__: None,
            step: None,
            time: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryMetricsInstantParametersAgg {
    Avg,
    Sum,
    Min,
    Max,
    Count,
    Last,
    Rate,
    Increase,
    Unknown(String),
}
impl QueryMetricsInstantParametersAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Sum => "sum",
            Self::Min => "min",
            Self::Max => "max",
            Self::Count => "count",
            Self::Last => "last",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QueryMetricsInstantParametersAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QueryMetricsInstantParametersAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "sum" => Self::Sum,
            "min" => Self::Min,
            "max" => Self::Max,
            "count" => Self::Count,
            "last" => Self::Last,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

pub type QueryMetricsInstantQuery = QueryMetricsInstantParameters;

pub type QueryMetricsInstantResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsInstantPostRequest {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: QueryMetricsInstantPostRequestAgg,

    #[serde(rename = "match[]", default, skip_serializing_if = "Option::is_none")]
    pub match__: Option<Vec<String>>,

    #[serde(rename = "by[]", default, skip_serializing_if = "Option::is_none")]
    pub by__: Option<Vec<String>>,

    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,

    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}
impl QueryMetricsInstantPostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, agg: QueryMetricsInstantPostRequestAgg) -> Self {
        Self {
            metric,
            agg,
            match__: None,
            by__: None,
            step: None,
            time: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryMetricsInstantPostRequestAgg {
    Avg,
    Sum,
    Min,
    Max,
    Count,
    Last,
    Rate,
    Increase,
    Unknown(String),
}
impl QueryMetricsInstantPostRequestAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Sum => "sum",
            Self::Min => "min",
            Self::Max => "max",
            Self::Count => "count",
            Self::Last => "last",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QueryMetricsInstantPostRequestAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QueryMetricsInstantPostRequestAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "sum" => Self::Sum,
            "min" => Self::Min,
            "max" => Self::Max,
            "count" => Self::Count,
            "last" => Self::Last,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

pub type QueryMetricsInstantPostBody = QueryMetricsInstantPostRequest;

pub type QueryMetricsInstantPostResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsRangeParameters {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: QueryMetricsRangeParametersAgg,

    #[serde(rename = "match[]", default, skip_serializing_if = "Option::is_none")]
    pub match__: Option<Vec<String>>,

    #[serde(rename = "by[]", default, skip_serializing_if = "Option::is_none")]
    pub by__: Option<Vec<String>>,

    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,

    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
}
impl QueryMetricsRangeParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        metric: String,
        agg: QueryMetricsRangeParametersAgg,
        start: String,
        end: String,
    ) -> Self {
        Self {
            metric,
            agg,
            match__: None,
            by__: None,
            start,
            end,
            step: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryMetricsRangeParametersAgg {
    Avg,
    Sum,
    Min,
    Max,
    Count,
    Last,
    Rate,
    Increase,
    Unknown(String),
}
impl QueryMetricsRangeParametersAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Sum => "sum",
            Self::Min => "min",
            Self::Max => "max",
            Self::Count => "count",
            Self::Last => "last",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QueryMetricsRangeParametersAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QueryMetricsRangeParametersAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "sum" => Self::Sum,
            "min" => Self::Min,
            "max" => Self::Max,
            "count" => Self::Count,
            "last" => Self::Last,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

pub type QueryMetricsRangeQuery = QueryMetricsRangeParameters;

pub type QueryMetricsRangeResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsRangePostRequest {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: QueryMetricsRangePostRequestAgg,

    #[serde(rename = "match[]", default, skip_serializing_if = "Option::is_none")]
    pub match__: Option<Vec<String>>,

    #[serde(rename = "by[]", default, skip_serializing_if = "Option::is_none")]
    pub by__: Option<Vec<String>>,

    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,

    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,
}
impl QueryMetricsRangePostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        metric: String,
        agg: QueryMetricsRangePostRequestAgg,
        start: String,
        end: String,
    ) -> Self {
        Self {
            metric,
            agg,
            match__: None,
            by__: None,
            start,
            end,
            step: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryMetricsRangePostRequestAgg {
    Avg,
    Sum,
    Min,
    Max,
    Count,
    Last,
    Rate,
    Increase,
    Unknown(String),
}
impl QueryMetricsRangePostRequestAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Sum => "sum",
            Self::Min => "min",
            Self::Max => "max",
            Self::Count => "count",
            Self::Last => "last",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QueryMetricsRangePostRequestAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QueryMetricsRangePostRequestAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "sum" => Self::Sum,
            "min" => Self::Min,
            "max" => Self::Max,
            "count" => Self::Count,
            "last" => Self::Last,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

pub type QueryMetricsRangePostBody = QueryMetricsRangePostRequest;

pub type QueryMetricsRangePostResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchLogsParameters {
    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "log_group", default, skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,

    #[serde(
        rename = "log_stream",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_stream: Option<String>,

    #[serde(
        rename = "min_severity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_severity: Option<SearchLogsParametersMinSeverity>,

    #[serde(rename = "region", default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    #[serde(rename = "q", default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    #[serde(rename = "trace_id", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}
impl SearchLogsParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            log_group: None,
            log_stream: None,
            min_severity: None,
            region: None,
            q: None,
            trace_id: None,
            limit: None,
            marker: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchLogsParametersMinSeverity {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
    Unknown(String),
}
impl SearchLogsParametersMinSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TRACE => "TRACE",
            Self::DEBUG => "DEBUG",
            Self::INFO => "INFO",
            Self::WARN => "WARN",
            Self::ERROR => "ERROR",
            Self::FATAL => "FATAL",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SearchLogsParametersMinSeverity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchLogsParametersMinSeverity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "TRACE" => Self::TRACE,
            "DEBUG" => Self::DEBUG,
            "INFO" => Self::INFO,
            "WARN" => Self::WARN,
            "ERROR" => Self::ERROR,
            "FATAL" => Self::FATAL,
            _ => Self::Unknown(value),
        })
    }
}

pub type SearchLogsQuery = SearchLogsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LogListResponse {
    #[serde(rename = "logs", default, skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<LogRecord>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type SearchLogsResponse = LogListResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SearchTracesParameters {
    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: String,

    #[serde(rename = "service", default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    #[serde(rename = "operation", default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,

    #[serde(
        rename = "status_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub status_code: Option<SearchTracesParametersStatusCode>,

    #[serde(
        rename = "min_duration_ms",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_duration_ms: Option<f64>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}
impl SearchTracesParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            service: None,
            operation: None,
            status_code: None,
            min_duration_ms: None,
            limit: None,
            marker: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchTracesParametersStatusCode {
    UNSET,
    OK,
    ERROR,
    Unknown(String),
}
impl SearchTracesParametersStatusCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::UNSET => "UNSET",
            Self::OK => "OK",
            Self::ERROR => "ERROR",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SearchTracesParametersStatusCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchTracesParametersStatusCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "UNSET" => Self::UNSET,
            "OK" => Self::OK,
            "ERROR" => Self::ERROR,
            _ => Self::Unknown(value),
        })
    }
}

pub type SearchTracesQuery = SearchTracesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TraceListResponse {
    #[serde(rename = "traces", default, skip_serializing_if = "Option::is_none")]
    pub traces: Option<Vec<TraceSummary>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TraceSummary {
    #[serde(rename = "trace_id", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    #[serde(
        rename = "root_span_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub root_span_id: Option<String>,

    #[serde(rename = "root_name", default, skip_serializing_if = "Option::is_none")]
    pub root_name: Option<String>,

    #[serde(
        rename = "root_service",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub root_service: Option<String>,

    #[serde(
        rename = "start_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time: Option<String>,

    #[serde(
        rename = "duration_ms",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_ms: Option<f64>,

    #[serde(
        rename = "span_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub span_count: Option<i64>,

    #[serde(
        rename = "error_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_count: Option<i64>,

    #[serde(
        rename = "service_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_count: Option<i64>,
}

pub type SearchTracesResponse = TraceListResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateLogGroupRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,
    /// 1..3650; pass clear_retention to switch to never expire
    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_days: Option<i64>,
    /// When true, sets retention to never-expire (ignores retention_days)
    #[serde(
        rename = "clear_retention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clear_retention: Option<bool>,
    /// KMS key UUID, CRN or exact name in the authenticated account and serving region. Omission preserves the pinned UUID; an empty string clears encryption for future records. Unavailable key metadata does not clear the binding.
    #[serde(
        rename = "kms_key",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub kms_key: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateLogGroupBody = UpdateLogGroupRequestInput;

pub type UpdateLogGroupResponse = LogGroupResponse;
