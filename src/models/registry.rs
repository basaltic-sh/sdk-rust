//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BatchDeleteImageRequestInput {
    /// Removes the manifest AND every tag pointing at it.
    #[serde(rename = "digests", default, skip_serializing_if = "Option::is_none")]
    pub digests: Option<Vec<String>>,
    /// Removes only the tag. The manifest survives, untagged.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

pub type BatchDeleteImageBody = BatchDeleteImageRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchDeleteResult {
    #[serde(rename = "deleted")]
    pub deleted: Vec<BatchDeleteResultDeletedItem>,

    #[serde(rename = "failed")]
    pub failed: Vec<BatchDeleteResultFailedItem>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchDeleteResultDeletedItem {
    #[serde(rename = "digest")]
    pub digest: String,
    /// The tags the delete took with it. Surfaced because deleting by digest silently removes every tag pointing at it, and a caller who meant to remove one needs to see the rest go.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchDeleteResultFailedItem {
    #[serde(rename = "digest", default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,

    #[serde(rename = "tag", default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "reason")]
    pub reason: String,
}

pub type BatchDeleteImageResponse = BatchDeleteResult;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CreatePullThroughCacheRuleRequestInput {
    /// One or two lowercase repository-name components. It becomes the leading path of every repository the rule creates, so it has to be a name a client can pull.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// A hostname with an optional port. `https://` is accepted and stripped; anything else is refused, because the connection can carry a credential.
    #[serde(rename = "upstream_registry")]
    pub upstream_registry: String,
    /// Optional path prepended upstream.
    #[serde(
        rename = "upstream_prefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_prefix: Option<String>,
    /// Optional. Worth setting even against a public registry: an authenticated Docker Hub pull draws on a far larger rate-limit budget than an anonymous one.
    #[serde(
        rename = "upstream_username",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_username: Option<String>,
    /// The password or token. Write-only — sealed at rest and never returned. Required with `upstream_username`, and refused without it: a half-specified credential produces an anonymous pull that silently draws on the small budget.
    #[serde(
        rename = "upstream_secret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_secret: Option<String>,
}
impl std::fmt::Debug for CreatePullThroughCacheRuleRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CreatePullThroughCacheRuleRequestInput");
        d.field("prefix", &self.prefix);
        d.field("upstream_registry", &self.upstream_registry);
        d.field("upstream_prefix", &self.upstream_prefix);
        d.field("upstream_username", &self.upstream_username);
        d.field("upstream_secret", &"[REDACTED]");
        d.finish()
    }
}
impl CreatePullThroughCacheRuleRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(prefix: String, upstream_registry: String) -> Self {
        Self {
            prefix,
            upstream_registry,
            upstream_prefix: None,
            upstream_username: None,
            upstream_secret: None,
        }
    }
}

pub type CreatePullThroughCacheRuleBody = CreatePullThroughCacheRuleRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PullThroughCacheRuleResponse {
    #[serde(rename = "pull_through_cache_rule")]
    pub pull_through_cache_rule: PullThroughCacheRule,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PullThroughCacheRule {
    #[serde(rename = "crn")]
    pub crn: String,
    /// The leading path component(s) of the repository names this rule claims.
    #[serde(rename = "prefix")]
    pub prefix: String,
    /// The pullable reference prefix the rule creates. Returned rather than assembled by the client, for the same reason a repository's `uri` is: the account handle's position in the path is the only thing separating two tenants' namespaces.
    #[serde(rename = "uri_prefix")]
    pub uri_prefix: String,
    /// The upstream host. `docker.io` is stored as `registry-1.docker.io`, which is the host that actually speaks `/v2` — the docker daemon makes the same rewrite internally.
    #[serde(rename = "upstream_registry")]
    pub upstream_registry: String,
    /// Prepended to the rest of the repository name upstream. `library` is what makes `hub/nginx` resolve to Docker Hub's `library/nginx`.
    #[serde(
        rename = "upstream_prefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_prefix: Option<String>,

    #[serde(
        rename = "upstream_username",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_username: Option<String>,
    /// Whether a secret is stored. The secret itself is never returned; rotation is a write of the new value.
    #[serde(rename = "has_credential")]
    pub has_credential: bool,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}
impl std::fmt::Debug for PullThroughCacheRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("PullThroughCacheRule");
        d.field("crn", &self.crn);
        d.field("prefix", &self.prefix);
        d.field("uri_prefix", &self.uri_prefix);
        d.field("upstream_registry", &self.upstream_registry);
        d.field("upstream_prefix", &self.upstream_prefix);
        d.field("upstream_username", &self.upstream_username);
        d.field("has_credential", &"[REDACTED]");
        d.field("created_at", &self.created_at);
        d.field("updated_at", &self.updated_at);
        d.finish()
    }
}

