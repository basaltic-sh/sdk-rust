//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListQuotasParameters {
    #[serde(rename = "region", default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

pub type ListQuotasQuery = ListQuotasParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotaListResponse {
    #[serde(rename = "quotas")]
    pub quotas: Vec<QuotaItem>,
    /// Non-reservation policies for the organization in the requested region. Currently contains the four Email size and recipient policies. Empty when region is omitted. Older servers may omit this additive field.
    #[serde(rename = "policies", default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<QuotaPolicy>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotaItem {
    /// The service that owns the quota. Together with resource_type it identifies the quota — resource_type alone is not unique (e.g. both compute and database have an `instances` quota).
    #[serde(rename = "service")]
    pub service: String,
    /// The resource the cap applies to. See migrations/quota/001_quotas.sql for the seeded list (instances, vcpus, ram_mb, volumes, …, domains_per_certificate, records_per_zone, …).
    #[serde(rename = "resource_type")]
    pub resource_type: String,

    #[serde(rename = "scope")]
    pub scope: QuotaItemScope,
    /// Effective limit (-1 = unlimited). Override if set, otherwise default.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Resources currently consuming this quota. Zero for per_resource quotas (no running counter — the cap applies per parent resource).
    #[serde(rename = "in_use")]
    pub in_use: i64,
    /// Resources temporarily reserved during async creation. Zero for per_resource quotas.
    #[serde(rename = "reserved")]
    pub reserved: i64,
    /// limit - in_use - reserved, floored at 0; -1 if unlimited.
    #[serde(rename = "available")]
    pub available: i64,
    /// True if the limit comes from the system default; false if there is a per-org override.
    #[serde(rename = "is_default")]
    pub is_default: bool,

    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuotaItemScope {
    Regional,
    Global,
    PerResource,
    Unknown(String),
}
impl QuotaItemScope {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Regional => "regional",
            Self::Global => "global",
            Self::PerResource => "per_resource",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QuotaItemScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotaItemScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "regional" => Self::Regional,
            "global" => Self::Global,
            "per_resource" => Self::PerResource,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QuotaPolicy {
    #[serde(rename = "service")]
    pub service: QuotaPolicyService,

    #[serde(rename = "resource_type")]
    pub resource_type: QuotaPolicyResourceType,
    /// The policy applies to the caller's organization in region, even if its value comes from an organization-wide override.
    #[serde(rename = "scope")]
    pub scope: QuotaPolicyScope,

    #[serde(rename = "region")]
    pub region: String,
    /// Effective policy value after regional and organization overrides; -1 means unlimited by this policy and 0 permits none. The separate Email message ceiling of 52428800 bytes (50 MiB) applies even when the size policy is unlimited or larger than the ceiling.
    #[serde(rename = "limit")]
    pub limit: i64,
    /// Bytes per complete encoded sending message, or accepted recipients per rolling window.
    #[serde(rename = "unit")]
    pub unit: QuotaPolicyUnit,
    /// Rolling window length for recipient policies. Omitted for message size; 86400 means rolling 24 hours.
    #[serde(
        rename = "window_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_seconds: Option<i64>,
    /// Platform default, organization-wide override, or override for the requested region.
    #[serde(rename = "limit_source")]
    pub limit_source: QuotaPolicyLimitSource,

    #[serde(rename = "description")]
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuotaPolicyService {
    Email,
    Unknown(String),
}
impl QuotaPolicyService {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Email => "email",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QuotaPolicyService {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotaPolicyService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "email" => Self::Email,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuotaPolicyResourceType {
    MessageSizeBytes,
    SentRecipientsPerSecond,
    SentRecipientsPerDay,
    ReceivedRecipientsPerSecond,
    Unknown(String),
}
impl QuotaPolicyResourceType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MessageSizeBytes => "message_size_bytes",
            Self::SentRecipientsPerSecond => "sent_recipients_per_second",
            Self::SentRecipientsPerDay => "sent_recipients_per_day",
            Self::ReceivedRecipientsPerSecond => "received_recipients_per_second",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QuotaPolicyResourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotaPolicyResourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "message_size_bytes" => Self::MessageSizeBytes,
            "sent_recipients_per_second" => Self::SentRecipientsPerSecond,
            "sent_recipients_per_day" => Self::SentRecipientsPerDay,
            "received_recipients_per_second" => Self::ReceivedRecipientsPerSecond,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuotaPolicyScope {
    Regional,
    Unknown(String),
}
impl QuotaPolicyScope {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Regional => "regional",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QuotaPolicyScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotaPolicyScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "regional" => Self::Regional,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuotaPolicyUnit {
    Bytes,
    Recipients,
    Unknown(String),
}
impl QuotaPolicyUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bytes => "bytes",
            Self::Recipients => "recipients",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QuotaPolicyUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotaPolicyUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "bytes" => Self::Bytes,
            "recipients" => Self::Recipients,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuotaPolicyLimitSource {
    Default,
    Organization,
    Region,
    Unknown(String),
}
impl QuotaPolicyLimitSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::Organization => "organization",
            Self::Region => "region",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QuotaPolicyLimitSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QuotaPolicyLimitSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "default" => Self::Default,
            "organization" => Self::Organization,
            "region" => Self::Region,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListQuotasResponse = QuotaListResponse;
