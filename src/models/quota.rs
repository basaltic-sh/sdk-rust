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

pub type ListQuotasResponse = QuotaListResponse;

pub type ListQuotasItem = QuotaItem;
