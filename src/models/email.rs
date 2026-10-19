//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SuppressionOverrideRequestInput {
    #[serde(rename = "address")]
    pub address: String,
}
impl SuppressionOverrideRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

pub type AddSuppressionOverrideBody = SuppressionOverrideRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConfigurationSetResponse {
    #[serde(rename = "configuration_set")]
    pub configuration_set: ConfigurationSet,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConfigurationSet {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "sending_enabled")]
    pub sending_enabled: bool,

    #[serde(rename = "suppression_overrides")]
    pub suppression_overrides: Vec<String>,

    #[serde(rename = "ip_pool", default, skip_serializing_if = "Option::is_none")]
    pub ip_pool: Option<ResourceRef>,

    #[serde(
        rename = "tracking_open",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tracking_open: Option<bool>,

    #[serde(
        rename = "tracking_click",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tracking_click: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

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

pub type Tags = std::collections::BTreeMap<String, String>;

pub type AddSuppressionOverrideResponse = ConfigurationSetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdentityResponse {
    #[serde(rename = "identity")]
    pub identity: Identity,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Identity {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Sender domain. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub type_: IdentityType,

    #[serde(rename = "status")]
    pub status: IdentityStatus,

    #[serde(rename = "dkim_selector")]
    pub dkim_selector: String,

    #[serde(
        rename = "dkim_pending_selector",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dkim_pending_selector: Option<String>,

    #[serde(
        rename = "dkim_selector_old",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dkim_selector_old: Option<String>,

    #[serde(
        rename = "dkim_old_expires_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub dkim_old_expires_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "mail_from_domain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mail_from_domain: Option<String>,

    #[serde(
        rename = "mail_from_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mail_from_status: Option<IdentityMailFromStatus>,

    #[serde(
        rename = "mail_from_verified_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub mail_from_verified_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "bimi_logo_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bimi_logo_url: Option<String>,

    #[serde(
        rename = "bimi_vmc_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bimi_vmc_url: Option<String>,

    #[serde(
        rename = "bimi_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bimi_status: Option<IdentityBimiStatus>,

    #[serde(
        rename = "bimi_verified_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub bimi_verified_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "verified_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub verified_at: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(
        rename = "suspended_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suspended_reason: Option<String>,

    #[serde(
        rename = "suspended_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub suspended_at: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(
        rename = "dns_records",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dns_records: Option<Vec<DNSRecord>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityType {
    Domain,
    Unknown(String),
}
impl IdentityType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Domain => "domain",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for IdentityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "domain" => Self::Domain,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityStatus {
    Pending,
    Verified,
    Failed,
    Suspended,
    Unknown(String),
}
impl IdentityStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Failed => "failed",
            Self::Suspended => "suspended",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for IdentityStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "verified" => Self::Verified,
            "failed" => Self::Failed,
            "suspended" => Self::Suspended,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityMailFromStatus {
    Pending,
    Verified,
    Failed,
    Unknown(String),
}
impl IdentityMailFromStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Failed => "failed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for IdentityMailFromStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityMailFromStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "verified" => Self::Verified,
            "failed" => Self::Failed,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IdentityBimiStatus {
    Pending,
    Verified,
    Failed,
    Unknown(String),
}
impl IdentityBimiStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Failed => "failed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for IdentityBimiStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IdentityBimiStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "verified" => Self::Verified,
            "failed" => Self::Failed,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DNSRecord {
    #[serde(rename = "purpose")]
    pub purpose: DNSRecordPurpose,

    #[serde(rename = "type")]
    pub type_: DNSRecordType,
    /// FQDN to publish the record at.
    #[serde(rename = "name")]
    pub name: String,
    /// Complete copyable record data: unquoted TXT content, CNAME target, or MX priority followed by host.
    #[serde(rename = "value")]
    pub value: String,
    /// Required for this sending/receiving function. For an alternative_group, one alternative satisfies the requirement.
    #[serde(rename = "required")]
    pub required: bool,

    #[serde(rename = "group")]
    pub group: DNSRecordGroup,
    /// Records sharing this value are alternatives. Publish exactly one record from a required alternative group; TXT and CNAME cannot coexist at the same name.
    #[serde(
        rename = "alternative_group",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub alternative_group: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DNSRecordPurpose {
    Ownership,
    Dkim,
    DkimCname,
    DkimPending,
    DkimPendingCname,
    DkimRotatedOut,
    Spf,
    Dmarc,
    ReturnPath,
    MailFromMx,
    MailFromSpf,
    Bimi,
    ReceivingMx,
    Unknown(String),
}
impl DNSRecordPurpose {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ownership => "ownership",
            Self::Dkim => "dkim",
            Self::DkimCname => "dkim_cname",
            Self::DkimPending => "dkim_pending",
            Self::DkimPendingCname => "dkim_pending_cname",
            Self::DkimRotatedOut => "dkim_rotated_out",
            Self::Spf => "spf",
            Self::Dmarc => "dmarc",
            Self::ReturnPath => "return_path",
            Self::MailFromMx => "mail_from_mx",
            Self::MailFromSpf => "mail_from_spf",
            Self::Bimi => "bimi",
            Self::ReceivingMx => "receiving_mx",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for DNSRecordPurpose {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DNSRecordPurpose {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ownership" => Self::Ownership,
            "dkim" => Self::Dkim,
            "dkim_cname" => Self::DkimCname,
            "dkim_pending" => Self::DkimPending,
            "dkim_pending_cname" => Self::DkimPendingCname,
            "dkim_rotated_out" => Self::DkimRotatedOut,
            "spf" => Self::Spf,
            "dmarc" => Self::Dmarc,
            "return_path" => Self::ReturnPath,
            "mail_from_mx" => Self::MailFromMx,
            "mail_from_spf" => Self::MailFromSpf,
            "bimi" => Self::Bimi,
            "receiving_mx" => Self::ReceivingMx,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DNSRecordType {
    TXT,
    CNAME,
    MX,
    Unknown(String),
}
impl DNSRecordType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TXT => "TXT",
            Self::CNAME => "CNAME",
            Self::MX => "MX",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for DNSRecordType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DNSRecordType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "TXT" => Self::TXT,
            "CNAME" => Self::CNAME,
            "MX" => Self::MX,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DNSRecordGroup {
    Sending,
    Receiving,
    Unknown(String),
}
impl DNSRecordGroup {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Sending => "sending",
            Self::Receiving => "receiving",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for DNSRecordGroup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DNSRecordGroup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "sending" => Self::Sending,
            "receiving" => Self::Receiving,
            _ => Self::Unknown(value),
        })
    }
}

pub type BeginDKIMRotationResponse = IdentityResponse;

pub type CancelDKIMRotationResponse = IdentityResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigurationSetRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "sending_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sending_enabled: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
    /// UUID, CRN or account-unique name. Omit to preserve; null detaches the pool.
    #[serde(
        rename = "ip_pool",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub ip_pool: Option<crate::Nullable<String>>,

    #[serde(
        rename = "tracking_open",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tracking_open: Option<bool>,

    #[serde(
        rename = "tracking_click",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tracking_click: Option<bool>,

    #[serde(
        rename = "suppression_overrides",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suppression_overrides: Option<Vec<String>>,
}
impl CreateConfigurationSetRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            sending_enabled: None,
            tags: None,
            ip_pool: None,
            tracking_open: None,
            tracking_click: None,
            suppression_overrides: None,
        }
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateConfigurationSetBody = CreateConfigurationSetRequestInput;

pub type CreateConfigurationSetResponse = ConfigurationSetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateIPPoolRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}
impl CreateIPPoolRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
        }
    }
}

