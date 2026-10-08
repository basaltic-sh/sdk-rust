//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateSecretRequestInput {
    /// Unique within the calling account. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
    /// Base64 of the initial value bytes (1 byte - 64 KiB).
    #[serde(rename = "value")]
    pub value: String,

    #[serde(
        rename = "recovery_window_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_window_days: Option<i64>,
    /// UUID, CRN or account-scoped name of a customer-managed KMS key to encrypt this secret under. Omit to use the platform-managed default key. The key must be enabled and have encrypt/decrypt usage.
    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}
impl CreateSecretRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            description: None,
            tags: None,
            value,
            recovery_window_days: None,
            kms_key: None,
        }
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateSecretBody = CreateSecretRequestInput;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SecretResponse {
    #[serde(rename = "secret")]
    pub secret: Secret,
}
impl std::fmt::Debug for SecretResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SecretResponse");
        d.field("secret", &"[REDACTED]");
        d.finish()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Secret {
    #[serde(rename = "id")]
    pub id: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(rename = "crn")]
    pub crn: String,
    /// True when the bound key has been deleted. The CRN is omitted, but the binding remains encrypted under its original key identity; this does not select platform encryption or plaintext.
    #[serde(
        rename = "kms_key_unavailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_key_unavailable: Option<bool>,
    /// Customer-managed KMS key the secret is encrypted under. Omitted for platform encryption or when kms_key_unavailable is true.
    #[serde(
        rename = "kms_key_crn",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub kms_key_crn: Option<crate::Nullable<String>>,
    /// True when a platform service generated this value and reads it back to act on. You can read and delete a managed secret, but UpdateSecret and PutSecretValue answer 403 SECRET_PLATFORM_MANAGED.
    #[serde(rename = "managed")]
    pub managed: bool,

    #[serde(
        rename = "deleted_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub deleted_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "scheduled_purge_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub scheduled_purge_at: Option<crate::Nullable<String>>,
    /// Whole-day recovery window.
    #[serde(rename = "recovery_window_days")]
    pub recovery_window_days: i64,
    /// 0 if no version exists yet.
    #[serde(
        rename = "current_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_version: Option<i64>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type Tags = std::collections::BTreeMap<String, String>;

pub type CreateSecretResponse = SecretResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteSecretRequestInput {
    /// Override the secret's stored window. Omit to keep it.
    #[serde(
        rename = "recovery_window_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_window_days: Option<i64>,
}

pub type DeleteSecretBody = DeleteSecretRequestInput;

pub type DeleteSecretResponse = SecretResponse;

pub type DescribeSecretResponse = SecretResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetSecretValueParameters {
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

pub type GetSecretValueQuery = GetSecretValueParameters;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SecretValueResponse {
    #[serde(rename = "secret")]
    pub secret: SecretValue,
}
impl std::fmt::Debug for SecretValueResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SecretValueResponse");
        d.field("secret", &"[REDACTED]");
        d.finish()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SecretValue {
    #[serde(rename = "secret_id")]
    pub secret_id: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "version")]
    pub version: i64,
    /// Base64 of the plaintext bytes.
    #[serde(rename = "value")]
    pub value: String,

    #[serde(rename = "created_at")]
    pub created_at: String,
}
impl std::fmt::Debug for SecretValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SecretValue");
        d.field("secret_id", &"[REDACTED]");
        d.field("name", &self.name);
        d.field("version", &self.version);
        d.field("value", &self.value);
        d.field("created_at", &self.created_at);
        d.finish()
    }
}

pub type GetSecretValueResponse = SecretValueResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSecretsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(
        rename = "include_deleted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_deleted: Option<bool>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListSecretsQuery = ListSecretsParameters;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SecretListResponse {
    #[serde(rename = "secrets")]
    pub secrets: Vec<Secret>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}
impl std::fmt::Debug for SecretListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SecretListResponse");
        d.field("secrets", &"[REDACTED]");
        d.field("meta", &self.meta);
        d.finish()
    }
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

pub type ListSecretsResponse = SecretListResponse;

pub type ListSecretsItem = Secret;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListVersionsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListVersionsQuery = ListVersionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VersionListResponse {
    #[serde(rename = "versions")]
    pub versions: Vec<SecretVersion>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecretVersion {
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "version")]
    pub version: i64,

    #[serde(rename = "is_current")]
    pub is_current: bool,
    /// CRN of the principal that created this version (e.g. crn:iam:::user/&lt;id&gt;, crn:iam:::service-account/&lt;id&gt;).
    #[serde(
        rename = "created_by",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_by: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

pub type ListVersionsResponse = VersionListResponse;

pub type ListVersionsItem = SecretVersion;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutSecretValueRequestInput {
    /// Base64 of the new value bytes (1 byte - 64 KiB).
    #[serde(rename = "value")]
    pub value: String,
}
impl PutSecretValueRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

pub type PutSecretValueBody = PutSecretValueRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VersionResponse {
    #[serde(rename = "version")]
    pub version: SecretVersion,
}

pub type PutSecretValueResponse = VersionResponse;

pub type RestoreSecretResponse = SecretResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateSecretRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateSecretBody = UpdateSecretRequestInput;

pub type UpdateSecretResponse = SecretResponse;