pub type CreatePullThroughCacheRuleResponse = PullThroughCacheRuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateRepositoryRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "tag_mutability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_mutability: Option<CreateRepositoryRequestInputTagMutability>,

    #[serde(
        rename = "deletion_protection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_protection: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl CreateRepositoryRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            tag_mutability: None,
            deletion_protection: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateRepositoryRequestInputTagMutability {
    Mutable,
    Immutable,
    Unknown(String),
}
impl CreateRepositoryRequestInputTagMutability {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mutable => "mutable",
            Self::Immutable => "immutable",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateRepositoryRequestInputTagMutability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateRepositoryRequestInputTagMutability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "mutable" => Self::Mutable,
            "immutable" => Self::Immutable,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateRepositoryBody = CreateRepositoryRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryResponse {
    #[serde(rename = "repository")]
    pub repository: Repository,
    /// The login/build/push sequence for this repository. Rendered server-side because the login username, the endpoint, and the account handle's position in the reference all come from service configuration.
    #[serde(
        rename = "push_commands",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub push_commands: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Repository {
    #[serde(rename = "id")]
    pub id: String,
    /// The IAM resource name. Built from the repository NAME, not its id, so a policy naming a repository keeps applying after a delete and re-create under the same name — which matters here more than elsewhere, because the name is what every deployment manifest pulls.
    #[serde(rename = "crn")]
    pub crn: String,
    /// Lowercase alphanumeric path components separated by a single period, one or two underscores, or hyphens, joined by slashes. This is the OCI grammar — anything else is not pushable, so a laxer rule would only produce repositories nobody can use. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// The pullable reference prefix. Returned rather than left for clients to assemble: the account handle's position in the path is the only thing separating two tenants' repositories of the same name.
    #[serde(rename = "uri")]
    pub uri: String,
    /// `immutable` refuses to move a tag that already resolves, so a deployed digest cannot be swapped under a running fleet. Pushing a tag that does not exist yet still works — the guarantee is that a tag never changes meaning, not that the tag set is frozen. A repeat push of the identical manifest is always allowed.
    #[serde(rename = "tag_mutability")]
    pub tag_mutability: RepositoryTagMutability,
    /// Refuses `DeleteRepository` while set, even for an empty repository and even with `force`.
    #[serde(rename = "deletion_protection")]
    pub deletion_protection: bool,
    /// AWS-style resource tags. Feed the `basalt:ResourceTag/*` policy conditions.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "stats", default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<RepositoryStats>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepositoryTagMutability {
    Mutable,
    Immutable,
    Unknown(String),
}
impl RepositoryTagMutability {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mutable => "mutable",
            Self::Immutable => "immutable",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RepositoryTagMutability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryTagMutability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "mutable" => Self::Mutable,
            "immutable" => Self::Immutable,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RepositoryStats {
    #[serde(
        rename = "image_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_count: Option<i64>,

    #[serde(rename = "tag_count", default, skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i64>,
    /// The sum of the DISTINCT blobs this repository's manifests reference. Not the sum of its images' sizes: ten tags of one image share their layers, and adding the per-image figures would report ten times the space occupied.
    #[serde(
        rename = "size_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub size_bytes: Option<i64>,
    /// Null for a repository that has never received an image.
    #[serde(
        rename = "last_pushed_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_pushed_at: Option<crate::Nullable<String>>,
}

pub type CreateRepositoryResponse = RepositoryResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DeleteRepositoryParameters {
    #[serde(rename = "force", default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

pub type DeleteRepositoryQuery = DeleteRepositoryParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageResponse {
    #[serde(rename = "image")]
    pub image: Image,
    /// The manifest document, parsed. NOT suitable for verifying the digest — that covers the exact bytes pushed, and a round-trip through a JSON encoder reorders keys. Fetch from `/v2` to re-hash.
    #[serde(rename = "manifest", default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Image {
    /// The image's row id — the pagination cursor for the list endpoint.
    #[serde(rename = "id")]
    pub id: String,
    /// The sha256 of the manifest document's exact bytes.
    #[serde(rename = "digest")]
    pub digest: String,
    /// The OCI or Docker v2 media type. Both vocabularies are stored as pushed: containerd emits OCI, older `docker push` emits Docker v2, and they are the same documents with different labels.
    #[serde(rename = "media_type")]
    pub media_type: String,
    /// Every tag pointing at this manifest. Empty for an untagged image.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// What kind of thing this is. Distinguishes a container image from a Helm chart, an SBOM, or a signature — all of which are stored as manifests and would otherwise be indistinguishable.
    #[serde(
        rename = "artifact_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub artifact_type: Option<String>,
    /// Absent on a multi-platform index, which has no config of its own.
    #[serde(
        rename = "config_digest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub config_digest: Option<String>,
    /// The digest this manifest is *about*, set on a signature, SBOM, or attestation. What the OCI referrers API resolves.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The manifest document itself.
    #[serde(
        rename = "manifest_size_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub manifest_size_bytes: Option<i64>,
    /// The sum of the distinct blobs this manifest references — what a customer means by "how big is this image".
    #[serde(rename = "size_bytes")]
    pub size_bytes: i64,
    /// True for a multi-platform image index or Docker manifest list.
    #[serde(rename = "is_index")]
    pub is_index: bool,

    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    /// Coarsened to a minute. A fleet of 500 nodes pulling one image in a few seconds would otherwise be 500 row updates contending on one page, and the lifecycle rule reading this measures in days.
    #[serde(
        rename = "last_pulled_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_pulled_at: Option<crate::Nullable<String>>,
}

pub type DescribeImageResponse = ImageResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DescribeImagesParameters {
    #[serde(
        rename = "tag_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_status: Option<DescribeImagesParametersTagStatus>,

    #[serde(
        rename = "tag_prefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_prefix: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DescribeImagesParametersTagStatus {
    Tagged,
    Untagged,
    Any,
    Unknown(String),
}
impl DescribeImagesParametersTagStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tagged => "tagged",
            Self::Untagged => "untagged",
            Self::Any => "any",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for DescribeImagesParametersTagStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DescribeImagesParametersTagStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "tagged" => Self::Tagged,
            "untagged" => Self::Untagged,
            "any" => Self::Any,
            _ => Self::Unknown(value),
        })
    }
}