pub type CreateIPPoolBody = CreateIPPoolRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IPPoolResponse {
    #[serde(rename = "ip_pool")]
    pub ip_pool: IPPool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IPPool {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type CreateIPPoolResponse = IPPoolResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateIdentityRequestInput {
    /// Sender domain (FQDN, lowercase). Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl CreateIdentityRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self { name, tags: None }
    }
}

pub type CreateIdentityBody = CreateIdentityRequestInput;

pub type CreateIdentityResponse = IdentityResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateRecipientRuleRequestInput {
    /// Exact local part, local-part prefix, or catch-all for this identity. Exact matches take precedence, then the longest prefix, then catch-all.
    #[serde(rename = "match_type")]
    pub match_type: CreateRecipientRuleRequestInputMatchType,
    /// Required for exact and prefix; empty or omitted for all.
    #[serde(
        rename = "local_part",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_part: Option<String>,
    /// Notifications topic name, UUID or CRN. Requires notifications:Publish on the same-account topic.
    #[serde(rename = "topic")]
    pub topic: String,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl CreateRecipientRuleRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(match_type: CreateRecipientRuleRequestInputMatchType, topic: String) -> Self {
        Self {
            match_type,
            local_part: None,
            topic,
            enabled: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateRecipientRuleRequestInputMatchType {
    Exact,
    Prefix,
    All,
    Unknown(String),
}
impl CreateRecipientRuleRequestInputMatchType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
            Self::Prefix => "prefix",
            Self::All => "all",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateRecipientRuleRequestInputMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateRecipientRuleRequestInputMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "exact" => Self::Exact,
            "prefix" => Self::Prefix,
            "all" => Self::All,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateRecipientRuleBody = CreateRecipientRuleRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RecipientRuleResponse {
    #[serde(rename = "rule")]
    pub rule: RecipientRule,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RecipientRule {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "local_part")]
    pub local_part: String,

    #[serde(rename = "topic")]
    pub topic: ResourceRef,

    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Exact local part, local-part prefix, or catch-all for this identity. Exact matches take precedence, then the longest prefix, then catch-all.
    #[serde(rename = "match_type")]
    pub match_type: RecipientRuleMatchType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecipientRuleMatchType {
    Exact,
    Prefix,
    All,
    Unknown(String),
}
impl RecipientRuleMatchType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
            Self::Prefix => "prefix",
            Self::All => "all",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RecipientRuleMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecipientRuleMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "exact" => Self::Exact,
            "prefix" => Self::Prefix,
            "all" => Self::All,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateRecipientRuleResponse = RecipientRuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateSuppressionRequestInput {
    #[serde(rename = "address")]
    pub address: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,
}
impl CreateSuppressionRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(address: String) -> Self {
        Self {
            address,
            description: None,
            expires_at: None,
        }
    }
}

pub type CreateSuppressionBody = CreateSuppressionRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SuppressionResponse {
    #[serde(rename = "suppression")]
    pub suppression: Suppression,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Suppression {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "reason")]
    pub reason: SuppressionReason,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename = "source_message",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_message: Option<ResourceRef>,

    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub expires_at: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuppressionReason {
    AutoBounce,
    AutoComplaint,
    Manual,
    Unknown(String),
}
impl SuppressionReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AutoBounce => "auto_bounce",
            Self::AutoComplaint => "auto_complaint",
            Self::Manual => "manual",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SuppressionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SuppressionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "auto_bounce" => Self::AutoBounce,
            "auto_complaint" => Self::AutoComplaint,
            "manual" => Self::Manual,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateSuppressionResponse = SuppressionResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTemplateRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "subject_template",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_template: Option<String>,

    #[serde(rename = "body_template")]
    pub body_template: String,

    #[serde(rename = "body_type", default, skip_serializing_if = "Option::is_none")]
    pub body_type: Option<CreateTemplateRequestInputBodyType>,
}
impl CreateTemplateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, body_template: String) -> Self {
        Self {
            name,
            subject_template: None,
            body_template,
            body_type: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTemplateRequestInputBodyType {
    Text,
    Html,
    Unknown(String),
}
impl CreateTemplateRequestInputBodyType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Text => "text",
            Self::Html => "html",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateTemplateRequestInputBodyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateTemplateRequestInputBodyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "text" => Self::Text,
            "html" => Self::Html,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateTemplateBody = CreateTemplateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TemplateResponse {
    #[serde(rename = "template")]
    pub template: Template,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Template {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "subject_template")]
    pub subject_template: String,

    #[serde(rename = "body_template")]
    pub body_template: String,

    #[serde(rename = "body_type")]
    pub body_type: TemplateBodyType,

    #[serde(rename = "variables")]
    pub variables: Vec<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplateBodyType {
    Text,
    Html,
    Unknown(String),
}
impl TemplateBodyType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Text => "text",
            Self::Html => "html",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TemplateBodyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TemplateBodyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "text" => Self::Text,
            "html" => Self::Html,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateTemplateResponse = TemplateResponse;

pub type DescribeConfigurationSetResponse = ConfigurationSetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DedicatedIPResponse {
    #[serde(rename = "dedicated_ip")]
    pub dedicated_ip: DedicatedIP,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DedicatedIP {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "warmup_status")]
    pub warmup_status: DedicatedIPWarmupStatus,
    /// Progress computed by the warmup worker; cannot be edited.
    #[serde(rename = "warmup_percentage")]
    pub warmup_percentage: i64,

    #[serde(
        rename = "warmup_started_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub warmup_started_at: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "ip_pool")]
    pub ip_pool: crate::Nullable<ResourceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DedicatedIPWarmupStatus {
    New,
    Warming,
    Warm,
    Unknown(String),
}
impl DedicatedIPWarmupStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::New => "new",
            Self::Warming => "warming",
            Self::Warm => "warm",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for DedicatedIPWarmupStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DedicatedIPWarmupStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "new" => Self::New,
            "warming" => Self::Warming,
            "warm" => Self::Warm,
            _ => Self::Unknown(value),
        })
    }
}

