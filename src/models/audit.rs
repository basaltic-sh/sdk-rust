//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AuditLogResponse {
    #[serde(rename = "audit_log", default, skip_serializing_if = "Option::is_none")]
    pub audit_log: Option<AuditLog>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AuditLog {
    /// Canonical event identity, scoped to the authenticated organization.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// When the event occurred
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Immutable event-time actor identity. Null for historical entries without a snapshot. Authenticated user events use crn:workspace:::user/&lt;username&gt;, with the user UUID as a fallback when a username is unavailable; service accounts use crn:iam::&lt;account-handle&gt;:service-account/&lt;name&gt;; assumed roles use crn:iam::&lt;account-handle&gt;:role/&lt;name&gt;, with the session identity in details.actor_session_crn and verified ancestry in details.auth_chain. Federated assumptions identify the verified provider in actor_crn and the verified subject in actor_name and details.auth_chain. Historical entries retain their original CRNs, including legacy IAM organization identities. System actors use crn:iam::platform:system/&lt;service&gt;, with service names certificate, registry, secrets, queue, notifications and email.
    #[serde(
        rename = "actor_crn",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub actor_crn: Option<crate::Nullable<String>>,
    /// Immutable event-time resource identity. Null for historical entries without a snapshot and enumerated events whose target cannot be identified from event-time data, such as an unknown-email password reset or a lookup that never resolved a row. A known target UUID is retained in details.resource_id; it is never substituted for the immutable name in a CRN. AssumeRole and AssumeRoleWithWebIdentity target the role being assumed. GetToken targets the authenticated service account. These authentication events record the resulting session in details.session_crn.
    #[serde(
        rename = "resource_crn",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub resource_crn: Option<crate::Nullable<String>>,
    /// Name of the actor at the time of the event
    #[serde(
        rename = "actor_name",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub actor_name: Option<crate::Nullable<String>>,
    /// Email of the actor (for users only)
    #[serde(
        rename = "actor_email",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub actor_email: Option<crate::Nullable<String>>,
    /// The action performed (e.g., "iam.policy.create", "instance.start")
    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Outcome of the action
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AuditLogStatus>,
    /// Name of the resource at the time of the event
    #[serde(
        rename = "resource_name",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub resource_name: Option<crate::Nullable<String>>,
    /// IP address of the request origin
    #[serde(
        rename = "ip_address",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub ip_address: Option<crate::Nullable<String>>,
    /// User agent string from the request
    #[serde(
        rename = "user_agent",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub user_agent: Option<crate::Nullable<String>>,
    /// Request ID for correlation
    #[serde(
        rename = "request_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub request_id: Option<crate::Nullable<String>>,
    /// Additional action-specific details. For assumed-role actors, actor_session_crn records crn:iam::&lt;account-handle&gt;:sts-session/&lt;id&gt; to correlate the event with its AssumeRole call. auth_chain is an immutable snapshot of verified identities, ordered from the original user, service account or workload through every assumed role to the effective actor. It contains identifiers and session metadata, never credentials. Historical events without recorded ancestry omit auth_chain; reads do not reconstruct it from current sessions or identities. Authentication grants include session_id, session_crn, grant_type and expires_at. A federated origin includes its verified issuer and subject independently of the caller-supplied session name. Initiated background work retains the original audit attribution and request correlation.
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<AuditLogDetails>,
    /// Error code for failed actions
    #[serde(
        rename = "error_code",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub error_code: Option<crate::Nullable<String>>,
    /// Error message for failed actions
    #[serde(
        rename = "error_message",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub error_message: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditLogStatus {
    Success,
    Failure,
    Denied,
    Unknown(String),
}
impl AuditLogStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::Denied => "denied",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AuditLogStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AuditLogStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "success" => Self::Success,
            "failure" => Self::Failure,
            "denied" => Self::Denied,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AuditLogDetails {
    #[serde(
        rename = "auth_chain",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_chain: Option<Vec<AuditAuthenticationStep>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AuditAuthenticationStep {
    /// Stable principal UUID. For a federated workload this is derived from its verified issuer and subject.
    #[serde(rename = "principal_id")]
    pub principal_id: String,
    /// Identity kind, such as user, service_account, assumed_role, instance or web_identity.
    #[serde(rename = "principal_type")]
    pub principal_type: String,
    /// Recorded principal identity; for web identity this identifies the verified provider.
    #[serde(
        rename = "principal_crn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_crn: Option<String>,
    /// Display name captured when the request was authenticated, when available.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Verified issuer of the originating web identity token, when applicable.
    #[serde(rename = "issuer", default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// Verified subject of the originating web identity token, independent of the session label.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// User email captured when the request was authenticated, when available.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(
        rename = "session_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_id: Option<String>,
    /// Account-qualified STS session identity, when applicable.
    #[serde(
        rename = "session_crn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_crn: Option<String>,
    /// Caller-supplied session label, not proof of identity.
    #[serde(
        rename = "session_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_name: Option<String>,
    /// How this session was issued, such as assume_role, client_credentials, imds or web_identity.
    #[serde(
        rename = "grant_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grant_type: Option<String>,
    /// When this session was created.
    #[serde(rename = "issued_at", default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
}

pub type GetAuditLogResponse = AuditLogResponse;

pub type GetAuditLogResource = AuditLog;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetAuditLogScope {
    #[serde(rename = "actor", default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,

    #[serde(
        rename = "actor_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_type: Option<GetAuditLogScopeActorType>,

    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(
        rename = "resource_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<String>,

    #[serde(rename = "resource", default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetAuditLogScopeStatus>,

    #[serde(
        rename = "ip_address",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_address: Option<String>,

    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetAuditLogScopeActorType {
    User,
    ServiceAccount,
    System,
    Unknown(String),
}
impl GetAuditLogScopeActorType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::ServiceAccount => "service_account",
            Self::System => "system",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetAuditLogScopeActorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetAuditLogScopeActorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "user" => Self::User,
            "service_account" => Self::ServiceAccount,
            "system" => Self::System,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetAuditLogScopeStatus {
    Success,
    Failure,
    Denied,
    Unknown(String),
}
impl GetAuditLogScopeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::Denied => "denied",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetAuditLogScopeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetAuditLogScopeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "success" => Self::Success,
            "failure" => Self::Failure,
            "denied" => Self::Denied,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListAuditLogsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "actor", default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,

    #[serde(
        rename = "actor_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actor_type: Option<ListAuditLogsParametersActorType>,

    #[serde(rename = "action", default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,

    #[serde(
        rename = "resource_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<String>,

    #[serde(rename = "resource", default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAuditLogsParametersStatus>,

    #[serde(
        rename = "ip_address",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_address: Option<String>,

    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListAuditLogsParametersActorType {
    User,
    ServiceAccount,
    System,
    Unknown(String),
}
impl ListAuditLogsParametersActorType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::ServiceAccount => "service_account",
            Self::System => "system",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListAuditLogsParametersActorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListAuditLogsParametersActorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "user" => Self::User,
            "service_account" => Self::ServiceAccount,
            "system" => Self::System,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListAuditLogsParametersStatus {
    Success,
    Failure,
    Denied,
    Unknown(String),
}
impl ListAuditLogsParametersStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Success => "success",
            Self::Failure => "failure",
            Self::Denied => "denied",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListAuditLogsParametersStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListAuditLogsParametersStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "success" => Self::Success,
            "failure" => Self::Failure,
            "denied" => Self::Denied,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListAuditLogsQuery = ListAuditLogsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AuditLogListResponse {
    #[serde(
        rename = "audit_logs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audit_logs: Option<Vec<AuditLog>>,

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

pub type ListAuditLogsResponse = AuditLogListResponse;

pub type ListAuditLogsItem = AuditLog;