pub type DescribeImagesQuery = DescribeImagesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageListResponse {
    #[serde(rename = "images")]
    pub images: Vec<Image>,

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

pub type DescribeImagesResponse = ImageListResponse;

pub type DescribePullThroughCacheRuleResponse = PullThroughCacheRuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DescribePullThroughCacheRulesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type DescribePullThroughCacheRulesQuery = DescribePullThroughCacheRulesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PullThroughCacheRuleListResponse {
    #[serde(rename = "pull_through_cache_rules")]
    pub pull_through_cache_rules: Vec<PullThroughCacheRule>,
}

pub type DescribePullThroughCacheRulesResponse = PullThroughCacheRuleListResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DescribeRepositoriesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type DescribeRepositoriesQuery = DescribeRepositoriesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryListResponse {
    #[serde(rename = "repositories")]
    pub repositories: Vec<Repository>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type DescribeRepositoriesResponse = RepositoryListResponse;

pub type DescribeRepositoryResponse = RepositoryResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthorizationTokenResponse {
    #[serde(rename = "authorization_data")]
    pub authorization_data: AuthorizationTokenResponseAuthorizationData,
}
impl std::fmt::Debug for AuthorizationTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AuthorizationTokenResponse");
        d.field("authorization_data", &"[REDACTED]");
        d.finish()
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthorizationTokenResponseAuthorizationData {
    /// `base64("<username>:<password>")` — the single opaque blob ECR-shaped tooling expects.
    #[serde(rename = "authorization_token")]
    pub authorization_token: String,
    /// Fixed, and ignored on the way in: the token identifies the principal. Present so `docker login -u` has something to pass.
    #[serde(rename = "username")]
    pub username: String,
    /// The credential itself, for `docker login --password-stdin`.
    #[serde(rename = "password")]
    pub password: String,

    #[serde(rename = "endpoint")]
    pub endpoint: String,

    #[serde(rename = "expires_at")]
    pub expires_at: String,
}
impl std::fmt::Debug for AuthorizationTokenResponseAuthorizationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AuthorizationTokenResponseAuthorizationData");
        d.field("authorization_token", &"[REDACTED]");
        d.field("username", &self.username);
        d.field("password", &"[REDACTED]");
        d.field("endpoint", &self.endpoint);
        d.field("expires_at", &self.expires_at);
        d.finish()
    }
}

