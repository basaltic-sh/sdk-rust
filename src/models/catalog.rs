//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RegionResponse {
    #[serde(rename = "region")]
    pub region: Region,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Region {
    /// Region lifecycle and availability. Only active regions accept new resources; planned, restricted, retiring, and retired regions do not. This does not guarantee live health, quota, or capacity.
    #[serde(rename = "state")]
    pub state: RegionState,
    /// Platform-owned global region identity, using the immutable region code.
    #[serde(rename = "crn")]
    pub crn: String,
    /// Unique region code used in API calls and CRNs
    #[serde(rename = "code")]
    pub code: String,
    /// Human-readable region name
    #[serde(rename = "name")]
    pub name: String,
    /// Geographic location of the region
    #[serde(rename = "location")]
    pub location: String,
    /// ISO 3166-1 alpha-2 country code (used to display flag in UI)
    #[serde(rename = "country_code")]
    pub country_code: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionState {
    Planned,
    Active,
    Restricted,
    Retiring,
    Retired,
    Unknown(String),
}
impl RegionState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Planned => "planned",
            Self::Active => "active",
            Self::Restricted => "restricted",
            Self::Retiring => "retiring",
            Self::Retired => "retired",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RegionState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RegionState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "planned" => Self::Planned,
            "active" => Self::Active,
            "restricted" => Self::Restricted,
            "retiring" => Self::Retiring,
            "retired" => Self::Retired,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetRegionResponse = RegionResponse;

pub type GetRegionResource = Region;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRegionScope {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRegionsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListRegionsQuery = ListRegionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListRegionsResult {
    #[serde(rename = "regions")]
    pub regions: Vec<Region>,
    /// Default accepting region code, or an empty string.
    #[serde(rename = "default")]
    pub default: String,

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

pub type ListRegionsResponse = ListRegionsResult;

pub type ListRegionsItem = Region;
