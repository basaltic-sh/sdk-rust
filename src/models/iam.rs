//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssumeRoleRequestInput {
    #[serde(rename = "role")]
    pub role: RoleReferenceInput,
    /// Credential validity duration (15 min to 12 hours)
    #[serde(
        rename = "duration_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_seconds: Option<i64>,

    #[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<SessionPolicyDocumentInput>,
}
impl AssumeRoleRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(role: RoleReferenceInput) -> Self {
        Self {
            role,
            duration_seconds: None,
            policy: None,
        }
    }
}

pub type RoleReferenceInput = String;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionPolicyDocumentInput {
    #[serde(rename = "version")]
    pub version: SessionPolicyDocumentInputVersion,

    #[serde(rename = "statements")]
    pub statements: Vec<SessionPolicyStatementInput>,
}
impl SessionPolicyDocumentInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        version: SessionPolicyDocumentInputVersion,
        statements: Vec<SessionPolicyStatementInput>,
    ) -> Self {
        Self {
            version,
            statements,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionPolicyDocumentInputVersion {
    Value20240101,
    Unknown(String),
}
impl SessionPolicyDocumentInputVersion {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value20240101 => "2024-01-01",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SessionPolicyDocumentInputVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionPolicyDocumentInputVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "2024-01-01" => Self::Value20240101,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionPolicyStatementInput {
    /// Statement identifier
    #[serde(rename = "sid", default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    #[serde(rename = "effect")]
    pub effect: SessionPolicyStatementInputEffect,
    /// Actions in service:action format
    #[serde(rename = "actions", default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// The statement covers every action except these
    #[serde(
        rename = "not_actions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_actions: Option<Vec<String>>,
    /// Resource identifiers or patterns
    #[serde(rename = "resources", default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// The statement covers every resource except these
    #[serde(
        rename = "not_resources",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_resources: Option<Vec<String>>,
}
impl SessionPolicyStatementInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(effect: SessionPolicyStatementInputEffect) -> Self {
        Self {
            sid: None,
            effect,
            actions: None,
            not_actions: None,
            resources: None,
            not_resources: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionPolicyStatementInputEffect {
    Allow,
    Deny,
    Unknown(String),
}
impl SessionPolicyStatementInputEffect {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SessionPolicyStatementInputEffect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionPolicyStatementInputEffect {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "allow" => Self::Allow,
            "deny" => Self::Deny,
            _ => Self::Unknown(value),
        })
    }
}

pub type AssumeRoleBody = AssumeRoleRequestInput;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AssumeRoleResponse2 {
    /// Bearer token for the Basaltic API. Present on every role session.
    #[serde(
        rename = "access_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    /// Always `Bearer` when `access_token` is present.
    #[serde(
        rename = "token_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub token_type: Option<String>,
    /// Seconds until `access_token` expires.
    #[serde(
        rename = "expires_in",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_in: Option<i64>,
    /// SigV4 access key id, for the S3 endpoint.
    #[serde(
        rename = "access_key_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_key_id: Option<String>,
    /// SigV4 secret, for the S3 endpoint.
    #[serde(
        rename = "secret_access_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_access_key: Option<String>,
    /// SigV4 session token, for the S3 endpoint. Send as `X-Amz-Security-Token` and include it in `SignedHeaders`.
    #[serde(
        rename = "session_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_token: Option<String>,
    /// When the session — and therefore both credential forms — expires.
    #[serde(
        rename = "expiration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration: Option<String>,
    /// Owning account UUID of the target role.
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,
    /// Owning account handle of the target role.
    #[serde(
        rename = "account_handle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_handle: Option<String>,
    /// Immutable UUID of the assumed role.
    #[serde(rename = "role_id", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}
impl std::fmt::Debug for AssumeRoleResponse2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AssumeRoleResponse2");
        d.field("access_token", &"[REDACTED]");
        d.field("token_type", &"[REDACTED]");
        d.field("expires_in", &self.expires_in);
        d.field("access_key_id", &self.access_key_id);
        d.field("secret_access_key", &"[REDACTED]");
        d.field("session_token", &"[REDACTED]");
        d.field("expiration", &self.expiration);
        d.field("account_id", &self.account_id);
        d.field("account_handle", &self.account_handle);
        d.field("role_id", &self.role_id);
        d.finish()
    }
}

pub type AssumeRoleResponse = AssumeRoleResponse2;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AssumeRoleWithWebIdentityRequestInput {
    /// The identity token to exchange, as a signed JWT. It is verified before any role is read: the signature must chain to a key the trusted provider publishes, the audience must be the one this platform was configured to accept, and `exp` must be in the future.
    #[serde(rename = "web_identity_token")]
    pub web_identity_token: String,

    #[serde(rename = "role")]
    pub role: RoleReferenceInput,

    #[serde(rename = "account")]
    pub account: AccountReferenceInput,
    /// A label recorded on the session and in the audit trail. Defaults to the token's `sub` claim, so an unnamed session still records which identity it came from.
    #[serde(
        rename = "session_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_name: Option<String>,
    /// Credential validity duration (15 min to 12 hours). A value above the role's own `max_session_duration` is rejected rather than clamped.
    #[serde(
        rename = "duration_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_seconds: Option<i64>,
}
impl std::fmt::Debug for AssumeRoleWithWebIdentityRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AssumeRoleWithWebIdentityRequestInput");
        d.field("web_identity_token", &"[REDACTED]");
        d.field("role", &self.role);
        d.field("account", &self.account);
        d.field("session_name", &self.session_name);
        d.field("duration_seconds", &self.duration_seconds);
        d.finish()
    }
}
impl AssumeRoleWithWebIdentityRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        web_identity_token: String,
        role: RoleReferenceInput,
        account: AccountReferenceInput,
    ) -> Self {
        Self {
            web_identity_token,
            role,
            account,
            session_name: None,
            duration_seconds: None,
        }
    }
}