pub type DescribeDedicatedIPResponse = DedicatedIPResponse;

pub type DescribeIPPoolResponse = IPPoolResponse;

pub type DescribeIdentityResponse = IdentityResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivedMessageResponse {
    #[serde(rename = "message")]
    pub message: ReceivedMessage,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivedMessage {
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "recipient")]
    pub recipient: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "subject")]
    pub subject: String,

    #[serde(rename = "received_at")]
    pub received_at: String,

    #[serde(rename = "expires_at")]
    pub expires_at: String,

    #[serde(rename = "published_at")]
    pub published_at: crate::Nullable<String>,

    #[serde(rename = "content_available")]
    pub content_available: bool,

    #[serde(rename = "review_reason")]
    pub review_reason: ReceivedMessageReviewReason,

    #[serde(rename = "authenticated")]
    pub authenticated: bool,

    #[serde(rename = "attachments")]
    pub attachments: Vec<Attachment>,
    /// Receiving identity; null only when the original identity no longer exists.
    #[serde(rename = "identity")]
    pub identity: crate::Nullable<ResourceRef>,

    #[serde(rename = "notification_status")]
    pub notification_status: ReceivedMessageNotificationStatus,
    /// Safe publication failure detail. Retried automatically while content is retained.
    #[serde(rename = "notification_error")]
    pub notification_error: crate::Nullable<String>,
    /// RFC Message-ID header, distinct from the Email resource UUID.
    #[serde(
        rename = "rfc_message_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rfc_message_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReceivedMessageReviewReason {
    Value,
    Spam,
    SenderUnverified,
    Malware,
    Unknown(String),
}
impl ReceivedMessageReviewReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value => "",
            Self::Spam => "spam",
            Self::SenderUnverified => "sender_unverified",
            Self::Malware => "malware",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ReceivedMessageReviewReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedMessageReviewReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "" => Self::Value,
            "spam" => Self::Spam,
            "sender_unverified" => Self::SenderUnverified,
            "malware" => Self::Malware,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "content_type")]
    pub content_type: String,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "bucket")]
    pub bucket: crate::Nullable<ResourceRef>,

    #[serde(rename = "key")]
    pub key: String,
    /// Opaque Storage object version token, when versioning is enabled.
    #[serde(
        rename = "version_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_id: Option<String>,
    /// SHA-256 of the decoded attachment bytes.
    #[serde(rename = "sha256")]
    pub sha256: String,

    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReceivedMessageNotificationStatus {
    Pending,
    Retrying,
    Published,
    Unknown(String),
}
impl ReceivedMessageNotificationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Retrying => "retrying",
            Self::Published => "published",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ReceivedMessageNotificationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReceivedMessageNotificationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "retrying" => Self::Retrying,
            "published" => Self::Published,
            _ => Self::Unknown(value),
        })
    }
}