pub type GetAuthorizationTokenResponse = AuthorizationTokenResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfigurationResponse {
    #[serde(rename = "encryption_configuration")]
    pub encryption_configuration: EncryptionConfiguration,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfiguration {
    /// `platform_managed` wraps each blob's data key under a platform key the customer does not manage. `kms` wraps it under a key in the account's own KMS, so disabling that key stops the account's layers from being readable. Reported as a named type rather than a boolean so a future third posture has somewhere to go.
    #[serde(rename = "encryption_type")]
    pub encryption_type: EncryptionConfigurationEncryptionType,
    /// Customer-managed key reference. Omitted under platform encryption. A deleted key retains its ID and CRN without a name; existing layers still require that key.
    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<ResourceRef>,
    /// When the setting last changed. Absent on an account that has never configured encryption — there is no record, and reporting the current time would read as a change that never happened.
    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EncryptionConfigurationEncryptionType {
    PlatformManaged,
    Kms,
    Unknown(String),
}
impl EncryptionConfigurationEncryptionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PlatformManaged => "platform_managed",
            Self::Kms => "kms",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for EncryptionConfigurationEncryptionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EncryptionConfigurationEncryptionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "platform_managed" => Self::PlatformManaged,
            "kms" => Self::Kms,
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

pub type GetEncryptionConfigurationResponse = EncryptionConfigurationResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecyclePolicyResponse {
    #[serde(rename = "policy")]
    pub policy: LifecyclePolicyResponsePolicy,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecyclePolicyResponsePolicy {
    #[serde(rename = "rules")]
    pub rules: Vec<LifecycleRule>,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Null for a policy the worker has not reached yet. Surfaced so a customer whose images are not expiring can tell "never ran" from "ran and matched nothing".
    #[serde(
        rename = "last_evaluated_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_evaluated_at: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleRule {
    /// Evaluation order, ascending, unique within the policy. An image is claimed by the first matching rule.
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "selection")]
    pub selection: LifecycleSelection,
    /// The only action there is. A field rather than an implication so the document has room for a future one without a policy written today meaning something different tomorrow.
    #[serde(rename = "action")]
    pub action: LifecycleRuleAction,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleSelection {
    /// `tagged` requires a prefix or pattern. `any` may appear on at most one rule and must carry the highest priority number.
    #[serde(rename = "tag_status")]
    pub tag_status: LifecycleSelectionTagStatus,

    #[serde(
        rename = "tag_prefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_prefixes: Option<Vec<String>>,
    /// A glob whose only wildcard is `*`. A bare `*` is refused.
    #[serde(
        rename = "tag_patterns",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_patterns: Option<Vec<String>>,
    /// `image_count_more_than` keeps the N newest matching images. `since_image_pushed` expires those older than N days. `since_image_pulled` expires those not pulled in N days, measuring a never-pulled image from its push time — otherwise a never-pulled image would be immortal, which is the opposite of what the rule is for.
    #[serde(rename = "count_type")]
    pub count_type: LifecycleSelectionCountType,
    /// Required by the two `since_*` types, forbidden by the other.
    #[serde(
        rename = "count_unit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_unit: Option<LifecycleSelectionCountUnit>,

    #[serde(rename = "count_number")]
    pub count_number: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleSelectionTagStatus {
    Tagged,
    Untagged,
    Any,
    Unknown(String),
}
impl LifecycleSelectionTagStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tagged => "tagged",
            Self::Untagged => "untagged",
            Self::Any => "any",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleSelectionTagStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleSelectionTagStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "tagged" => Self::Tagged,
            "untagged" => Self::Untagged,
            "any" => Self::Any,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleSelectionCountType {
    ImageCountMoreThan,
    SinceImagePushed,
    SinceImagePulled,
    Unknown(String),
}
impl LifecycleSelectionCountType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ImageCountMoreThan => "image_count_more_than",
            Self::SinceImagePushed => "since_image_pushed",
            Self::SinceImagePulled => "since_image_pulled",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleSelectionCountType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleSelectionCountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "image_count_more_than" => Self::ImageCountMoreThan,
            "since_image_pushed" => Self::SinceImagePushed,
            "since_image_pulled" => Self::SinceImagePulled,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleSelectionCountUnit {
    Days,
    Unknown(String),
}
impl LifecycleSelectionCountUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Days => "days",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleSelectionCountUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleSelectionCountUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "days" => Self::Days,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleRuleAction {
    Expire,
    Unknown(String),
}
impl LifecycleRuleAction {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Expire => "expire",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleRuleAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRuleAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "expire" => Self::Expire,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetLifecyclePolicyResponse = LifecyclePolicyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryPolicyResponse {
    #[serde(rename = "policy")]
    pub policy: RepositoryPolicyResponsePolicy,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryPolicyResponsePolicy {
    #[serde(rename = "document")]
    pub document: RepositoryPolicyDocument,

    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryPolicyDocument {
    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "statements")]
    pub statements: Vec<RepositoryPolicyDocumentStatementsItem>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryPolicyDocumentStatementsItem {
    #[serde(rename = "sid", default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    #[serde(rename = "effect")]
    pub effect: RepositoryPolicyDocumentStatementsItemEffect,
    /// Account CRNs, an org CRN (`crn:iam:::org/<org-id>`), or `*` for the public.
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// Must all be `registry:` actions. A repository's owner cannot grant `storage:GetObject` by writing it here, and ignoring such a statement would leave a document that reads as a grant and is not one.
    #[serde(rename = "actions")]
    pub actions: Vec<String>,
    /// Must be this repository's own CRN, or the bare `*` — the same thing here, since the document is only ever evaluated against this repository.
    #[serde(rename = "resources")]
    pub resources: Vec<String>,

    #[serde(
        rename = "conditions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions: Option<Vec<PolicyCondition>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepositoryPolicyDocumentStatementsItemEffect {
    Allow,
    Deny,
    Unknown(String),
}
impl RepositoryPolicyDocumentStatementsItemEffect {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RepositoryPolicyDocumentStatementsItemEffect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryPolicyDocumentStatementsItemEffect {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "allow" => Self::Allow,
            "deny" => Self::Deny,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyCondition {
    #[serde(rename = "operator")]
    pub operator: String,

    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "values")]
    pub values: Vec<String>,

    #[serde(
        rename = "set_operator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub set_operator: Option<PolicyConditionSetOperator>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyConditionSetOperator {
    ForAllValues,
    ForAnyValue,
    Unknown(String),
}
impl PolicyConditionSetOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ForAllValues => "for_all_values",
            Self::ForAnyValue => "for_any_value",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyConditionSetOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyConditionSetOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "for_all_values" => Self::ForAllValues,
            "for_any_value" => Self::ForAnyValue,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetRepositoryPolicyResponse = RepositoryPolicyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetRetainedRegistryPresenceResult {
    #[serde(rename = "has_resources")]
    pub has_resources: bool,
}

pub type GetRetainedRegistryPresenceResponse = GetRetainedRegistryPresenceResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListTagsParameters {
    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListTagsQuery = ListTagsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TagListResponse {
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "digest")]
    pub digest: String,

    #[serde(rename = "created_at")]
    pub created_at: String,
    /// When the tag last moved. Equal to created_at for a tag that never has.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type ListTagsResponse = TagListResponse;

pub type ListTagsItem = Tag;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfigurationRequestInput {
    /// Required. Inferring it from whether `kms_key` is present would make "move me back to the platform key" indistinguishable from a client that forgot the field.
    #[serde(rename = "encryption_type")]
    pub encryption_type: EncryptionConfigurationRequestInputEncryptionType,
    /// KMS key UUID, CRN or account-scoped name. Required with `kms`, and refused with `platform_managed` rather than ignored — the two together are contradictory, and guessing which was meant could leave content under a key the caller did not intend.
    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}
impl EncryptionConfigurationRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(encryption_type: EncryptionConfigurationRequestInputEncryptionType) -> Self {
        Self {
            encryption_type,
            kms_key: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EncryptionConfigurationRequestInputEncryptionType {
    PlatformManaged,
    Kms,
    Unknown(String),
}
impl EncryptionConfigurationRequestInputEncryptionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PlatformManaged => "platform_managed",
            Self::Kms => "kms",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for EncryptionConfigurationRequestInputEncryptionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EncryptionConfigurationRequestInputEncryptionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "platform_managed" => Self::PlatformManaged,
            "kms" => Self::Kms,
            _ => Self::Unknown(value),
        })
    }
}

pub type PutEncryptionConfigurationBody = EncryptionConfigurationRequestInput;

pub type PutEncryptionConfigurationResponse = EncryptionConfigurationResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecyclePolicyRequestInput {
    #[serde(rename = "rules")]
    pub rules: Vec<LifecycleRuleInput>,
}
impl LifecyclePolicyRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(rules: Vec<LifecycleRuleInput>) -> Self {
        Self { rules }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleRuleInput {
    /// Evaluation order, ascending, unique within the policy. An image is claimed by the first matching rule.
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "selection")]
    pub selection: LifecycleSelectionInput,
    /// The only action there is. A field rather than an implication so the document has room for a future one without a policy written today meaning something different tomorrow.
    #[serde(rename = "action")]
    pub action: LifecycleRuleInputAction,
}
impl LifecycleRuleInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        priority: i64,
        selection: LifecycleSelectionInput,
        action: LifecycleRuleInputAction,
    ) -> Self {
        Self {
            priority,
            description: None,
            selection,
            action,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleSelectionInput {
    /// `tagged` requires a prefix or pattern. `any` may appear on at most one rule and must carry the highest priority number.
    #[serde(rename = "tag_status")]
    pub tag_status: LifecycleSelectionInputTagStatus,

    #[serde(
        rename = "tag_prefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_prefixes: Option<Vec<String>>,
    /// A glob whose only wildcard is `*`. A bare `*` is refused.
    #[serde(
        rename = "tag_patterns",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_patterns: Option<Vec<String>>,
    /// `image_count_more_than` keeps the N newest matching images. `since_image_pushed` expires those older than N days. `since_image_pulled` expires those not pulled in N days, measuring a never-pulled image from its push time — otherwise a never-pulled image would be immortal, which is the opposite of what the rule is for.
    #[serde(rename = "count_type")]
    pub count_type: LifecycleSelectionInputCountType,
    /// Required by the two `since_*` types, forbidden by the other.
    #[serde(
        rename = "count_unit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub count_unit: Option<LifecycleSelectionInputCountUnit>,

    #[serde(rename = "count_number")]
    pub count_number: i64,
}
impl LifecycleSelectionInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tag_status: LifecycleSelectionInputTagStatus,
        count_type: LifecycleSelectionInputCountType,
        count_number: i64,
    ) -> Self {
        Self {
            tag_status,
            tag_prefixes: None,
            tag_patterns: None,
            count_type,
            count_unit: None,
            count_number,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleSelectionInputTagStatus {
    Tagged,
    Untagged,
    Any,
    Unknown(String),
}
impl LifecycleSelectionInputTagStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tagged => "tagged",
            Self::Untagged => "untagged",
            Self::Any => "any",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleSelectionInputTagStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleSelectionInputTagStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "tagged" => Self::Tagged,
            "untagged" => Self::Untagged,
            "any" => Self::Any,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleSelectionInputCountType {
    ImageCountMoreThan,
    SinceImagePushed,
    SinceImagePulled,
    Unknown(String),
}
impl LifecycleSelectionInputCountType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ImageCountMoreThan => "image_count_more_than",
            Self::SinceImagePushed => "since_image_pushed",
            Self::SinceImagePulled => "since_image_pulled",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleSelectionInputCountType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleSelectionInputCountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "image_count_more_than" => Self::ImageCountMoreThan,
            "since_image_pushed" => Self::SinceImagePushed,
            "since_image_pulled" => Self::SinceImagePulled,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleSelectionInputCountUnit {
    Days,
    Unknown(String),
}
impl LifecycleSelectionInputCountUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Days => "days",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleSelectionInputCountUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleSelectionInputCountUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "days" => Self::Days,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleRuleInputAction {
    Expire,
    Unknown(String),
}
impl LifecycleRuleInputAction {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Expire => "expire",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleRuleInputAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRuleInputAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "expire" => Self::Expire,
            _ => Self::Unknown(value),
        })
    }
}

pub type PutLifecyclePolicyBody = LifecyclePolicyRequestInput;

pub type PutLifecyclePolicyResponse = LifecyclePolicyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryPolicyRequestInput {
    #[serde(rename = "document")]
    pub document: RepositoryPolicyDocumentInput,
}
impl RepositoryPolicyRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(document: RepositoryPolicyDocumentInput) -> Self {
        Self { document }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryPolicyDocumentInput {
    #[serde(rename = "version")]
    pub version: String,

    #[serde(rename = "statements")]
    pub statements: Vec<RepositoryPolicyDocumentInputStatementsItem>,
}
impl RepositoryPolicyDocumentInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        version: String,
        statements: Vec<RepositoryPolicyDocumentInputStatementsItem>,
    ) -> Self {
        Self {
            version,
            statements,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RepositoryPolicyDocumentInputStatementsItem {
    #[serde(rename = "sid", default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    #[serde(rename = "effect")]
    pub effect: RepositoryPolicyDocumentInputStatementsItemEffect,
    /// Account CRNs, an org CRN (`crn:iam:::org/<org-id>`), or `*` for the public.
    #[serde(rename = "principals")]
    pub principals: Vec<String>,
    /// Must all be `registry:` actions. A repository's owner cannot grant `storage:GetObject` by writing it here, and ignoring such a statement would leave a document that reads as a grant and is not one.
    #[serde(rename = "actions")]
    pub actions: Vec<String>,
    /// Must be this repository's own CRN, or the bare `*` — the same thing here, since the document is only ever evaluated against this repository.
    #[serde(rename = "resources")]
    pub resources: Vec<String>,

    #[serde(
        rename = "conditions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions: Option<Vec<PolicyConditionInput>>,
}
impl RepositoryPolicyDocumentInputStatementsItem {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        effect: RepositoryPolicyDocumentInputStatementsItemEffect,
        principals: Vec<String>,
        actions: Vec<String>,
        resources: Vec<String>,
    ) -> Self {
        Self {
            sid: None,
            effect,
            principals,
            actions,
            resources,
            conditions: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepositoryPolicyDocumentInputStatementsItemEffect {
    Allow,
    Deny,
    Unknown(String),
}
impl RepositoryPolicyDocumentInputStatementsItemEffect {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RepositoryPolicyDocumentInputStatementsItemEffect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryPolicyDocumentInputStatementsItemEffect {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "allow" => Self::Allow,
            "deny" => Self::Deny,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyConditionInput {
    #[serde(rename = "operator")]
    pub operator: String,

    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "values")]
    pub values: Vec<String>,

    #[serde(
        rename = "set_operator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub set_operator: Option<PolicyConditionInputSetOperator>,
}
impl PolicyConditionInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(operator: String, key: String, values: Vec<String>) -> Self {
        Self {
            operator,
            key,
            values,
            set_operator: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyConditionInputSetOperator {
    ForAllValues,
    ForAnyValue,
    Unknown(String),
}
impl PolicyConditionInputSetOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ForAllValues => "for_all_values",
            Self::ForAnyValue => "for_any_value",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyConditionInputSetOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyConditionInputSetOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "for_all_values" => Self::ForAllValues,
            "for_any_value" => Self::ForAnyValue,
            _ => Self::Unknown(value),
        })
    }
}

pub type SetRepositoryPolicyBody = RepositoryPolicyRequestInput;

pub type SetRepositoryPolicyResponse = RepositoryPolicyResponse;

pub type StartLifecyclePolicyPreviewBody = LifecyclePolicyRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecyclePreviewResponse {
    #[serde(rename = "preview")]
    pub preview: LifecyclePreviewResponsePreview,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecyclePreviewResponsePreview {
    #[serde(rename = "matches")]
    pub matches: Vec<LifecyclePreviewResponsePreviewMatchesItem>,
    /// How many images the repository holds, so 400 matches can be read against the 402 that exist.
    #[serde(rename = "total_images")]
    pub total_images: i64,
    /// What the expiry would reclaim: only layers no surviving image still references, each counted once.
    #[serde(rename = "size_bytes")]
    pub size_bytes: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecyclePreviewResponsePreviewMatchesItem {
    #[serde(rename = "digest")]
    pub digest: String,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "pushed_at")]
    pub pushed_at: String,

    #[serde(
        rename = "last_pulled_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_pulled_at: Option<crate::Nullable<String>>,
    /// Which rule claimed this image.
    #[serde(rename = "rule_priority")]
    pub rule_priority: i64,
}

pub type StartLifecyclePolicyPreviewResponse = LifecyclePreviewResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdatePullThroughCacheRuleRequestInput {
    #[serde(
        rename = "upstream_registry",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_registry: Option<String>,

    #[serde(
        rename = "upstream_prefix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_prefix: Option<String>,

    #[serde(
        rename = "upstream_username",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_username: Option<String>,

    #[serde(
        rename = "upstream_secret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upstream_secret: Option<String>,
}
impl std::fmt::Debug for UpdatePullThroughCacheRuleRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("UpdatePullThroughCacheRuleRequestInput");
        d.field("upstream_registry", &self.upstream_registry);
        d.field("upstream_prefix", &self.upstream_prefix);
        d.field("upstream_username", &self.upstream_username);
        d.field("upstream_secret", &"[REDACTED]");
        d.finish()
    }
}

pub type UpdatePullThroughCacheRuleBody = UpdatePullThroughCacheRuleRequestInput;

pub type UpdatePullThroughCacheRuleResponse = PullThroughCacheRuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateRepositoryRequestInput {
    #[serde(
        rename = "tag_mutability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tag_mutability: Option<UpdateRepositoryRequestInputTagMutability>,

    #[serde(
        rename = "deletion_protection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_protection: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateRepositoryRequestInputTagMutability {
    Mutable,
    Immutable,
    Unknown(String),
}
impl UpdateRepositoryRequestInputTagMutability {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Mutable => "mutable",
            Self::Immutable => "immutable",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UpdateRepositoryRequestInputTagMutability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRepositoryRequestInputTagMutability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "mutable" => Self::Mutable,
            "immutable" => Self::Immutable,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateRepositoryBody = UpdateRepositoryRequestInput;

pub type UpdateRepositoryResponse = RepositoryResponse;
