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
    /// Immutable event-time actor identity. Null for historical entries without a snapshot. New user events use crn:workspace:::organization/&lt;organization-uuid&gt;/user/&lt;user-uuid&gt;; service accounts use crn:iam::&lt;account-handle&gt;:service-account/&lt;name&gt;; assumed roles use crn:iam::&lt;account-handle&gt;:role/&lt;name&gt;, with the session identity in details.actor_session_crn. Historical entries retain their original CRNs, including legacy IAM organization identities. System actors use crn:iam::platform:system/&lt;service&gt;, with service names certificate, registry, secrets, queue, notifications and email.
    #[serde(
        rename = "actor_crn",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub actor_crn: Option<crate::Nullable<String>>,
    /// Immutable event-time resource identity. Null for historical entries without a snapshot and enumerated events whose target cannot be identified from event-time data, such as an unknown-email password reset or a lookup that never resolved a row. A known target UUID is retained in details.resource_id; it is never substituted for the immutable name in a CRN.
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
    /// Additional action-specific details. For assumed-role actors, actor_session_crn records crn:iam:::sts-session/&lt;id&gt; to correlate the event with its AssumeRole call.
    #[serde(rename = "details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::BTreeMap<String, serde_json::Value>>,
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