pub type DescribeReceivedMessageResponse = ReceivedMessageResponse;

pub type DescribeRecipientRuleResponse = RecipientRuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageResponse {
    #[serde(rename = "message")]
    pub message: Message,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    /// Message content expires 30 days after acceptance.
    #[serde(rename = "expires_at")]
    pub expires_at: String,

    #[serde(rename = "attachments")]
    pub attachments: Vec<Attachment>,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "identity")]
    pub identity: ResourceRef,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: Vec<String>,

    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(rename = "status")]
    pub status: MessageStatus,

    #[serde(
        rename = "smtp_response",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub smtp_response: Option<String>,

    #[serde(
        rename = "error_message",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_message: Option<String>,

    #[serde(
        rename = "outbound_ip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_ip: Option<String>,

    #[serde(rename = "queued_at")]
    pub queued_at: String,

    #[serde(
        rename = "sent_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub sent_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "failed_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub failed_at: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageStatus {
    Queued,
    Sending,
    Sent,
    Failed,
    Bounced,
    Complained,
    Unknown(String),
}
impl MessageStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Queued => "queued",
            Self::Sending => "sending",
            Self::Sent => "sent",
            Self::Failed => "failed",
            Self::Bounced => "bounced",
            Self::Complained => "complained",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for MessageStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MessageStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "queued" => Self::Queued,
            "sending" => Self::Sending,
            "sent" => Self::Sent,
            "failed" => Self::Failed,
            "bounced" => Self::Bounced,
            "complained" => Self::Complained,
            _ => Self::Unknown(value),
        })
    }
}

pub type DescribeSentMessageResponse = MessageResponse;

pub type DescribeSuppressionResponse = SuppressionResponse;

pub type DescribeTemplateResponse = TemplateResponse;

pub type FinalizeDKIMRotationResponse = IdentityResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageContentResponse {
    #[serde(rename = "content")]
    pub content: MessageContent,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageContent {
    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: Vec<String>,

    #[serde(rename = "subject")]
    pub subject: String,
    /// Full decoded plain-text body.
    #[serde(rename = "text")]
    pub text: String,
    /// Full decoded HTML. Untrusted content: sanitize before rendering; do not load remote resources automatically.
    #[serde(rename = "html")]
    pub html: String,
    /// RFC Message-ID header, distinct from the Email resource UUID.
    #[serde(
        rename = "rfc_message_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rfc_message_id: Option<String>,

    #[serde(rename = "attachments")]
    pub attachments: Vec<Attachment>,
}

pub type GetReceivedMessageContentResponse = MessageContentResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivingConfigurationResponse {
    #[serde(rename = "receiving")]
    pub receiving: ReceivingConfiguration,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivingConfiguration {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "attachments_bucket")]
    pub attachments_bucket: crate::Nullable<ResourceRef>,
}

pub type GetReceivingConfigurationResponse = ReceivingConfigurationResponse;

