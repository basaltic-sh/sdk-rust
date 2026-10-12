//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetCountryResult {
    #[serde(rename = "country")]
    pub country: CountryDetail,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CountryDetail {
    /// Stable in every environment.
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// ISO 3166-1 alpha-2 code. Also accepted wherever a country selector is.
    #[serde(rename = "code")]
    pub code: String,
    /// ISO 3166-1 alpha-3 code.
    #[serde(rename = "iso3")]
    pub iso3: String,
    /// English short name. Clients localize it from `code`.
    #[serde(rename = "name")]
    pub name: String,
    /// International calling code without the plus sign. Several countries share one (1 covers the United States and Canada). Empty when none is assigned.
    #[serde(rename = "calling_code")]
    pub calling_code: String,

    #[serde(rename = "rules")]
    pub rules: CountryRules,
    /// True when `GET /v1/postal-codes/{code}` can resolve this country's postal codes.
    #[serde(rename = "postal_lookup")]
    pub postal_lookup: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CountryRules {
    #[serde(rename = "customer_types")]
    pub customer_types: Vec<CountryRulesCustomerTypesItem>,
    /// Label for the recipient name, keyed by customer type.
    #[serde(rename = "name_labels")]
    pub name_labels: std::collections::BTreeMap<String, String>,
    /// Accepted tax identifier kinds, keyed by customer type.
    #[serde(rename = "tax_ids")]
    pub tax_ids: std::collections::BTreeMap<String, Vec<TaxIDRule>>,
    /// Address fields in form order. A key that is absent is not asked for in this country.
    #[serde(rename = "fields")]
    pub fields: Vec<AddressField>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CountryRulesCustomerTypesItem {
    Individual,
    Company,
    Unknown(String),
}
impl CountryRulesCustomerTypesItem {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Individual => "individual",
            Self::Company => "company",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CountryRulesCustomerTypesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CountryRulesCustomerTypesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "individual" => Self::Individual,
            "company" => Self::Company,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxIDRule {
    /// Identifier kind, returned as `tax_id_kind` on the billing profile. `other` is free text in a country without curated rules.
    #[serde(rename = "kind")]
    pub kind: String,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "mask", default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,

    #[serde(rename = "pattern", default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Named check-digit algorithm the server applies.
    #[serde(rename = "validator", default, skip_serializing_if = "Option::is_none")]
    pub validator: Option<TaxIDRuleValidator>,

    #[serde(rename = "example", default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    /// When true, `no_tax_id_reason` is not accepted instead.
    #[serde(rename = "required")]
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaxIDRuleValidator {
    BrCpf,
    BrCnpj,
    ArCuit,
    ClRut,
    Unknown(String),
}
impl TaxIDRuleValidator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BrCpf => "br_cpf",
            Self::BrCnpj => "br_cnpj",
            Self::ArCuit => "ar_cuit",
            Self::ClRut => "cl_rut",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TaxIDRuleValidator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxIDRuleValidator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "br_cpf" => Self::BrCpf,
            "br_cnpj" => Self::BrCnpj,
            "ar_cuit" => Self::ArCuit,
            "cl_rut" => Self::ClRut,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddressField {
    #[serde(rename = "key")]
    pub key: AddressFieldKey,

    #[serde(rename = "label")]
    pub label: String,

    #[serde(rename = "required")]
    pub required: bool,
    /// Maximum length in characters.
    #[serde(rename = "max", default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    /// Display mask: `9` a digit, `A` a letter, `*` a letter or digit, `?` makes the previous symbol optional, anything else is a literal. Values are stored without the literals.
    #[serde(rename = "mask", default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,
    /// Regular expression the stored value must match in full.
    #[serde(rename = "pattern", default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    #[serde(rename = "example", default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    /// For state and city only. `list` means pick a catalog row and send its selector; `free` means type the value under the address key.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<AddressFieldSource>,
    /// For a listed city, an internal attribute the city must carry for the profile to be complete.
    #[serde(
        rename = "require_metadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_metadata: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddressFieldKey {
    StreetName,
    StreetNumber,
    Complement,
    Neighborhood,
    City,
    State,
    PostalCode,
    Unknown(String),
}
impl AddressFieldKey {
    pub fn as_str(&self) -> &str {
        match self {
            Self::StreetName => "street_name",
            Self::StreetNumber => "street_number",
            Self::Complement => "complement",
            Self::Neighborhood => "neighborhood",
            Self::City => "city",
            Self::State => "state",
            Self::PostalCode => "postal_code",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AddressFieldKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AddressFieldKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "street_name" => Self::StreetName,
            "street_number" => Self::StreetNumber,
            "complement" => Self::Complement,
            "neighborhood" => Self::Neighborhood,
            "city" => Self::City,
            "state" => Self::State,
            "postal_code" => Self::PostalCode,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddressFieldSource {
    List,
    Free,
    Unknown(String),
}
impl AddressFieldSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::List => "list",
            Self::Free => "free",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AddressFieldSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AddressFieldSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "list" => Self::List,
            "free" => Self::Free,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetCountryResponse = GetCountryResult;

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
pub struct ListCitiesParameters {
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(rename = "q", default, skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListCitiesQuery = ListCitiesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListCitiesResult {
    #[serde(rename = "cities")]
    pub cities: Vec<City>,

    #[serde(rename = "meta")]
    pub meta: GeoPage,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct City {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "country")]
    pub country: ResourceRef,
    /// Null for countries without states.
    #[serde(rename = "state")]
    pub state: CityState,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CityState {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GeoPage {
    /// Items in this response.
    #[serde(rename = "total")]
    pub total: i64,

    #[serde(rename = "limit")]
    pub limit: i64,
    /// True when more rows matched than were returned. Narrow with `q`.
    #[serde(rename = "has_more")]
    pub has_more: bool,
}

pub type ListCitiesResponse = ListCitiesResult;

pub type ListCitiesItem = City;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListCountriesResult {
    #[serde(rename = "countries")]
    pub countries: Vec<Country>,

    #[serde(rename = "meta")]
    pub meta: GeoPage,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Country {
    /// Stable in every environment.
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// ISO 3166-1 alpha-2 code. Also accepted wherever a country selector is.
    #[serde(rename = "code")]
    pub code: String,
    /// ISO 3166-1 alpha-3 code.
    #[serde(rename = "iso3")]
    pub iso3: String,
    /// English short name. Clients localize it from `code`.
    #[serde(rename = "name")]
    pub name: String,
    /// International calling code without the plus sign. Several countries share one (1 covers the United States and Canada). Empty when none is assigned.
    #[serde(rename = "calling_code")]
    pub calling_code: String,
}

pub type ListCountriesResponse = ListCountriesResult;

pub type ListCountriesItem = Country;

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListStatesParameters {
    #[serde(rename = "country")]
    pub country: String,
}
impl ListStatesParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(country: String) -> Self {
        Self { country }
    }
}

pub type ListStatesQuery = ListStatesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListStatesResult {
    #[serde(rename = "states")]
    pub states: Vec<State>,

    #[serde(rename = "meta")]
    pub meta: GeoPage,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct State {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// ISO 3166-2 subdivision code without the country prefix, where one exists.
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "country")]
    pub country: ResourceRef,
}

pub type ListStatesResponse = ListStatesResult;

pub type ListStatesItem = State;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LookupPostalCodeParameters {
    #[serde(rename = "country")]
    pub country: String,
}
impl LookupPostalCodeParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(country: String) -> Self {
        Self { country }
    }
}

pub type LookupPostalCodeQuery = LookupPostalCodeParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LookupPostalCodeResult {
    #[serde(rename = "postal_code")]
    pub postal_code: PostalCode,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PostalCode {
    /// The postal code as stored, without mask literals.
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "country")]
    pub country: ResourceRef,
    /// The resolved catalog state, when the country lists states and the answer matched one.
    #[serde(rename = "state")]
    pub state: PostalCodeState,
    /// The resolved catalog city, when the country lists cities and the answer matched one.
    #[serde(rename = "city")]
    pub city: PostalCodeCity,
    /// Other address values the provider knew, limited to keys the country uses. State and city appear here as text where the country types them.
    #[serde(rename = "address")]
    pub address: std::collections::BTreeMap<String, String>,
    /// The provider that answered.
    #[serde(rename = "source")]
    pub source: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PostalCodeState {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PostalCodeCity {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

pub type LookupPostalCodeResponse = LookupPostalCodeResult;