pub type AccountReferenceInput = String;

pub type AssumeRoleWithWebIdentityBody = AssumeRoleWithWebIdentityRequestInput;

pub type AssumeRoleWithWebIdentityResponse = AssumeRoleResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RolePolicyAttachRequestInput {
    #[serde(rename = "policy")]
    pub policy: PolicyReferenceInput,
}
impl RolePolicyAttachRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(policy: PolicyReferenceInput) -> Self {
        Self { policy }
    }
}

pub type PolicyReferenceInput = String;

pub type AttachRolePolicyBody = RolePolicyAttachRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyAttachRequestInput {
    #[serde(rename = "policy")]
    pub policy: PolicyReferenceInput,
}
impl PolicyAttachRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(policy: PolicyReferenceInput) -> Self {
        Self { policy }
    }
}

pub type AttachServiceAccountPolicyBody = PolicyAttachRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OAuthAuthorizeRequestInput {
    /// The registered client being approved.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// For the CLI this must be `urn:ietf:wg:oauth:2.0:oob` — the out-of-band pseudo-redirect, meaning the code is DISPLAYED rather than delivered anywhere. Nothing else is accepted for that client. Out-of-band because a redirect assumes the browser and the client are on the same machine, which is false for anyone signing in on a server they reach over SSH. What makes redemption safe is PKCE, not the delivery address.
    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,
    /// Base64url SHA-256 of the client's PKCE verifier, without padding.
    #[serde(rename = "code_challenge")]
    pub code_challenge: String,
    /// S256 only. `plain` is refused rather than merely discouraged: whoever intercepts the code also saw the challenge, so a plain challenge protects nothing.
    #[serde(rename = "code_challenge_method")]
    pub code_challenge_method: OAuthAuthorizeRequestInputCodeChallengeMethod,
    /// Opaque value echoed back on the redirect, unchanged. The client generated it and compares it on return.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "organization")]
    pub organization: OrganizationReferenceInput,
}
impl OAuthAuthorizeRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        client_id: String,
        redirect_uri: String,
        code_challenge: String,
        code_challenge_method: OAuthAuthorizeRequestInputCodeChallengeMethod,
        organization: OrganizationReferenceInput,
    ) -> Self {
        Self {
            client_id,
            redirect_uri,
            code_challenge,
            code_challenge_method,
            state: None,
            organization,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OAuthAuthorizeRequestInputCodeChallengeMethod {
    S256,
    Unknown(String),
}
impl OAuthAuthorizeRequestInputCodeChallengeMethod {
    pub fn as_str(&self) -> &str {
        match self {
            Self::S256 => "S256",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OAuthAuthorizeRequestInputCodeChallengeMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OAuthAuthorizeRequestInputCodeChallengeMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "S256" => Self::S256,
            _ => Self::Unknown(value),
        })
    }
}

pub type OrganizationReferenceInput = String;

pub type AuthorizeOAuthClientBody = OAuthAuthorizeRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OAuthAuthorizeResponse {
    /// The authorization code, for an out-of-band client — one with nowhere to redirect to. Show it to the user so they can carry it to the program that asked. Treat it as a credential: single use, and not something to log.
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Send the browser here, for a client that registered a real redirect. The URL carries the authorization code and the state — treat it as a credential, and do not log it.
    #[serde(
        rename = "redirect_to",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redirect_to: Option<String>,
    /// How long the code stays redeemable, in seconds.
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
}

pub type AuthorizeOAuthClientResponse = OAuthAuthorizeResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SSHKeyCreateRequestInput {
    #[serde(rename = "name")]
    pub name: String,
    /// One OpenSSH public key. Ed25519, ECDSA, security-key variants, and RSA of at least 2048 bits are supported. Private keys, certificates, multiple keys and authorized_keys options are rejected.
    #[serde(rename = "public_key")]
    pub public_key: String,
    /// Optional expiry at least one minute in the future. Rotation requires a new credential and revocation of the old one.
    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,
}
impl SSHKeyCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, public_key: String) -> Self {
        Self {
            name,
            public_key,
            expires_at: None,
        }
    }
}