pub type GetSentMessageContentResponse = MessageContentResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageEngagementResponse {
    #[serde(rename = "engagement")]
    pub engagement: MessageEngagement,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageEngagement {
    #[serde(rename = "generated_at")]
    pub generated_at: String,

    #[serde(rename = "open_count")]
    pub open_count: i64,

    #[serde(rename = "unique_opens")]
    pub unique_opens: i64,

    #[serde(rename = "click_count")]
    pub click_count: i64,

    #[serde(rename = "unique_clicks")]
    pub unique_clicks: i64,

    #[serde(
        rename = "first_open_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub first_open_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "last_open_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_open_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "first_click_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub first_click_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "last_click_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_click_at: Option<crate::Nullable<String>>,
}

pub type GetSentMessageEngagementResponse = MessageEngagementResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListConfigurationSetsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListConfigurationSetsQuery = ListConfigurationSetsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ConfigurationSetListResponse {
    #[serde(rename = "configuration_sets")]
    pub configuration_sets: Vec<ConfigurationSet>,

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

pub type ListConfigurationSetsResponse = ConfigurationSetListResponse;

pub type ListConfigurationSetsItem = ConfigurationSet;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListDMARCReportsParameters {
    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

pub type ListDMARCReportsQuery = ListDMARCReportsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DMARCReportListResponse {
    #[serde(rename = "records")]
    pub records: Vec<DMARCReportRecord>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DMARCReportRecord {
    #[serde(rename = "reporter")]
    pub reporter: String,

    #[serde(rename = "report_id")]
    pub report_id: String,

    #[serde(rename = "period_start")]
    pub period_start: String,

    #[serde(rename = "period_end")]
    pub period_end: String,

    #[serde(rename = "received_at")]
    pub received_at: String,

    #[serde(rename = "source_ip")]
    pub source_ip: String,

    #[serde(rename = "message_count")]
    pub message_count: i64,

    #[serde(rename = "disposition")]
    pub disposition: String,

    #[serde(rename = "dkim_aligned")]
    pub dkim_aligned: bool,

    #[serde(rename = "spf_aligned")]
    pub spf_aligned: bool,

    #[serde(rename = "dkim_domain")]
    pub dkim_domain: String,

    #[serde(rename = "dkim_result")]
    pub dkim_result: String,

    #[serde(rename = "spf_domain")]
    pub spf_domain: String,

    #[serde(rename = "spf_result")]
    pub spf_result: String,

    #[serde(rename = "header_from")]
    pub header_from: String,
}

pub type ListDMARCReportsResponse = DMARCReportListResponse;

pub type ListDMARCReportsItem = DMARCReportRecord;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListDedicatedIPsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "ip_pool", default, skip_serializing_if = "Option::is_none")]
    pub ip_pool: Option<String>,
}

pub type ListDedicatedIPsQuery = ListDedicatedIPsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DedicatedIPListResponse {
    #[serde(rename = "dedicated_ips")]
    pub dedicated_ips: Vec<DedicatedIP>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListDedicatedIPsResponse = DedicatedIPListResponse;

pub type ListDedicatedIPsItem = DedicatedIP;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListIPPoolsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListIPPoolsQuery = ListIPPoolsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IPPoolListResponse {
    #[serde(rename = "ip_pools")]
    pub ip_pools: Vec<IPPool>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListIPPoolsResponse = IPPoolListResponse;

pub type ListIPPoolsItem = IPPool;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListIdentitiesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListIdentitiesQuery = ListIdentitiesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IdentityListResponse {
    #[serde(rename = "identities")]
    pub identities: Vec<Identity>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListIdentitiesResponse = IdentityListResponse;

pub type ListIdentitiesItem = Identity;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListReceivedMessagesParameters {
    #[serde(rename = "identity", default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,

    #[serde(rename = "recipient", default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,

    #[serde(
        rename = "review_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub review_reason: Option<ListReceivedMessagesParametersReviewReason>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListReceivedMessagesParametersReviewReason {
    Clear,
    Spam,
    SenderUnverified,
    Malware,
    Unknown(String),
}
impl ListReceivedMessagesParametersReviewReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Clear => "clear",
            Self::Spam => "spam",
            Self::SenderUnverified => "sender_unverified",
            Self::Malware => "malware",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListReceivedMessagesParametersReviewReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListReceivedMessagesParametersReviewReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "clear" => Self::Clear,
            "spam" => Self::Spam,
            "sender_unverified" => Self::SenderUnverified,
            "malware" => Self::Malware,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListReceivedMessagesQuery = ListReceivedMessagesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivedMessageListResponse {
    #[serde(rename = "messages")]
    pub messages: Vec<ReceivedMessage>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListReceivedMessagesResponse = ReceivedMessageListResponse;

pub type ListReceivedMessagesItem = ReceivedMessage;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRecipientRulesParameters {
    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListRecipientRulesQuery = ListRecipientRulesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RecipientRuleListResponse {
    #[serde(rename = "rules")]
    pub rules: Vec<RecipientRule>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListRecipientRulesResponse = RecipientRuleListResponse;

pub type ListRecipientRulesItem = RecipientRule;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRejectionsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "before", default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "identity", default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,

    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<ListRejectionsParametersReason>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListRejectionsParametersReason {
    ALLRECIPIENTSSUPPRESSED,
    IDENTITYSUSPENDED,
    IDENTITYNOTVERIFIED,
    FROMDOMAINMISMATCH,
    RATELIMIT,
    Unknown(String),
}
impl ListRejectionsParametersReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ALLRECIPIENTSSUPPRESSED => "ALL_RECIPIENTS_SUPPRESSED",
            Self::IDENTITYSUSPENDED => "IDENTITY_SUSPENDED",
            Self::IDENTITYNOTVERIFIED => "IDENTITY_NOT_VERIFIED",
            Self::FROMDOMAINMISMATCH => "FROM_DOMAIN_MISMATCH",
            Self::RATELIMIT => "RATE_LIMIT",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListRejectionsParametersReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListRejectionsParametersReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ALL_RECIPIENTS_SUPPRESSED" => Self::ALLRECIPIENTSSUPPRESSED,
            "IDENTITY_SUSPENDED" => Self::IDENTITYSUSPENDED,
            "IDENTITY_NOT_VERIFIED" => Self::IDENTITYNOTVERIFIED,
            "FROM_DOMAIN_MISMATCH" => Self::FROMDOMAINMISMATCH,
            "RATE_LIMIT" => Self::RATELIMIT,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListRejectionsQuery = ListRejectionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RejectionListResponse {
    #[serde(rename = "rejections")]
    pub rejections: Vec<Rejection>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Rejection {
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "identity", default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceRef>,

    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(rename = "to")]
    pub to: Vec<String>,

    #[serde(rename = "reason")]
    pub reason: RejectionReason,

    #[serde(rename = "detail", default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    #[serde(rename = "rejected_at")]
    pub rejected_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RejectionReason {
    ALLRECIPIENTSSUPPRESSED,
    IDENTITYSUSPENDED,
    IDENTITYNOTVERIFIED,
    FROMDOMAINMISMATCH,
    RATELIMIT,
    Unknown(String),
}
impl RejectionReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ALLRECIPIENTSSUPPRESSED => "ALL_RECIPIENTS_SUPPRESSED",
            Self::IDENTITYSUSPENDED => "IDENTITY_SUSPENDED",
            Self::IDENTITYNOTVERIFIED => "IDENTITY_NOT_VERIFIED",
            Self::FROMDOMAINMISMATCH => "FROM_DOMAIN_MISMATCH",
            Self::RATELIMIT => "RATE_LIMIT",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RejectionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RejectionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ALL_RECIPIENTS_SUPPRESSED" => Self::ALLRECIPIENTSSUPPRESSED,
            "IDENTITY_SUSPENDED" => Self::IDENTITYSUSPENDED,
            "IDENTITY_NOT_VERIFIED" => Self::IDENTITYNOTVERIFIED,
            "FROM_DOMAIN_MISMATCH" => Self::FROMDOMAINMISMATCH,
            "RATE_LIMIT" => Self::RATELIMIT,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListRejectionsResponse = RejectionListResponse;

pub type ListRejectionsItem = Rejection;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSentMessagesParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "identity", default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "recipient", default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

pub type ListSentMessagesQuery = ListSentMessagesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageListResponse {
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListSentMessagesResponse = MessageListResponse;

pub type ListSentMessagesItem = Message;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSuppressionsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<ListSuppressionsParametersReason>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListSuppressionsParametersReason {
    AutoBounce,
    AutoComplaint,
    Manual,
    Unknown(String),
}
impl ListSuppressionsParametersReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AutoBounce => "auto_bounce",
            Self::AutoComplaint => "auto_complaint",
            Self::Manual => "manual",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListSuppressionsParametersReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListSuppressionsParametersReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "auto_bounce" => Self::AutoBounce,
            "auto_complaint" => Self::AutoComplaint,
            "manual" => Self::Manual,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListSuppressionsQuery = ListSuppressionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SuppressionListResponse {
    #[serde(rename = "suppressions")]
    pub suppressions: Vec<Suppression>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListSuppressionsResponse = SuppressionListResponse;

pub type ListSuppressionsItem = Suppression;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListTemplatesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListTemplatesQuery = ListTemplatesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TemplateListResponse {
    #[serde(rename = "templates")]
    pub templates: Vec<Template>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListTemplatesResponse = TemplateListResponse;

pub type ListTemplatesItem = Template;

pub type RemoveSuppressionOverrideResponse = ConfigurationSetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RequestDedicatedIPRequestInput {
    /// UUID, CRN or account-unique pool name; empty leaves unassigned.
    #[serde(rename = "ip_pool", default, skip_serializing_if = "Option::is_none")]
    pub ip_pool: Option<String>,
}

pub type RequestDedicatedIPBody = RequestDedicatedIPRequestInput;

pub type RequestDedicatedIPResponse = DedicatedIPResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendMessageRequestInput {
    /// Bucket UUID, CRN or name for attachments embedded in raw MIME. Objects are written under email/{identity_id}/sent/{message_id}/. Separate attachments entries reference existing bucket objects.
    #[serde(
        rename = "attachments_bucket",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachments_bucket: Option<String>,
    /// Bucket objects to attach. GetObject permission is required; version and checksum are pinned at acceptance.
    #[serde(
        rename = "attachments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attachments: Option<Vec<AttachmentInputInput>>,
    /// UUID, CRN or account-unique name.
    #[serde(rename = "identity")]
    pub identity: String,

    #[serde(rename = "from")]
    pub from: String,

    #[serde(rename = "to")]
    pub to: Vec<String>,
    /// Subject for text/HTML content. For raw or template content, the subject is supplied by that content.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// UUID, CRN or account-unique name.
    #[serde(
        rename = "configuration_set",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_set: Option<String>,

    #[serde(rename = "content")]
    pub content: SendContentInput,
}
impl SendMessageRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(identity: String, from: String, to: Vec<String>, content: SendContentInput) -> Self {
        Self {
            attachments_bucket: None,
            attachments: None,
            identity,
            from,
            to,
            subject: None,
            configuration_set: None,
            content,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachmentInputInput {
    /// Same-account, same-region bucket name, UUID or CRN.
    #[serde(rename = "bucket")]
    pub bucket: String,

    #[serde(rename = "key")]
    pub key: String,
    /// Optional opaque Storage object version token.
    #[serde(
        rename = "version_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_id: Option<String>,
    /// Attachment filename.
    #[serde(rename = "name")]
    pub name: String,
    /// MIME type; defaults to the object content type.
    #[serde(
        rename = "content_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
}
impl AttachmentInputInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(bucket: String, key: String, name: String) -> Self {
        Self {
            bucket,
            key,
            version_id: None,
            name,
            content_type: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SendContentInput {
    Variant1(Box<SendContentInputVariant1>),
    Variant2(Box<SendContentInputVariant2>),
    Variant3(Box<SendContentInputVariant3>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SendContentInputVariant1 {
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "html", default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Complete RFC 5322 message. Its From header must belong to the authorized identity domain.
    #[serde(rename = "raw", default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    /// Template UUID, CRN or name; requires email:SendTemplatedMessage on that template.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(
        rename = "template_data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_data: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendContentInputVariant2 {
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "html", default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Complete RFC 5322 message. Its From header must belong to the authorized identity domain.
    #[serde(rename = "raw")]
    pub raw: String,
    /// Template UUID, CRN or name; requires email:SendTemplatedMessage on that template.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    #[serde(
        rename = "template_data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_data: Option<std::collections::BTreeMap<String, String>>,
}
impl SendContentInputVariant2 {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(raw: String) -> Self {
        Self {
            text: None,
            html: None,
            raw,
            template: None,
            template_data: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendContentInputVariant3 {
    #[serde(rename = "text", default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    #[serde(rename = "html", default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Complete RFC 5322 message. Its From header must belong to the authorized identity domain.
    #[serde(rename = "raw", default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    /// Template UUID, CRN or name; requires email:SendTemplatedMessage on that template.
    #[serde(rename = "template")]
    pub template: String,

    #[serde(
        rename = "template_data",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub template_data: Option<std::collections::BTreeMap<String, String>>,
}
impl SendContentInputVariant3 {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(template: String) -> Self {
        Self {
            text: None,
            html: None,
            raw: None,
            template,
            template_data: None,
        }
    }
}

pub type SendMessageBody = SendMessageRequestInput;

pub type SendMessageResponse = MessageResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchSendRequestInput {
    #[serde(rename = "messages")]
    pub messages: Vec<SendMessageRequestInput>,
}
impl BatchSendRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(messages: Vec<SendMessageRequestInput>) -> Self {
        Self { messages }
    }
}

pub type SendMessageBatchBody = BatchSendRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchSendResponse {
    #[serde(rename = "results")]
    pub results: Vec<BatchSendResult>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchSendResult {
    #[serde(rename = "index")]
    pub index: i64,

    #[serde(rename = "status")]
    pub status: BatchSendResultStatus,

    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<ResourceRef>,

    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BatchSendResultStatus {
    Queued,
    Rejected,
    Unknown(String),
}
impl BatchSendResultStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Queued => "queued",
            Self::Rejected => "rejected",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for BatchSendResultStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BatchSendResultStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "queued" => Self::Queued,
            "rejected" => Self::Rejected,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Error {
    /// Error code identifying the type of error
    #[serde(rename = "code")]
    pub code: String,
    /// Human-readable error message
    #[serde(rename = "message")]
    pub message: String,
    /// Optional non-sensitive values for interpolation of localized errors, keyed by error code. Never present for server errors.
    #[serde(rename = "params", default, skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Request ID for debugging
    #[serde(rename = "request_id")]
    pub request_id: String,
}

pub type SendMessageBatchResponse = BatchSendResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SetBIMIRequestInput {
    /// HTTPS SVG Tiny PS URL; null clears BIMI.
    #[serde(
        rename = "logo_url",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub logo_url: Option<crate::Nullable<String>>,
    /// Optional HTTPS VMC URL; omission preserves it and null removes the certificate.
    #[serde(
        rename = "vmc_url",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub vmc_url: Option<crate::Nullable<String>>,
}

pub type SetBIMIBody = SetBIMIRequestInput;

pub type SetBIMIResponse = IdentityResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SetMailFromDomainRequestInput {
    /// Mail-from domain; null clears the binding.
    #[serde(
        rename = "domain",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub domain: Option<crate::Nullable<String>>,
}

pub type SetMailFromDomainBody = SetMailFromDomainRequestInput;

pub type SetMailFromDomainResponse = IdentityResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateConfigurationSetRequestInput {
    #[serde(
        rename = "sending_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sending_enabled: Option<bool>,

    #[serde(
        rename = "suppression_overrides",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suppression_overrides: Option<Vec<String>>,
    /// UUID, CRN or account-unique name. Omit to preserve; null detaches the pool.
    #[serde(
        rename = "ip_pool",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub ip_pool: Option<crate::Nullable<String>>,

    #[serde(
        rename = "tracking_open",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tracking_open: Option<bool>,

    #[serde(
        rename = "tracking_click",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tracking_click: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateConfigurationSetBody = UpdateConfigurationSetRequestInput;

pub type UpdateConfigurationSetResponse = ConfigurationSetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateDedicatedIPRequestInput {
    #[serde(
        rename = "warmup_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warmup_status: Option<UpdateDedicatedIPRequestInputWarmupStatus>,
    /// UUID, CRN or account-unique pool name. Null detaches; omission leaves unchanged.
    #[serde(
        rename = "ip_pool",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub ip_pool: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateDedicatedIPRequestInputWarmupStatus {
    New,
    Warming,
    Warm,
    Unknown(String),
}
impl UpdateDedicatedIPRequestInputWarmupStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::New => "new",
            Self::Warming => "warming",
            Self::Warm => "warm",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UpdateDedicatedIPRequestInputWarmupStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UpdateDedicatedIPRequestInputWarmupStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "new" => Self::New,
            "warming" => Self::Warming,
            "warm" => Self::Warm,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateDedicatedIPBody = UpdateDedicatedIPRequestInput;

pub type UpdateDedicatedIPResponse = DedicatedIPResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateIPPoolRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

pub type UpdateIPPoolBody = UpdateIPPoolRequestInput;

pub type UpdateIPPoolResponse = IPPoolResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateIdentityRequestInput {
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateIdentityBody = UpdateIdentityRequestInput;

pub type UpdateIdentityResponse = IdentityResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateReceivingConfigurationRequestInput {
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Same-account, same-region bucket name, UUID or CRN. Omission preserves the current bucket; null clears it while receiving is disabled. Saving requires storage PutObject and GetObject permission under email/{identity_id}/ in this bucket. Changing the bucket affects new messages only.
    #[serde(
        rename = "attachments_bucket",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub attachments_bucket: Option<crate::Nullable<String>>,
}

pub type UpdateReceivingConfigurationBody = UpdateReceivingConfigurationRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateRecipientRuleRequestInput {
    /// Exact local part, local-part prefix, or catch-all for this identity. Exact matches take precedence, then the longest prefix, then catch-all.
    #[serde(
        rename = "match_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub match_type: Option<UpdateRecipientRuleRequestInputMatchType>,
    /// Required for exact and prefix; empty or omitted for all.
    #[serde(
        rename = "local_part",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_part: Option<String>,
    /// Notifications topic name, UUID or CRN. Requires notifications:Publish on the same-account topic.
    #[serde(rename = "topic", default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateRecipientRuleRequestInputMatchType {
    Exact,
    Prefix,
    All,
    Unknown(String),
}
impl UpdateRecipientRuleRequestInputMatchType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
            Self::Prefix => "prefix",
            Self::All => "all",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UpdateRecipientRuleRequestInputMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRecipientRuleRequestInputMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "exact" => Self::Exact,
            "prefix" => Self::Prefix,
            "all" => Self::All,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateRecipientRuleBody = UpdateRecipientRuleRequestInput;

pub type UpdateRecipientRuleResponse = RecipientRuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateSuppressionRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Null makes the suppression permanent; omission preserves its expiry.
    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub expires_at: Option<crate::Nullable<String>>,
}

pub type UpdateSuppressionBody = UpdateSuppressionRequestInput;

pub type UpdateSuppressionResponse = SuppressionResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateTemplateRequestInput {
    #[serde(
        rename = "subject_template",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_template: Option<String>,

    #[serde(
        rename = "body_template",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub body_template: Option<String>,

    #[serde(rename = "body_type", default, skip_serializing_if = "Option::is_none")]
    pub body_type: Option<UpdateTemplateRequestInputBodyType>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateTemplateRequestInputBodyType {
    Text,
    Html,
    Unknown(String),
}
impl UpdateTemplateRequestInputBodyType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Text => "text",
            Self::Html => "html",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UpdateTemplateRequestInputBodyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UpdateTemplateRequestInputBodyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "text" => Self::Text,
            "html" => Self::Html,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateTemplateBody = UpdateTemplateRequestInput;

pub type UpdateTemplateResponse = TemplateResponse;

pub type VerifyBIMIResponse = IdentityResponse;

pub type VerifyIdentityResponse = IdentityResponse;

pub type VerifyMailFromDomainResponse = IdentityResponse;
