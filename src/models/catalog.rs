//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Region {
    /// Catalog lifecycle, independent of health and capacity. Retired codes are never reused.
    #[serde(rename = "state")]
    pub state: RegionState,
    /// True only for active regions. Product services still enforce eligibility, quotas, placement and capacity.
    #[serde(rename = "accepting_new_resources")]
    pub accepting_new_resources: bool,
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
    /// Compatibility alias for accepting_new_resources; not a live health signal
    #[serde(rename = "available")]
    pub available: bool,
    /// Compatibility flag that is true only when state is planned
    #[serde(rename = "coming_soon")]
    pub coming_soon: bool,
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

pub type GetRegionResponse = Region;

pub type GetRegionResource = GetRegionResponse;

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
}

pub type ListRegionsResponse = ListRegionsResult;

pub type ListRegionsItem = Region;