pub type CreatePersonalSSHKeyBody = SSHKeyCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreatePersonalSSHKeyResult {
    #[serde(rename = "ssh_key")]
    pub ssh_key: SSHKey,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SSHKey {
    #[serde(rename = "id")]
    pub id: String,
    /// Identity-owned SSH credential CRN.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "name")]
    pub name: String,
    /// Canonical OpenSSH public key without a comment or authorized_keys options.
    #[serde(rename = "public_key")]
    pub public_key: String,
    /// SHA-256 fingerprint in OpenSSH format.
    #[serde(rename = "fingerprint")]
    pub fingerprint: String,

    #[serde(rename = "algorithm")]
    pub algorithm: String,

    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Optional expiry. Expired keys remain listed until revoked.
    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,
}

pub type CreatePersonalSSHKeyResponse = CreatePersonalSSHKeyResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyCreateRequestInput {
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
    pub tags: Option<TagsInput>,

    #[serde(rename = "document")]
    pub document: PolicyDocumentInput,
}
impl PolicyCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, document: PolicyDocumentInput) -> Self {
        Self {
            name,
            description: None,
            tags: None,
            document,
        }
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyDocumentInput {
    #[serde(rename = "version")]
    pub version: PolicyDocumentInputVersion,

    #[serde(rename = "statements")]
    pub statements: Vec<PolicyStatementInput>,
}
impl PolicyDocumentInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(version: PolicyDocumentInputVersion, statements: Vec<PolicyStatementInput>) -> Self {
        Self {
            version,
            statements,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyDocumentInputVersion {
    Value20240101,
    Unknown(String),
}
impl PolicyDocumentInputVersion {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value20240101 => "2024-01-01",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyDocumentInputVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyDocumentInputVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "2024-01-01" => Self::Value20240101,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyStatementInput {
    /// Statement identifier
    #[serde(rename = "sid", default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    #[serde(rename = "effect")]
    pub effect: PolicyStatementInputEffect,
    /// Actions in service:action format
    #[serde(rename = "actions", default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// The statement covers every action *except* these. Pairs naturally with `effect: deny` to carve a hole out of a broad allow; with `effect: allow` it grants everything the listed patterns don't name, including actions added by future services.
    #[serde(
        rename = "not_actions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_actions: Option<Vec<String>>,
    /// Resource identifiers or patterns
    #[serde(rename = "resources", default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// The statement covers every resource *except* these. Same trade-off as `not_actions`: with `effect: allow` it reaches resources that do not exist yet.
    #[serde(
        rename = "not_resources",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_resources: Option<Vec<String>>,
    /// Optional conditions for the statement
    #[serde(
        rename = "conditions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions: Option<Vec<PolicyConditionInput>>,
}
impl PolicyStatementInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(effect: PolicyStatementInputEffect) -> Self {
        Self {
            sid: None,
            effect,
            actions: None,
            not_actions: None,
            resources: None,
            not_resources: None,
            conditions: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyStatementInputEffect {
    Allow,
    Deny,
    Unknown(String),
}
impl PolicyStatementInputEffect {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyStatementInputEffect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyStatementInputEffect {
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
    /// The comparison operator
    #[serde(rename = "operator")]
    pub operator: PolicyConditionInputOperator,
    /// The condition key to evaluate
    #[serde(rename = "key")]
    pub key: String,
    /// Values to compare against
    #[serde(rename = "values")]
    pub values: Vec<String>,
    /// Evaluates `operator` against a multi-valued context key (a set, such as `basalt:TagKeys` — the tag keys a request carries) rather than a single value. Omit for an ordinary single-valued condition. - `for_all_values` — holds when every member of the request set satisfies `operator`. An absent or empty set holds vacuously, so a request carrying no tags is not fenced by a tag-key restriction. - `for_any_value` — holds when at least one member does. An absent or empty set does not hold.
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
    pub fn new(operator: PolicyConditionInputOperator, key: String, values: Vec<String>) -> Self {
        Self {
            operator,
            key,
            values,
            set_operator: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyConditionInputOperator {
    Equals,
    NotEquals,
    StartsWith,
    EndsWith,
    Contains,
    In,
    NotIn,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    Exists,
    NotExists,
    IpAddress,
    NotIpAddress,
    Unknown(String),
}
impl PolicyConditionInputOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Equals => "equals",
            Self::NotEquals => "not_equals",
            Self::StartsWith => "starts_with",
            Self::EndsWith => "ends_with",
            Self::Contains => "contains",
            Self::In => "in",
            Self::NotIn => "not_in",
            Self::GreaterThan => "greater_than",
            Self::LessThan => "less_than",
            Self::GreaterThanOrEquals => "greater_than_or_equals",
            Self::LessThanOrEquals => "less_than_or_equals",
            Self::Exists => "exists",
            Self::NotExists => "not_exists",
            Self::IpAddress => "ip_address",
            Self::NotIpAddress => "not_ip_address",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyConditionInputOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyConditionInputOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "equals" => Self::Equals,
            "not_equals" => Self::NotEquals,
            "starts_with" => Self::StartsWith,
            "ends_with" => Self::EndsWith,
            "contains" => Self::Contains,
            "in" => Self::In,
            "not_in" => Self::NotIn,
            "greater_than" => Self::GreaterThan,
            "less_than" => Self::LessThan,
            "greater_than_or_equals" => Self::GreaterThanOrEquals,
            "less_than_or_equals" => Self::LessThanOrEquals,
            "exists" => Self::Exists,
            "not_exists" => Self::NotExists,
            "ip_address" => Self::IpAddress,
            "not_ip_address" => Self::NotIpAddress,
            _ => Self::Unknown(value),
        })
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

pub type CreatePolicyBody = PolicyCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CreatePolicyResult {
    #[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Policy {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Managed policy CRN; absent on inline policy projections in effective-policy lists.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    /// Whether this is a system-managed policy (cannot be modified or deleted)
    #[serde(rename = "is_system", default, skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,

    #[serde(rename = "document", default, skip_serializing_if = "Option::is_none")]
    pub document: Option<PolicyDocument>,
    /// Creation timestamp (not present for system policies)
    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,
    /// Last update timestamp (not present for system policies)
    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
    /// Owning account UUID; absent for shared system policies.
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,
    /// Immutable handle of the owning account.
    #[serde(
        rename = "account_handle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_handle: Option<String>,
}

pub type Tags = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyDocument {
    #[serde(rename = "version")]
    pub version: PolicyDocumentVersion,

    #[serde(rename = "statements")]
    pub statements: Vec<PolicyStatement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyDocumentVersion {
    Value20240101,
    Unknown(String),
}
impl PolicyDocumentVersion {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value20240101 => "2024-01-01",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyDocumentVersion {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyDocumentVersion {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "2024-01-01" => Self::Value20240101,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PolicyStatement {
    /// Statement identifier
    #[serde(rename = "sid", default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,

    #[serde(rename = "effect")]
    pub effect: PolicyStatementEffect,
    /// Actions in service:action format
    #[serde(rename = "actions", default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    /// The statement covers every action *except* these. Pairs naturally with `effect: deny` to carve a hole out of a broad allow; with `effect: allow` it grants everything the listed patterns don't name, including actions added by future services.
    #[serde(
        rename = "not_actions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_actions: Option<Vec<String>>,
    /// Resource identifiers or patterns
    #[serde(rename = "resources", default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// The statement covers every resource *except* these. Same trade-off as `not_actions`: with `effect: allow` it reaches resources that do not exist yet.
    #[serde(
        rename = "not_resources",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub not_resources: Option<Vec<String>>,
    /// Optional conditions for the statement
    #[serde(
        rename = "conditions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions: Option<Vec<PolicyCondition>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyStatementEffect {
    Allow,
    Deny,
    Unknown(String),
}
impl PolicyStatementEffect {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Allow => "allow",
            Self::Deny => "deny",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyStatementEffect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyStatementEffect {
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
    /// The comparison operator
    #[serde(rename = "operator")]
    pub operator: PolicyConditionOperator,
    /// The condition key to evaluate
    #[serde(rename = "key")]
    pub key: String,
    /// Values to compare against
    #[serde(rename = "values")]
    pub values: Vec<String>,
    /// Evaluates `operator` against a multi-valued context key (a set, such as `basalt:TagKeys` — the tag keys a request carries) rather than a single value. Omit for an ordinary single-valued condition. - `for_all_values` — holds when every member of the request set satisfies `operator`. An absent or empty set holds vacuously, so a request carrying no tags is not fenced by a tag-key restriction. - `for_any_value` — holds when at least one member does. An absent or empty set does not hold.
    #[serde(
        rename = "set_operator",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub set_operator: Option<PolicyConditionSetOperator>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PolicyConditionOperator {
    Equals,
    NotEquals,
    StartsWith,
    EndsWith,
    Contains,
    In,
    NotIn,
    GreaterThan,
    LessThan,
    GreaterThanOrEquals,
    LessThanOrEquals,
    Exists,
    NotExists,
    IpAddress,
    NotIpAddress,
    Unknown(String),
}
impl PolicyConditionOperator {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Equals => "equals",
            Self::NotEquals => "not_equals",
            Self::StartsWith => "starts_with",
            Self::EndsWith => "ends_with",
            Self::Contains => "contains",
            Self::In => "in",
            Self::NotIn => "not_in",
            Self::GreaterThan => "greater_than",
            Self::LessThan => "less_than",
            Self::GreaterThanOrEquals => "greater_than_or_equals",
            Self::LessThanOrEquals => "less_than_or_equals",
            Self::Exists => "exists",
            Self::NotExists => "not_exists",
            Self::IpAddress => "ip_address",
            Self::NotIpAddress => "not_ip_address",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PolicyConditionOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyConditionOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "equals" => Self::Equals,
            "not_equals" => Self::NotEquals,
            "starts_with" => Self::StartsWith,
            "ends_with" => Self::EndsWith,
            "contains" => Self::Contains,
            "in" => Self::In,
            "not_in" => Self::NotIn,
            "greater_than" => Self::GreaterThan,
            "less_than" => Self::LessThan,
            "greater_than_or_equals" => Self::GreaterThanOrEquals,
            "less_than_or_equals" => Self::LessThanOrEquals,
            "exists" => Self::Exists,
            "not_exists" => Self::NotExists,
            "ip_address" => Self::IpAddress,
            "not_ip_address" => Self::NotIpAddress,
            _ => Self::Unknown(value),
        })
    }
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

pub type CreatePolicyResponse = CreatePolicyResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RoleCreateRequestInput {
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
    pub tags: Option<TagsInput>,

    #[serde(
        rename = "trust_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trust_policy: Option<TrustPolicyInput>,
}
impl RoleCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            tags: None,
            trust_policy: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TrustPolicyInput {
    /// Qualified CRN patterns identifying who may assume this account role. Same-organization membership alone does not establish trust. The caller also needs iam:AssumeRole permission for the target role. Account roles and service accounts use crn:iam::&lt;account-handle&gt;:role/&lt;name&gt; and crn:iam::&lt;account-handle&gt;:service-account/&lt;name&gt;. Human users use crn:workspace:::user/&lt;username&gt; or crn:workspace:::user/* in the selected organization. An instance presents its compute CRN through IMDS. A federated caller names the trusted provider as crn:iam:::oidc-provider/&lt;provider&gt; and uses conditions to restrict token claims.
    #[serde(
        rename = "principals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principals: Option<Vec<String>>,
    /// Optional conditions for role assumption
    #[serde(
        rename = "conditions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions: Option<Vec<PolicyConditionInput>>,
}

pub type CreateRoleBody = RoleCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateRoleResult {
    #[serde(rename = "role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Role {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(
        rename = "trust_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trust_policy: Option<TrustPolicy>,

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
    /// Owning account UUID.
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,
    /// Immutable handle of the owning account.
    #[serde(
        rename = "account_handle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_handle: Option<String>,
    /// System roles cannot be changed or deleted.
    #[serde(rename = "is_system", default, skip_serializing_if = "Option::is_none")]
    pub is_system: Option<bool>,
    /// Present on the account's built-in Administrator and ReadOnly roles. These assignable roles are created with the account, do not consume custom-role quota, and do not block account deletion.
    #[serde(
        rename = "builtin_kind",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub builtin_kind: Option<RoleBuiltinKind>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TrustPolicy {
    /// Qualified CRN patterns identifying who may assume this account role. Same-organization membership alone does not establish trust. The caller also needs iam:AssumeRole permission for the target role. Account roles and service accounts use crn:iam::&lt;account-handle&gt;:role/&lt;name&gt; and crn:iam::&lt;account-handle&gt;:service-account/&lt;name&gt;. Human users use crn:workspace:::user/&lt;username&gt; or crn:workspace:::user/* in the selected organization. An instance presents its compute CRN through IMDS. A federated caller names the trusted provider as crn:iam:::oidc-provider/&lt;provider&gt; and uses conditions to restrict token claims.
    #[serde(
        rename = "principals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principals: Option<Vec<String>>,
    /// Optional conditions for role assumption
    #[serde(
        rename = "conditions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub conditions: Option<Vec<PolicyCondition>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoleBuiltinKind {
    Administrator,
    Readonly,
    Unknown(String),
}
impl RoleBuiltinKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Administrator => "administrator",
            Self::Readonly => "readonly",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RoleBuiltinKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RoleBuiltinKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "administrator" => Self::Administrator,
            "readonly" => Self::Readonly,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateRoleResponse = CreateRoleResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ServiceAccountCreateRequestInput {
    /// Immutable account-scoped name. The Linux login is sa_&lt;name&gt;. Resource names must not start with the literal crn: prefix or be UUIDs.
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
}
impl ServiceAccountCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            tags: None,
        }
    }
}

pub type CreateServiceAccountBody = ServiceAccountCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateServiceAccountResult {
    #[serde(
        rename = "service_account",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account: Option<ServiceAccount>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ServiceAccount {
    #[serde(
        rename = "linux_identity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_identity: Option<LinuxIdentity>,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Owning account UUID.
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

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
    /// Immutable handle of the owning account.
    #[serde(
        rename = "account_handle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_handle: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LinuxIdentity {
    /// Home allocated to this identity. New human identities use /home/bsu_&lt;uid&gt; and service accounts use /home/bsa_&lt;uid&gt;. Existing identities retain their home. Read this value instead of deriving it from the username.
    #[serde(rename = "home_directory")]
    pub home_directory: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "uid")]
    pub uid: i64,

    #[serde(rename = "gid")]
    pub gid: i64,
}

pub type CreateServiceAccountResponse = CreateServiceAccountResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CredentialCreateRequestInput {
    #[serde(rename = "name")]
    pub name: String,
    /// Optional expiration date
    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,
}
impl CredentialCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            expires_at: None,
        }
    }
}

pub type CreateServiceAccountCredentialBody = CredentialCreateRequestInput;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CredentialCreateResponse {
    #[serde(
        rename = "credential",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credential: Option<Credential>,
    /// Only returned once at creation time
    #[serde(
        rename = "secret_access_key",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_access_key: Option<String>,
}
impl std::fmt::Debug for CredentialCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CredentialCreateResponse");
        d.field("credential", &"[REDACTED]");
        d.field("secret_access_key", &"[REDACTED]");
        d.finish()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Credential {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "access_key_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_key_id: Option<String>,

    #[serde(
        rename = "last_used_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_used_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub expires_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,
}

pub type CreateServiceAccountCredentialResponse = CredentialCreateResponse;

pub type CreateServiceAccountSSHKeyBody = SSHKeyCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateServiceAccountSSHKeyResult {
    #[serde(rename = "ssh_key")]
    pub ssh_key: SSHKey,
}

pub type CreateServiceAccountSSHKeyResponse = CreateServiceAccountSSHKeyResult;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthTokenRequestInput {
    /// `client_credentials` is the one to use for a service account: it exchanges an access key pair for a token, and needs nothing else. `authorization_code` and `refresh_token` belong to the interactive login a person runs (`basaltic login`), where the token names a USER rather than a service account. They are driven by the CLI, not written by hand. Check the authorization-server metadata document before branching on them — they are advertised only where an authorization endpoint is configured.
    #[serde(rename = "grant_type")]
    pub grant_type: OAuthTokenRequestInputGrantType,
    /// The access key id. Omit when using HTTP Basic.
    #[serde(rename = "client_id", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The secret access key. Omit when using HTTP Basic.
    #[serde(
        rename = "client_secret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub client_secret: Option<String>,
    /// Requested token lifetime. A Basaltic extension, not an OAuth parameter — omit it and you get the default. Values outside the range are clamped into it rather than refused, so asking for a day yields the longest token allowed.
    #[serde(
        rename = "duration_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_seconds: Option<i64>,
    /// The authorization code from the consent redirect. Single use, and valid for five minutes. `authorization_code` grant only.
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The PKCE verifier whose SHA-256 was sent as `code_challenge` when the flow started (RFC 7636). Required with `authorization_code`: it is what proves this is the client that began the flow, since a CLI holds no client secret.
    #[serde(
        rename = "code_verifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code_verifier: Option<String>,
    /// The same `redirect_uri` the code was issued for — for the CLI, `urn:ietf:wg:oauth:2.0:oob`. Re-checked here, so a code cannot be redeemed under a different one (RFC 6749 4.1.3).
    #[serde(
        rename = "redirect_uri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redirect_uri: Option<String>,
    /// `refresh_token` grant only. Renews a user session without another trip through the browser. Rotated on every use — store the new one.
    #[serde(
        rename = "refresh_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<String>,
}
impl std::fmt::Debug for OAuthTokenRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("OAuthTokenRequestInput");
        d.field("grant_type", &self.grant_type);
        d.field("client_id", &self.client_id);
        d.field("client_secret", &"[REDACTED]");
        d.field("duration_seconds", &self.duration_seconds);
        d.field("code", &self.code);
        d.field("code_verifier", &"[REDACTED]");
        d.field("redirect_uri", &self.redirect_uri);
        d.field("refresh_token", &"[REDACTED]");
        d.finish()
    }
}
impl OAuthTokenRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(grant_type: OAuthTokenRequestInputGrantType) -> Self {
        Self {
            grant_type,
            client_id: None,
            client_secret: None,
            duration_seconds: None,
            code: None,
            code_verifier: None,
            redirect_uri: None,
            refresh_token: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OAuthTokenRequestInputGrantType {
    ClientCredentials,
    AuthorizationCode,
    RefreshToken,
    Unknown(String),
}
impl OAuthTokenRequestInputGrantType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientCredentials => "client_credentials",
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OAuthTokenRequestInputGrantType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OAuthTokenRequestInputGrantType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "client_credentials" => Self::ClientCredentials,
            "authorization_code" => Self::AuthorizationCode,
            "refresh_token" => Self::RefreshToken,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetOAuthTokenBody = OAuthTokenRequestInput;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthTokenResponse {
    /// Send as `Authorization: Bearer <token>`. Opaque to clients: do not parse it, and do not key anything on the token string.
    #[serde(rename = "access_token")]
    pub access_token: String,

    #[serde(rename = "token_type")]
    pub token_type: OAuthTokenResponseTokenType,
    /// Seconds until the token expires.
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    /// Returned only by the user grants (`authorization_code` and `refresh_token`). Present it to the `refresh_token` grant to renew without another browser round trip; it is ROTATED on each use, so replace the stored copy every time. A service account gets none. It already holds a long-lived access key and can simply run `client_credentials` again, so a refresh token would be a second credential to store for no gain.
    #[serde(
        rename = "refresh_token",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<String>,
}
impl std::fmt::Debug for OAuthTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("OAuthTokenResponse");
        d.field("access_token", &"[REDACTED]");
        d.field("token_type", &"[REDACTED]");
        d.field("expires_in", &self.expires_in);
        d.field("refresh_token", &"[REDACTED]");
        d.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OAuthTokenResponseTokenType {
    Bearer,
    Unknown(String),
}
impl OAuthTokenResponseTokenType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bearer => "Bearer",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OAuthTokenResponseTokenType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OAuthTokenResponseTokenType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Bearer" => Self::Bearer,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetOAuthTokenResponse = OAuthTokenResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetPersonalLinuxIdentityResult {
    #[serde(rename = "linux_identity")]
    pub linux_identity: LinuxIdentity,
}

pub type GetPersonalLinuxIdentityResponse = GetPersonalLinuxIdentityResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetPolicyResult {
    #[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

pub type GetPolicyResponse = GetPolicyResult;

pub type GetPolicyResource = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetPolicyScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRoleResult {
    #[serde(rename = "role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

pub type GetRoleResponse = GetRoleResult;

pub type GetRoleResource = Role;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRoleScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InlinePolicyResponse {
    #[serde(
        rename = "inline_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_policy: Option<InlinePolicy>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InlinePolicy {
    /// Canonical inline policy identity under the owning account principal.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename = "principal_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_id: Option<String>,

    #[serde(
        rename = "principal_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_type: Option<InlinePolicyPrincipalType>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "document", default, skip_serializing_if = "Option::is_none")]
    pub document: Option<PolicyDocument>,

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InlinePolicyPrincipalType {
    ServiceAccount,
    Role,
    Unknown(String),
}
impl InlinePolicyPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ServiceAccount => "service_account",
            Self::Role => "role",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InlinePolicyPrincipalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InlinePolicyPrincipalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "service_account" => Self::ServiceAccount,
            "role" => Self::Role,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetRoleInlinePolicyResponse = InlinePolicyResponse;

pub type GetRoleInlinePolicyResource = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRoleInlinePolicyScope {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PermissionBoundaryResponse {
    #[serde(
        rename = "permission_boundary",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_boundary: Option<PermissionBoundary>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PermissionBoundary {
    #[serde(
        rename = "principal_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_id: Option<String>,

    #[serde(
        rename = "principal_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_type: Option<PermissionBoundaryPrincipalType>,

    #[serde(rename = "policy_id", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,

    #[serde(
        rename = "policy_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_name: Option<String>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PermissionBoundaryPrincipalType {
    ServiceAccount,
    Role,
    Unknown(String),
}
impl PermissionBoundaryPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ServiceAccount => "service_account",
            Self::Role => "role",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PermissionBoundaryPrincipalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PermissionBoundaryPrincipalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "service_account" => Self::ServiceAccount,
            "role" => Self::Role,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetRolePermissionBoundaryResponse = PermissionBoundaryResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct STSSessionResponse {
    #[serde(
        rename = "sts_session",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sts_session: Option<STSSession>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct STSSession {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The assumed role UUID; absent on service-account OAuth sessions that do not assume a role.
    #[serde(rename = "role_id", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// ID of the principal assuming the role (the source identity)
    #[serde(
        rename = "principal_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_id: Option<String>,

    #[serde(
        rename = "principal_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_type: Option<STSSessionPrincipalType>,
    /// Optional session identifier
    #[serde(
        rename = "session_name",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub session_name: Option<crate::Nullable<String>>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,

    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,

    #[serde(
        rename = "last_used_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_used_at: Option<crate::Nullable<String>>,
    /// Whether the session has been revoked
    #[serde(rename = "revoked", default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,

    #[serde(
        rename = "revoked_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub revoked_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "revoked_reason",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub revoked_reason: Option<crate::Nullable<String>>,
    /// IP address where the session was created
    #[serde(
        rename = "source_ip",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub source_ip: Option<crate::Nullable<String>>,
    /// User-Agent of the caller that created the session
    #[serde(
        rename = "user_agent",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub user_agent: Option<crate::Nullable<String>>,

    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,

    #[serde(
        rename = "account_handle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_handle: Option<String>,

    #[serde(
        rename = "source_account_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub source_account_id: Option<crate::Nullable<String>>,

    #[serde(
        rename = "source_principal_type",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub source_principal_type: Option<crate::Nullable<String>>,

    #[serde(
        rename = "source_principal_crn",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub source_principal_crn: Option<crate::Nullable<String>>,

    #[serde(
        rename = "parent_session_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub parent_session_id: Option<crate::Nullable<String>>,

    #[serde(
        rename = "grant_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_type: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum STSSessionPrincipalType {
    User,
    ServiceAccount,
    AssumedRole,
    Unknown(String),
}
impl STSSessionPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::ServiceAccount => "service_account",
            Self::AssumedRole => "assumed_role",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for STSSessionPrincipalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for STSSessionPrincipalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "user" => Self::User,
            "service_account" => Self::ServiceAccount,
            "assumed_role" => Self::AssumedRole,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetSTSSessionResponse = STSSessionResponse;

pub type GetSTSSessionResource = STSSession;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetSTSSessionScope {
    #[serde(rename = "role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(rename = "principal", default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,

    #[serde(
        rename = "principal_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_type: Option<GetSTSSessionScopePrincipalType>,

    #[serde(
        rename = "active_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_only: Option<bool>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetSTSSessionScopePrincipalType {
    User,
    ServiceAccount,
    Role,
    AssumedRole,
    Unknown(String),
}
impl GetSTSSessionScopePrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::ServiceAccount => "service_account",
            Self::Role => "role",
            Self::AssumedRole => "assumed_role",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetSTSSessionScopePrincipalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetSTSSessionScopePrincipalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "user" => Self::User,
            "service_account" => Self::ServiceAccount,
            "role" => Self::Role,
            "assumed_role" => Self::AssumedRole,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetServiceAccountResult {
    #[serde(
        rename = "service_account",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account: Option<ServiceAccount>,
}

pub type GetServiceAccountResponse = GetServiceAccountResult;

pub type GetServiceAccountResource = ServiceAccount;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetServiceAccountScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetServiceAccountInlinePolicyResponse = InlinePolicyResponse;

pub type GetServiceAccountInlinePolicyResource = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetServiceAccountInlinePolicyScope {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetServiceAccountLinuxIdentityResult {
    #[serde(rename = "linux_identity")]
    pub linux_identity: LinuxIdentity,
}

pub type GetServiceAccountLinuxIdentityResponse = GetServiceAccountLinuxIdentityResult;

pub type GetServiceAccountPermissionBoundaryResponse = PermissionBoundaryResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListPersonalSSHKeysResult {
    #[serde(rename = "ssh_keys")]
    pub ssh_keys: Vec<SSHKey>,
}

pub type ListPersonalSSHKeysResponse = ListPersonalSSHKeysResult;

pub type ListPersonalSSHKeysItem = SSHKey;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListPoliciesQuery = ListPoliciesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PolicyListResponse {
    #[serde(rename = "policies", default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,

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

pub type ListPoliciesResponse = PolicyListResponse;

pub type ListPoliciesItem = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPolicyRolesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListPolicyRolesQuery = ListPolicyRolesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PolicyRolesListResponse {
    #[serde(rename = "roles", default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListPolicyRolesResponse = PolicyRolesListResponse;

pub type ListPolicyRolesItem = Role;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPolicyServiceAccountsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListPolicyServiceAccountsQuery = ListPolicyServiceAccountsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PolicyServiceAccountsListResponse {
    #[serde(
        rename = "service_accounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_accounts: Option<Vec<ServiceAccount>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListPolicyServiceAccountsResponse = PolicyServiceAccountsListResponse;

pub type ListPolicyServiceAccountsItem = ServiceAccount;

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
    /// List of available regions
    #[serde(rename = "regions")]
    pub regions: Vec<Region>,
    /// The default region code
    #[serde(rename = "default")]
    pub default: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Region {
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
    /// Whether the region is currently available for use
    #[serde(rename = "available")]
    pub available: bool,
    /// Whether the region is announced but not yet available
    #[serde(rename = "coming_soon")]
    pub coming_soon: bool,
}

pub type ListRegionsResponse = ListRegionsResult;

pub type ListRegionsItem = Region;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRoleInlinePoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListRoleInlinePoliciesQuery = ListRoleInlinePoliciesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InlinePolicyListResponse {
    #[serde(
        rename = "inline_policies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_policies: Option<Vec<InlinePolicy>>,
}

pub type ListRoleInlinePoliciesResponse = InlinePolicyListResponse;

pub type ListRoleInlinePoliciesItem = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRolePoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListRolePoliciesQuery = ListRolePoliciesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RolePoliciesListResponse {
    #[serde(rename = "policies", default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

pub type ListRolePoliciesResponse = RolePoliciesListResponse;

pub type ListRolePoliciesItem = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRolesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListRolesQuery = ListRolesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RoleListResponse {
    #[serde(rename = "roles", default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListRolesResponse = RoleListResponse;

pub type ListRolesItem = Role;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSTSSessionsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(rename = "principal", default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,

    #[serde(
        rename = "principal_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_type: Option<ListSTSSessionsParametersPrincipalType>,

    #[serde(
        rename = "active_only",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_only: Option<bool>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListSTSSessionsParametersPrincipalType {
    User,
    ServiceAccount,
    Role,
    AssumedRole,
    Unknown(String),
}
impl ListSTSSessionsParametersPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::ServiceAccount => "service_account",
            Self::Role => "role",
            Self::AssumedRole => "assumed_role",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListSTSSessionsParametersPrincipalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListSTSSessionsParametersPrincipalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "user" => Self::User,
            "service_account" => Self::ServiceAccount,
            "role" => Self::Role,
            "assumed_role" => Self::AssumedRole,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListSTSSessionsQuery = ListSTSSessionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct STSSessionListResponse {
    #[serde(
        rename = "sts_sessions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sts_sessions: Option<Vec<STSSession>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListSTSSessionsResponse = STSSessionListResponse;

pub type ListSTSSessionsItem = STSSession;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListServiceAccountCredentialsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListServiceAccountCredentialsQuery = ListServiceAccountCredentialsParameters;

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CredentialListResponse {
    #[serde(
        rename = "credentials",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub credentials: Option<Vec<Credential>>,
}
impl std::fmt::Debug for CredentialListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CredentialListResponse");
        d.field("credentials", &"[REDACTED]");
        d.finish()
    }
}

pub type ListServiceAccountCredentialsResponse = CredentialListResponse;

pub type ListServiceAccountCredentialsItem = Credential;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListServiceAccountInlinePoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListServiceAccountInlinePoliciesQuery = ListServiceAccountInlinePoliciesParameters;

pub type ListServiceAccountInlinePoliciesResponse = InlinePolicyListResponse;

pub type ListServiceAccountInlinePoliciesItem = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListServiceAccountPoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListServiceAccountPoliciesQuery = ListServiceAccountPoliciesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PrincipalPoliciesListResponse {
    #[serde(rename = "policies", default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

pub type ListServiceAccountPoliciesResponse = PrincipalPoliciesListResponse;

pub type ListServiceAccountPoliciesItem = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListServiceAccountSSHKeysResult {
    #[serde(rename = "ssh_keys")]
    pub ssh_keys: Vec<SSHKey>,
}

pub type ListServiceAccountSSHKeysResponse = ListServiceAccountSSHKeysResult;

pub type ListServiceAccountSSHKeysItem = SSHKey;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListServiceAccountsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListServiceAccountsQuery = ListServiceAccountsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ServiceAccountListResponse {
    #[serde(
        rename = "service_accounts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_accounts: Option<Vec<ServiceAccount>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListServiceAccountsResponse = ServiceAccountListResponse;

pub type ListServiceAccountsItem = ServiceAccount;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutInlinePolicyRequestInput {
    #[serde(rename = "document")]
    pub document: PolicyDocumentInput,
}
impl PutInlinePolicyRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(document: PolicyDocumentInput) -> Self {
        Self { document }
    }
}

pub type PutRoleInlinePolicyBody = PutInlinePolicyRequestInput;

pub type PutRoleInlinePolicyResponse = InlinePolicyResponse;

pub type PutServiceAccountInlinePolicyBody = PutInlinePolicyRequestInput;

pub type PutServiceAccountInlinePolicyResponse = InlinePolicyResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OAuthRevokeRequestInput {
    /// The access token to revoke.
    #[serde(rename = "token")]
    pub token: String,
    /// Accepted and ignored — the token identifies itself. Present because RFC 7009 clients send it.
    #[serde(
        rename = "token_type_hint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub token_type_hint: Option<String>,
}
impl std::fmt::Debug for OAuthRevokeRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("OAuthRevokeRequestInput");
        d.field("token", &"[REDACTED]");
        d.field("token_type_hint", &"[REDACTED]");
        d.finish()
    }
}
impl OAuthRevokeRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type_hint: None,
        }
    }
}

pub type RevokeOAuthTokenBody = OAuthRevokeRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RevokeSTSSessionRequest {
    /// Reason for revoking the session
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

pub type RevokeSTSSessionBody = RevokeSTSSessionRequest;

pub type RevokeSTSSessionResponse = STSSessionResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetBoundaryRequestInput {
    #[serde(rename = "policy")]
    pub policy: PolicyReferenceInput,
}
impl SetBoundaryRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(policy: PolicyReferenceInput) -> Self {
        Self { policy }
    }
}

pub type SetRolePermissionBoundaryBody = SetBoundaryRequestInput;

pub type SetServiceAccountPermissionBoundaryBody = SetBoundaryRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PolicyUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,

    #[serde(rename = "document", default, skip_serializing_if = "Option::is_none")]
    pub document: Option<PolicyDocumentInput>,
}

pub type UpdatePolicyBody = PolicyUpdateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdatePolicyResult {
    #[serde(rename = "policy", default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<Policy>,
}

pub type UpdatePolicyResponse = UpdatePolicyResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RoleUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,

    #[serde(
        rename = "trust_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trust_policy: Option<TrustPolicyInput>,
}

pub type UpdateRoleBody = RoleUpdateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateRoleResult {
    #[serde(rename = "role", default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

pub type UpdateRoleResponse = UpdateRoleResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ServiceAccountUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

pub type UpdateServiceAccountBody = ServiceAccountUpdateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateServiceAccountResult {
    #[serde(
        rename = "service_account",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account: Option<ServiceAccount>,
}

pub type UpdateServiceAccountResponse = UpdateServiceAccountResult;
