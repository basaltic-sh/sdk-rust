//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateAlertRequestInput {
    /// Immutable account-unique name. UUID names and the crn: prefix are reserved.
    #[serde(rename = "name")]
    pub name: String,
    /// Required same-account, same-region notification topic reference: UUID, canonical CRN or exact name, classified by syntax. Multiple alerts may share a topic. Binding requires notifications:Publish and respects explicit topic-policy denies.
    #[serde(rename = "topic")]
    pub topic: String,

    #[serde(rename = "expression")]
    pub expression: AlertExpressionInput,

    #[serde(rename = "operator")]
    pub operator: CreateAlertRequestInputOperator,

    #[serde(rename = "threshold")]
    pub threshold: f64,
    /// Whole-minute window. Evaluated every minute against the last complete UTC-aligned window.
    #[serde(rename = "window_seconds")]
    pub window_seconds: i64,
    /// Continuous breach duration before firing; 0 fires immediately. Missing data, query failures and scheduling gaps reset a pending duration.
    #[serde(rename = "for_seconds")]
    pub for_seconds: i64,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Mute notifications until this instant (at most 30 days ahead). Evaluation continues; null removes silence. An active alert notifies on the next evaluation after silence expires.
    #[serde(
        rename = "silenced_until",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub silenced_until: Option<crate::Nullable<String>>,
}
impl CreateAlertRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        topic: String,
        expression: AlertExpressionInput,
        operator: CreateAlertRequestInputOperator,
        threshold: f64,
        window_seconds: i64,
        for_seconds: i64,
    ) -> Self {
        Self {
            name,
            topic,
            expression,
            operator,
            threshold,
            window_seconds,
            for_seconds,
            enabled: None,
            silenced_until: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AlertExpressionInput {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: AlertExpressionInputAgg,
    /// Label matchers (label=value, label!=value, label=~regex, label!~regex). Tenant scope is always applied independently.
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<Vec<String>>,
    /// Grouping labels; the resulting series are summed to match the console metric card.
    #[serde(rename = "by", default, skip_serializing_if = "Option::is_none")]
    pub by: Option<Vec<String>>,

    #[serde(
        rename = "denominator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub denominator: Option<AlertQueryInput>,
    /// Multiplier after optional division, e.g. 100 for a percentage or 1/1048576 for MiB/s.
    #[serde(rename = "scale")]
    pub scale: f64,
    /// Display unit for the scaled threshold and value.
    #[serde(rename = "unit", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl AlertExpressionInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, agg: AlertExpressionInputAgg, scale: f64) -> Self {
        Self {
            metric,
            agg,
            match_: None,
            by: None,
            denominator: None,
            scale,
            unit: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertExpressionInputAgg {
    Avg,
    Min,
    Max,
    Sum,
    Last,
    Count,
    Rate,
    Increase,
    Unknown(String),
}
impl AlertExpressionInputAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Min => "min",
            Self::Max => "max",
            Self::Sum => "sum",
            Self::Last => "last",
            Self::Count => "count",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlertExpressionInputAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertExpressionInputAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "min" => Self::Min,
            "max" => Self::Max,
            "sum" => Self::Sum,
            "last" => Self::Last,
            "count" => Self::Count,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AlertQueryInput {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: AlertQueryInputAgg,
    /// Label matchers (label=value, label!=value, label=~regex, label!~regex). Tenant scope is always applied independently.
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<Vec<String>>,
    /// Grouping labels; the resulting series are summed to match the console metric card.
    #[serde(rename = "by", default, skip_serializing_if = "Option::is_none")]
    pub by: Option<Vec<String>>,
}
impl AlertQueryInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, agg: AlertQueryInputAgg) -> Self {
        Self {
            metric,
            agg,
            match_: None,
            by: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertQueryInputAgg {
    Avg,
    Min,
    Max,
    Sum,
    Last,
    Count,
    Rate,
    Increase,
    Unknown(String),
}
impl AlertQueryInputAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Min => "min",
            Self::Max => "max",
            Self::Sum => "sum",
            Self::Last => "last",
            Self::Count => "count",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlertQueryInputAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertQueryInputAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "min" => Self::Min,
            "max" => Self::Max,
            "sum" => Self::Sum,
            "last" => Self::Last,
            "count" => Self::Count,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateAlertRequestInputOperator {
    Gt,
    Gte,
    Lt,
    Lte,
    Unknown(String),
}
impl CreateAlertRequestInputOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateAlertRequestInputOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateAlertRequestInputOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "gt" => Self::Gt,
            "gte" => Self::Gte,
            "lt" => Self::Lt,
            "lte" => Self::Lte,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateAlertBody = CreateAlertRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AlertResponse {
    #[serde(rename = "alert")]
    pub alert: Alert,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Alert {
    #[serde(rename = "expression")]
    pub expression: AlertExpression,

    #[serde(rename = "operator")]
    pub operator: AlertOperator,

    #[serde(rename = "threshold")]
    pub threshold: f64,
    /// Whole-minute window. Evaluated every minute against the last complete UTC-aligned window.
    #[serde(rename = "window_seconds")]
    pub window_seconds: i64,
    /// Continuous breach duration before firing; 0 fires immediately. Missing data, query failures and scheduling gaps reset a pending duration.
    #[serde(rename = "for_seconds")]
    pub for_seconds: i64,

    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Mute notifications until this instant (at most 30 days ahead). Evaluation continues; null removes silence. An active alert notifies on the next evaluation after silence expires.
    #[serde(rename = "silenced_until")]
    pub silenced_until: crate::Nullable<String>,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "topic")]
    pub topic: ResourceRef,
    /// Evaluation state, independent of enabled and silenced_until. Missing data/errors do not resolve an open incident.
    #[serde(rename = "state")]
    pub state: AlertState,

    #[serde(rename = "last_value")]
    pub last_value: crate::Nullable<f64>,

    #[serde(
        rename = "evaluated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evaluated_at: Option<String>,

    #[serde(
        rename = "state_changed_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_changed_at: Option<String>,

    #[serde(
        rename = "pending_since",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pending_since: Option<String>,

    #[serde(
        rename = "evaluation_error",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evaluation_error: Option<String>,
    /// Notification remains queued for retry; metric evaluation continues.
    #[serde(
        rename = "delivery_error",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_error: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AlertExpression {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: AlertExpressionAgg,
    /// Label matchers (label=value, label!=value, label=~regex, label!~regex). Tenant scope is always applied independently.
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<Vec<String>>,
    /// Grouping labels; the resulting series are summed to match the console metric card.
    #[serde(rename = "by", default, skip_serializing_if = "Option::is_none")]
    pub by: Option<Vec<String>>,

    #[serde(
        rename = "denominator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub denominator: Option<AlertQuery>,
    /// Multiplier after optional division, e.g. 100 for a percentage or 1/1048576 for MiB/s.
    #[serde(rename = "scale")]
    pub scale: f64,
    /// Display unit for the scaled threshold and value.
    #[serde(rename = "unit", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertExpressionAgg {
    Avg,
    Min,
    Max,
    Sum,
    Last,
    Count,
    Rate,
    Increase,
    Unknown(String),
}
impl AlertExpressionAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Min => "min",
            Self::Max => "max",
            Self::Sum => "sum",
            Self::Last => "last",
            Self::Count => "count",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlertExpressionAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertExpressionAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "min" => Self::Min,
            "max" => Self::Max,
            "sum" => Self::Sum,
            "last" => Self::Last,
            "count" => Self::Count,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AlertQuery {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg")]
    pub agg: AlertQueryAgg,
    /// Label matchers (label=value, label!=value, label=~regex, label!~regex). Tenant scope is always applied independently.
    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<Vec<String>>,
    /// Grouping labels; the resulting series are summed to match the console metric card.
    #[serde(rename = "by", default, skip_serializing_if = "Option::is_none")]
    pub by: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertQueryAgg {
    Avg,
    Min,
    Max,
    Sum,
    Last,
    Count,
    Rate,
    Increase,
    Unknown(String),
}
impl AlertQueryAgg {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Avg => "avg",
            Self::Min => "min",
            Self::Max => "max",
            Self::Sum => "sum",
            Self::Last => "last",
            Self::Count => "count",
            Self::Rate => "rate",
            Self::Increase => "increase",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlertQueryAgg {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertQueryAgg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "avg" => Self::Avg,
            "min" => Self::Min,
            "max" => Self::Max,
            "sum" => Self::Sum,
            "last" => Self::Last,
            "count" => Self::Count,
            "rate" => Self::Rate,
            "increase" => Self::Increase,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertOperator {
    Gt,
    Gte,
    Lt,
    Lte,
    Unknown(String),
}
impl AlertOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlertOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "gt" => Self::Gt,
            "gte" => Self::Gte,
            "lt" => Self::Lt,
            "lte" => Self::Lte,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceRef {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Display label at response time — the resource's name where it has one, otherwise its human label (an invoice number, an account display name). Absent when the target is gone. Not a lookup key.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AlertState {
    Ok,
    Pending,
    Firing,
    NoData,
    Error,
    Unknown(String),
}
impl AlertState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ok => "ok",
            Self::Pending => "pending",
            Self::Firing => "firing",
            Self::NoData => "no_data",
            Self::Error => "error",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlertState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ok" => Self::Ok,
            "pending" => Self::Pending,
            "firing" => Self::Firing,
            "no_data" => Self::NoData,
            "error" => Self::Error,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateAlertResponse = AlertResponse;

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
    /// Retention in days. Omit or use null for no expiry.
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LogGroupResponse {
    #[serde(rename = "log_group")]
    pub log_group: LogGroup,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LogGroup {
    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<ResourceRef>,

    #[serde(rename = "id")]
    pub id: String,
    /// CRN of the log group (used as the IAM resource ARN)
    #[serde(rename = "crn")]
    pub crn: String,
    /// 1..512 chars, \[A-Za-z0-9_./#-\]. Leading "/" not allowed. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Retention in days, or null for no expiry.
    #[serde(rename = "retention_days")]
    pub retention_days: crate::Nullable<i64>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type CreateLogGroupResponse = LogGroupResponse;

pub type GetAlertResponse = AlertResponse;

pub type GetAlertResource = Alert;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetAlertScope {
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<GetAlertScopeState>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetAlertScopeState {
    Ok,
    Pending,
    Firing,
    NoData,
    Error,
    Silenced,
    Disabled,
    Unknown(String),
}
impl GetAlertScopeState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ok => "ok",
            Self::Pending => "pending",
            Self::Firing => "firing",
            Self::NoData => "no_data",
            Self::Error => "error",
            Self::Silenced => "silenced",
            Self::Disabled => "disabled",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetAlertScopeState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetAlertScopeState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ok" => Self::Ok,
            "pending" => Self::Pending,
            "firing" => Self::Firing,
            "no_data" => Self::NoData,
            "error" => Self::Error,
            "silenced" => Self::Silenced,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LogResponse {
    #[serde(rename = "log")]
    pub log: LogRecord,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LogRecord {
    #[serde(rename = "id")]
    pub id: String,
    /// Producer-stamped event time
    #[serde(rename = "timestamp")]
    pub timestamp: String,

    #[serde(rename = "log_group")]
    pub log_group: ResourceRef,
    /// Within-group stream name (typically the producer host)
    #[serde(rename = "log_stream")]
    pub log_stream: String,

    #[serde(rename = "severity")]
    pub severity: LogRecordSeverity,
    /// OTel numeric severity band (1..24)
    #[serde(rename = "severity_number")]
    pub severity_number: i64,

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
pub struct RetainedDataResponse {
    #[serde(rename = "retained_data")]
    pub retained_data: RetainedDataResponseRetainedData,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RetainedDataResponseRetainedData {
    #[serde(rename = "has_data")]
    pub has_data: bool,
    /// True when retained logs, spans or their storage rollups still block account deletion. Metric samples alone do not block account soft-deletion and expire through normal retention. Clients talking to older servers that omit this field must fall back to has_data.
    #[serde(
        rename = "blocks_deletion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocks_deletion: Option<bool>,
}

pub type GetRetainedTelemetryPresenceResponse = RetainedDataResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TraceResponse {
    #[serde(rename = "trace")]
    pub trace: TraceSummary,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TraceSummary {
    #[serde(rename = "trace_id")]
    pub trace_id: String,

    #[serde(rename = "root_span_id")]
    pub root_span_id: String,

    #[serde(rename = "root_name")]
    pub root_name: String,

    #[serde(rename = "root_service")]
    pub root_service: String,

    #[serde(rename = "start_time")]
    pub start_time: String,

    #[serde(rename = "duration_ms")]
    pub duration_ms: f64,

    #[serde(rename = "span_count")]
    pub span_count: i64,

    #[serde(rename = "error_count")]
    pub error_count: i64,

    #[serde(rename = "service_count")]
    pub service_count: i64,
}

pub type GetTraceResponse = TraceResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TraceSettingsResponse {
    #[serde(rename = "trace_settings")]
    pub trace_settings: TraceSettings,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TraceSettings {
    /// Effective retention in days. Defaults to 30; traces always expire.
    #[serde(rename = "retention_days")]
    pub retention_days: i64,

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

    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<ResourceRef>,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IngestResponse {
    #[serde(rename = "ingestion")]
    pub ingestion: IngestResult,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IngestResult {
    #[serde(rename = "accepted")]
    pub accepted: i64,

    #[serde(rename = "rejected")]
    pub rejected: i64,

    #[serde(rename = "errors", default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<IngestError>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IngestError {
    /// Zero-based input record index.
    #[serde(rename = "index")]
    pub index: i64,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "message")]
    pub message: String,
}

pub type IngestLogsResponse = IngestResponse;

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

pub type IngestSpansResponse = IngestResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListAlertsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ListAlertsParametersState>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListAlertsParametersState {
    Ok,
    Pending,
    Firing,
    NoData,
    Error,
    Silenced,
    Disabled,
    Unknown(String),
}
impl ListAlertsParametersState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ok => "ok",
            Self::Pending => "pending",
            Self::Firing => "firing",
            Self::NoData => "no_data",
            Self::Error => "error",
            Self::Silenced => "silenced",
            Self::Disabled => "disabled",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListAlertsParametersState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListAlertsParametersState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ok" => Self::Ok,
            "pending" => Self::Pending,
            "firing" => Self::Firing,
            "no_data" => Self::NoData,
            "error" => Self::Error,
            "silenced" => Self::Silenced,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListAlertsQuery = ListAlertsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AlertListResponse {
    #[serde(rename = "alerts")]
    pub alerts: Vec<Alert>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
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

pub type ListAlertsResponse = AlertListResponse;

pub type ListAlertsItem = Alert;

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LogGroupListResponse {
    #[serde(rename = "log_groups")]
    pub log_groups: Vec<LogGroup>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListLogGroupsResponse = LogGroupListResponse;

pub type ListLogGroupsItem = LogGroup;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricNamesParameters {
    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}
impl ListMetricNamesParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(start: String, end: String) -> Self {
        Self {
            start,
            end,
            limit: None,
            marker: None,
        }
    }
}

pub type ListMetricNamesQuery = ListMetricNamesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricNamesResponse {
    #[serde(rename = "metric_names")]
    pub metric_names: Vec<String>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListMetricNamesResponse = MetricNamesResponse;

pub type ListMetricNamesItem = String;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricNamesPostRequest {
    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}
impl ListMetricNamesPostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(start: String, end: String) -> Self {
        Self {
            start,
            end,
            limit: None,
            marker: None,
        }
    }
}

pub type ListMetricNamesPostBody = ListMetricNamesPostRequest;

pub type ListMetricNamesPostResponse = MetricNamesResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricSeriesParameters {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}
impl ListMetricSeriesParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, start: String, end: String) -> Self {
        Self {
            metric,
            start,
            end,
            limit: None,
            marker: None,
        }
    }
}

pub type ListMetricSeriesQuery = ListMetricSeriesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricSeriesResponse {
    #[serde(rename = "series")]
    pub series: Vec<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListMetricSeriesResponse = MetricSeriesResponse;

pub type ListMetricSeriesItem = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMetricSeriesPostRequest {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}
impl ListMetricSeriesPostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, start: String, end: String) -> Self {
        Self {
            metric,
            start,
            end,
            limit: None,
            marker: None,
        }
    }
}

pub type ListMetricSeriesPostBody = ListMetricSeriesPostRequest;

pub type ListMetricSeriesPostResponse = MetricSeriesResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListTraceSpansParameters {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListTraceSpansQuery = ListTraceSpansParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SpanListResponse {
    #[serde(rename = "spans")]
    pub spans: Vec<Span>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Span {
    #[serde(rename = "trace_id")]
    pub trace_id: String,

    #[serde(rename = "span_id")]
    pub span_id: String,

    #[serde(
        rename = "parent_span_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_span_id: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "kind")]
    pub kind: SpanKind,

    #[serde(rename = "service_name")]
    pub service_name: String,

    #[serde(rename = "start_time")]
    pub start_time: String,

    #[serde(rename = "end_time")]
    pub end_time: String,

    #[serde(rename = "duration_ms")]
    pub duration_ms: f64,

    #[serde(rename = "status_code")]
    pub status_code: SpanStatusCode,

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

pub type ListTraceSpansResponse = SpanListResponse;

pub type ListTraceSpansItem = Span;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsInstantParameters {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg", default, skip_serializing_if = "Option::is_none")]
    pub agg: Option<QueryMetricsInstantParametersAgg>,

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
    pub fn new(metric: String) -> Self {
        Self {
            metric,
            agg: None,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricQueryResponse {
    #[serde(rename = "query_result")]
    pub query_result: MetricQueryResult,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricQueryResult {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "series")]
    pub series: Vec<MetricQuerySeries>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricQuerySeries {
    #[serde(rename = "labels")]
    pub labels: std::collections::BTreeMap<String, String>,

    #[serde(rename = "points")]
    pub points: Vec<MetricPoint>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MetricPoint {
    /// Unix timestamp in seconds, including fractional seconds.
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    /// Float value as text, including NaN and infinities.
    #[serde(rename = "value")]
    pub value: String,
}

pub type QueryMetricsInstantResponse = MetricQueryResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsInstantPostRequest {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg", default, skip_serializing_if = "Option::is_none")]
    pub agg: Option<QueryMetricsInstantPostRequestAgg>,

    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,

    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,

    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<Vec<String>>,

    #[serde(rename = "by", default, skip_serializing_if = "Option::is_none")]
    pub by: Option<Vec<String>>,
}
impl QueryMetricsInstantPostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String) -> Self {
        Self {
            metric,
            agg: None,
            step: None,
            time: None,
            match_: None,
            by: None,
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

pub type QueryMetricsInstantPostResponse = MetricQueryResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsRangeParameters {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg", default, skip_serializing_if = "Option::is_none")]
    pub agg: Option<QueryMetricsRangeParametersAgg>,

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
    pub fn new(metric: String, start: String, end: String) -> Self {
        Self {
            metric,
            agg: None,
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

pub type QueryMetricsRangeResponse = MetricQueryResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueryMetricsRangePostRequest {
    #[serde(rename = "metric")]
    pub metric: String,

    #[serde(rename = "agg", default, skip_serializing_if = "Option::is_none")]
    pub agg: Option<QueryMetricsRangePostRequestAgg>,

    #[serde(rename = "start")]
    pub start: String,

    #[serde(rename = "end")]
    pub end: String,

    #[serde(rename = "step", default, skip_serializing_if = "Option::is_none")]
    pub step: Option<String>,

    #[serde(rename = "match", default, skip_serializing_if = "Option::is_none")]
    pub match_: Option<Vec<String>>,

    #[serde(rename = "by", default, skip_serializing_if = "Option::is_none")]
    pub by: Option<Vec<String>>,
}
impl QueryMetricsRangePostRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(metric: String, start: String, end: String) -> Self {
        Self {
            metric,
            agg: None,
            start,
            end,
            step: None,
            match_: None,
            by: None,
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

pub type QueryMetricsRangePostResponse = MetricQueryResponse;

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LogListResponse {
    #[serde(rename = "logs")]
    pub logs: Vec<LogRecord>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TraceListResponse {
    #[serde(rename = "traces")]
    pub traces: Vec<TraceSummary>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type SearchTracesResponse = TraceListResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateAlertRequestInput {
    /// Required same-account, same-region notification topic reference: UUID, canonical CRN or exact name, classified by syntax. Multiple alerts may share a topic. Binding requires notifications:Publish and respects explicit topic-policy denies.
    #[serde(rename = "topic", default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    #[serde(
        rename = "expression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expression: Option<AlertExpressionInput>,

    #[serde(rename = "operator", default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<UpdateAlertRequestInputOperator>,

    #[serde(rename = "threshold", default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    /// Whole-minute window. Evaluated every minute against the last complete UTC-aligned window.
    #[serde(
        rename = "window_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_seconds: Option<i64>,
    /// Continuous breach duration before firing; 0 fires immediately. Missing data, query failures and scheduling gaps reset a pending duration.
    #[serde(
        rename = "for_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub for_seconds: Option<i64>,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Mute notifications until this instant (at most 30 days ahead). Evaluation continues; null removes silence. An active alert notifies on the next evaluation after silence expires.
    #[serde(
        rename = "silenced_until",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub silenced_until: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateAlertRequestInputOperator {
    Gt,
    Gte,
    Lt,
    Lte,
    Unknown(String),
}
impl UpdateAlertRequestInputOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Gt => "gt",
            Self::Gte => "gte",
            Self::Lt => "lt",
            Self::Lte => "lte",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UpdateAlertRequestInputOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UpdateAlertRequestInputOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "gt" => Self::Gt,
            "gte" => Self::Gte,
            "lt" => Self::Lt,
            "lte" => Self::Lte,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateAlertBody = UpdateAlertRequestInput;

pub type UpdateAlertResponse = AlertResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateLogGroupRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,
    /// Omit to preserve retention. Null selects no expiry.
    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub retention_days: Option<crate::Nullable<i64>>,
    /// KMS key UUID, CRN or exact name in this account and region. Omit to preserve the binding; null clears encryption for future records.
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateTraceSettingsRequestInput {
    /// Omit to preserve retention. Null restores the default 30 days; traces always expire.
    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub retention_days: Option<crate::Nullable<i64>>,
    /// KMS key UUID, CRN or exact name in this account and region. Omit to preserve the binding; null clears encryption for future records.
    #[serde(
        rename = "kms_key",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub kms_key: Option<crate::Nullable<String>>,
}

pub type UpdateTraceSettingsBody = UpdateTraceSettingsRequestInput;

pub type UpdateTraceSettingsResponse = TraceSettingsResponse;
