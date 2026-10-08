//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserAddRequestInput {
    /// Email of the user to add
    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
    /// Groups to assign when the invitation is accepted. Each reference is validated in the caller organization before the invitation is created.
    #[serde(rename = "groups", default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupReferenceInput>>,
}
impl UserAddRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(email: String) -> Self {
        Self {
            email,
            tags: None,
            groups: None,
        }
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type GroupReferenceInput = String;

pub type AddUserBody = UserAddRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserAddResponse {
    #[serde(rename = "invitation")]
    pub invitation: Invitation,
    /// Always `invited` — adding a user always goes through an invitation the invitee has to accept, whether or not they already have a platform account.
    #[serde(rename = "status")]
    pub status: UserAddResponseStatus,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Invitation {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Email address of the invited user
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Groups the user will be added to upon accepting
    #[serde(rename = "groups", default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GroupSummary>>,

    #[serde(
        rename = "invited_by",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invited_by: Option<InvitationInvitedBy>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InvitationStatus>,

    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupSummary {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InvitationInvitedBy {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Human inviter email; omitted for machine identities.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Actual actor type. Assumed-role invitations record the session UUID in id.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<InvitationInvitedByType>,
    /// Canonical Workspace user CRN or account IAM service-account/session CRN captured when invited.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Owning account for a service-account or assumed-role inviter; omitted for a human inviter.
    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvitationInvitedByType {
    User,
    ServiceAccount,
    AssumedRole,
    Unknown(String),
}
impl InvitationInvitedByType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::ServiceAccount => "service_account",
            Self::AssumedRole => "assumed_role",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InvitationInvitedByType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvitationInvitedByType {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Expired,
    Cancelled,
    Unknown(String),
}
impl InvitationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InvitationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvitationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "accepted" => Self::Accepted,
            "expired" => Self::Expired,
            "cancelled" => Self::Cancelled,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserAddResponseStatus {
    Invited,
    Unknown(String),
}
impl UserAddResponseStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Invited => "invited",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UserAddResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserAddResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "invited" => Self::Invited,
            _ => Self::Unknown(value),
        })
    }
}

pub type AddUserResponse = UserAddResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserGroupAddRequestInput {
    #[serde(rename = "group")]
    pub group: GroupReferenceInput,
}
impl UserGroupAddRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(group: GroupReferenceInput) -> Self {
        Self { group }
    }
}

pub type AddUserToGroupBody = UserGroupAddRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountRoleAssignmentCreateRequestInput {
    #[serde(rename = "principal_type")]
    pub principal_type: AccountRoleAssignmentCreateRequestInputPrincipalType,
    /// Immutable UUID of a user or users-only group in this organization.
    #[serde(rename = "principal_id")]
    pub principal_id: String,
    /// Immutable UUID of a role owned by the target account.
    #[serde(rename = "role_id")]
    pub role_id: String,
}
impl AccountRoleAssignmentCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        principal_type: AccountRoleAssignmentCreateRequestInputPrincipalType,
        principal_id: String,
        role_id: String,
    ) -> Self {
        Self {
            principal_type,
            principal_id,
            role_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountRoleAssignmentCreateRequestInputPrincipalType {
    User,
    Group,
    Unknown(String),
}
impl AccountRoleAssignmentCreateRequestInputPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Group => "group",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AccountRoleAssignmentCreateRequestInputPrincipalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountRoleAssignmentCreateRequestInputPrincipalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "user" => Self::User,
            "group" => Self::Group,
            _ => Self::Unknown(value),
        })
    }
}

pub type AssignAccountRoleBody = AccountRoleAssignmentCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountRoleAssignmentResponse {
    #[serde(
        rename = "role_assignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub role_assignment: Option<AccountRoleAssignment>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountRoleAssignment {
    /// Organization-qualified identity of this assignment in its target account.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename = "account_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_id: Option<String>,

    #[serde(rename = "role_id", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,

    #[serde(rename = "role_name", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,

    #[serde(
        rename = "principal_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_type: Option<AccountRoleAssignmentPrincipalType>,

    #[serde(
        rename = "principal_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_id: Option<String>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountRoleAssignmentPrincipalType {
    User,
    Group,
    Unknown(String),
}
impl AccountRoleAssignmentPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Group => "group",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AccountRoleAssignmentPrincipalType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountRoleAssignmentPrincipalType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "user" => Self::User,
            "group" => Self::Group,
            _ => Self::Unknown(value),
        })
    }
}

pub type AssignAccountRoleResponse = AccountRoleAssignmentResponse;

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

pub type PolicyReferenceInput = String;

pub type AttachGroupPolicyBody = PolicyAttachRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OrganizationPolicyAttachRequestInput {
    /// Immutable UUID of the organization policy to attach.
    #[serde(rename = "policy_id")]
    pub policy_id: String,
}
impl OrganizationPolicyAttachRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(policy_id: String) -> Self {
        Self { policy_id }
    }
}

pub type AttachRolePolicyBody = OrganizationPolicyAttachRequestInput;

pub type AttachServiceAccountPolicyBody = OrganizationPolicyAttachRequestInput;

pub type AttachUserPolicyBody = PolicyAttachRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateAccountRequestInput {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "handle")]
    pub handle: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}
impl CreateAccountRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, handle: String) -> Self {
        Self {
            name,
            handle,
            description: None,
        }
    }
}

pub type CreateAccountBody = CreateAccountRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountResponse {
    #[serde(rename = "account", default, skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Account {
    /// Internal UUID. Used for joins; the handle is the public identifier.
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(
        rename = "organization_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_id: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Globally-unique, immutable handle (URL-safe identifier). Sent as X-Account-Id on every request that needs account context and embedded in CRNs.
    #[serde(rename = "handle", default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AccountStatus>,

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
    /// Canonical Workspace resource identity.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Returned on creation. Immutable ID of the AccountAdministrator role trusted only to the account creator.
    #[serde(
        rename = "bootstrap_role_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bootstrap_role_id: Option<String>,
    /// Returned on creation. Assume this role to administer the new account; source AssumeRole permission is still required.
    #[serde(
        rename = "bootstrap_role_crn",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bootstrap_role_crn: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AccountStatus {
    Active,
    Suspended,
    Deleted,
    Unknown(String),
}
impl AccountStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Deleted => "deleted",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AccountStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "active" => Self::Active,
            "suspended" => Self::Suspended,
            "deleted" => Self::Deleted,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateAccountResponse = AccountResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GroupCreateRequestInput {
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
impl GroupCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
        }
    }
}

pub type CreateGroupBody = GroupCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateGroupResult {
    #[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Group {
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

pub type CreateGroupResponse = CreateGroupResult;

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

pub type GetAccountResponse = AccountResponse;

pub type GetAccountResource = Account;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetAccountScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetAccountResourcesResult {
    #[serde(rename = "has_resources")]
    pub has_resources: bool,
}

pub type GetAccountResourcesResponse = GetAccountResourcesResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetGroupResult {
    #[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

pub type GetGroupResponse = GetGroupResult;

pub type GetGroupResource = Group;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetGroupScope {
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
    /// Canonical principal-scoped inline policy identity; named principals use their immutable name, users use UUID.
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
    User,
    Group,
    Unknown(String),
}
impl InlinePolicyPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Group => "group",
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
            "user" => Self::User,
            "group" => Self::Group,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetGroupInlinePolicyResponse = InlinePolicyResponse;

pub type GetGroupInlinePolicyResource = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetGroupInlinePolicyScope {}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInvitationResult {
    #[serde(
        rename = "invitation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invitation: Option<Invitation>,
}

pub type GetInvitationResponse = GetInvitationResult;

pub type GetInvitationResource = Invitation;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInvitationScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct OrganizationResponse {
    #[serde(
        rename = "organization",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization: Option<Organization>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Organization {
    /// Language for organization billing and operational emails, independent of each user's console preference.
    #[serde(rename = "language", default, skip_serializing_if = "Option::is_none")]
    pub language: Option<OrganizationLanguage>,
    /// IANA timezone for formatting organization emails. Does not change billing periods or resource schedules.
    #[serde(rename = "time_zone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// ID of the organization owner
    #[serde(rename = "owner_id", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// Lifecycle state. A newly created organization is `pending` until its owner has verified a phone number and attached a payment method; until then every resource API refuses it with `ORGANIZATION_ONBOARDING_REQUIRED`. `suspended` is a billing or administrative hold, and `terminated` is irreversible.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrganizationStatus>,
    /// Why the organization is suspended; absent unless it is. The two are the same `status` but not the same situation — a `billing` hold is one the customer can clear by settling their account, and the platform still grants organization context for it so they can reach billing to do so. A `manual` hold is an operator decision and grants nothing.
    #[serde(
        rename = "suspension_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suspension_reason: Option<OrganizationSuspensionReason>,

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
    /// Canonical Workspace resource identity.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationLanguage {
    En,
    PtBR,
    Es,
    Unknown(String),
}
impl OrganizationLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::En => "en",
            Self::PtBR => "pt-BR",
            Self::Es => "es",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrganizationLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "en" => Self::En,
            "pt-BR" => Self::PtBR,
            "es" => Self::Es,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationStatus {
    Pending,
    Active,
    Suspended,
    Terminated,
    Unknown(String),
}
impl OrganizationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Terminated => "terminated",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrganizationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "active" => Self::Active,
            "suspended" => Self::Suspended,
            "terminated" => Self::Terminated,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationSuspensionReason {
    Billing,
    Manual,
    Unknown(String),
}
impl OrganizationSuspensionReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Billing => "billing",
            Self::Manual => "manual",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrganizationSuspensionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationSuspensionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "billing" => Self::Billing,
            "manual" => Self::Manual,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetOrganizationResponse = OrganizationResponse;

pub type GetOrganizationResource = Organization;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetOrganizationScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

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
pub struct GetUserResult {
    #[serde(rename = "user", default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct User {
    /// Globally unique permanent login handle; empty until the user completes username selection.
    #[serde(rename = "username", default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "linux_identity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub linux_identity: Option<LinuxIdentity>,

    #[serde(rename = "added_at", default, skip_serializing_if = "Option::is_none")]
    pub added_at: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LinuxIdentity {
    /// Home directory derived from the permanent username, as /home/&lt;username&gt;. Numeric file ownership is defined by UID and GID.
    #[serde(rename = "home_directory")]
    pub home_directory: String,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "uid")]
    pub uid: i64,

    #[serde(rename = "gid")]
    pub gid: i64,
}

pub type GetUserResponse = GetUserResult;

pub type GetUserResource = User;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetUserScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetUserInlinePolicyResponse = InlinePolicyResponse;

pub type GetUserInlinePolicyResource = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetUserInlinePolicyScope {}

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
    User,
    Unknown(String),
}
impl PermissionBoundaryPrincipalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
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
            "user" => Self::User,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetUserPermissionBoundaryResponse = PermissionBoundaryResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountRoleAssignmentListResponse {
    #[serde(
        rename = "role_assignments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub role_assignments: Option<Vec<AccountRoleAssignment>>,
}

pub type ListAccountRoleAssignmentsResponse = AccountRoleAssignmentListResponse;

pub type ListAccountRoleAssignmentsItem = AccountRoleAssignment;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountRoleListResponse {
    #[serde(
        rename = "account_roles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_roles: Option<Vec<AccountRole>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountRole {
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
        rename = "account_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,

    #[serde(rename = "role_id", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,

    #[serde(rename = "role_name", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,

    #[serde(rename = "role_crn", default, skip_serializing_if = "Option::is_none")]
    pub role_crn: Option<String>,
}

pub type ListAccountRolesResponse = AccountRoleListResponse;

pub type ListAccountRolesItem = AccountRole;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListAccountsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListAccountsQuery = ListAccountsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountListResponse {
    #[serde(rename = "accounts", default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Account>>,

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

pub type ListAccountsResponse = AccountListResponse;

pub type ListAccountsItem = Account;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListGroupInlinePoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListGroupInlinePoliciesQuery = ListGroupInlinePoliciesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InlinePolicyListResponse {
    #[serde(
        rename = "inline_policies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_policies: Option<Vec<InlinePolicy>>,
}

pub type ListGroupInlinePoliciesResponse = InlinePolicyListResponse;

pub type ListGroupInlinePoliciesItem = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListGroupPoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListGroupPoliciesQuery = ListGroupPoliciesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PrincipalPoliciesListResponse {
    #[serde(rename = "policies", default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<Policy>>,
}

pub type ListGroupPoliciesResponse = PrincipalPoliciesListResponse;

pub type ListGroupPoliciesItem = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListGroupUsersParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListGroupUsersQuery = ListGroupUsersParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupUsersListResponse {
    #[serde(rename = "users", default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GroupUser>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupUser {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// CRN of the user
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Display name of the user
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "added_at", default, skip_serializing_if = "Option::is_none")]
    pub added_at: Option<String>,
}

pub type ListGroupUsersResponse = GroupUsersListResponse;

pub type ListGroupUsersItem = GroupUser;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListGroupsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListGroupsQuery = ListGroupsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupListResponse {
    #[serde(rename = "groups", default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListGroupsResponse = GroupListResponse;

pub type ListGroupsItem = Group;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInvitationsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListInvitationsQuery = ListInvitationsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InvitationListResponse {
    #[serde(
        rename = "invitations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invitations: Option<Vec<Invitation>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListInvitationsResponse = InvitationListResponse;

pub type ListInvitationsItem = Invitation;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListOrganizationsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListOrganizationsQuery = ListOrganizationsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct OrganizationListResponse {
    #[serde(
        rename = "organizations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organizations: Option<Vec<OrganizationWithMembership>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct OrganizationWithMembership {
    /// Language for organization billing and operational emails, independent of each user's console preference.
    #[serde(rename = "language", default, skip_serializing_if = "Option::is_none")]
    pub language: Option<OrganizationWithMembershipLanguage>,
    /// IANA timezone for formatting organization emails. Does not change billing periods or resource schedules.
    #[serde(rename = "time_zone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// ID of the organization owner
    #[serde(rename = "owner_id", default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// Lifecycle state. A newly created organization is `pending` until its owner has verified a phone number and attached a payment method; until then every resource API refuses it with `ORGANIZATION_ONBOARDING_REQUIRED`. `suspended` is a billing or administrative hold, and `terminated` is irreversible.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrganizationWithMembershipStatus>,
    /// Why the organization is suspended; absent unless it is. The two are the same `status` but not the same situation — a `billing` hold is one the customer can clear by settling their account, and the platform still grants organization context for it so they can reach billing to do so. A `manual` hold is an operator decision and grants nothing.
    #[serde(
        rename = "suspension_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub suspension_reason: Option<OrganizationWithMembershipSuspensionReason>,

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
    /// Canonical Workspace resource identity.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationWithMembershipLanguage {
    En,
    PtBR,
    Es,
    Unknown(String),
}
impl OrganizationWithMembershipLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::En => "en",
            Self::PtBR => "pt-BR",
            Self::Es => "es",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrganizationWithMembershipLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationWithMembershipLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "en" => Self::En,
            "pt-BR" => Self::PtBR,
            "es" => Self::Es,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationWithMembershipStatus {
    Pending,
    Active,
    Suspended,
    Terminated,
    Unknown(String),
}
impl OrganizationWithMembershipStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Terminated => "terminated",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrganizationWithMembershipStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationWithMembershipStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "active" => Self::Active,
            "suspended" => Self::Suspended,
            "terminated" => Self::Terminated,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationWithMembershipSuspensionReason {
    Billing,
    Manual,
    Unknown(String),
}
impl OrganizationWithMembershipSuspensionReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Billing => "billing",
            Self::Manual => "manual",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrganizationWithMembershipSuspensionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationWithMembershipSuspensionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "billing" => Self::Billing,
            "manual" => Self::Manual,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListOrganizationsResponse = OrganizationListResponse;

pub type ListOrganizationsItem = OrganizationWithMembership;

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

pub type ListPoliciesResponse = PolicyListResponse;

pub type ListPoliciesItem = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPolicyGroupsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListPolicyGroupsQuery = ListPolicyGroupsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PolicyGroupsListResponse {
    #[serde(rename = "groups", default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<Group>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListPolicyGroupsResponse = PolicyGroupsListResponse;

pub type ListPolicyGroupsItem = Group;

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
    pub roles: Option<Vec<AccountPrincipalReference>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AccountPrincipalReference {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

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
}

pub type ListPolicyRolesResponse = PolicyRolesListResponse;

pub type ListPolicyRolesItem = AccountPrincipalReference;

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
    pub service_accounts: Option<Vec<AccountPrincipalReference>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListPolicyServiceAccountsResponse = PolicyServiceAccountsListResponse;

pub type ListPolicyServiceAccountsItem = AccountPrincipalReference;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPolicyUsersParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListPolicyUsersQuery = ListPolicyUsersParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PolicyUsersListResponse {
    #[serde(rename = "users", default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListPolicyUsersResponse = PolicyUsersListResponse;

pub type ListPolicyUsersItem = User;

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
pub struct ListServiceAccountPoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListServiceAccountPoliciesQuery = ListServiceAccountPoliciesParameters;

pub type ListServiceAccountPoliciesResponse = PrincipalPoliciesListResponse;

pub type ListServiceAccountPoliciesItem = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListUserGroupsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListUserGroupsQuery = ListUserGroupsParameters;

pub type ListUserGroupsResponse = GroupListResponse;

pub type ListUserGroupsItem = Group;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListUserInlinePoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListUserInlinePoliciesQuery = ListUserInlinePoliciesParameters;

pub type ListUserInlinePoliciesResponse = InlinePolicyListResponse;

pub type ListUserInlinePoliciesItem = InlinePolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListUserPoliciesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListUserPoliciesQuery = ListUserPoliciesParameters;

pub type ListUserPoliciesResponse = PrincipalPoliciesListResponse;

pub type ListUserPoliciesItem = Policy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListUsersParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListUsersQuery = ListUsersParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UserListResponse {
    #[serde(rename = "users", default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListUsersResponse = UserListResponse;

pub type ListUsersItem = User;

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

pub type PutGroupInlinePolicyBody = PutInlinePolicyRequestInput;

pub type PutGroupInlinePolicyResponse = InlinePolicyResponse;

pub type PutUserInlinePolicyBody = PutInlinePolicyRequestInput;

pub type PutUserInlinePolicyResponse = InlinePolicyResponse;

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

pub type SetUserPermissionBoundaryBody = SetBoundaryRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateAccountRequestInput {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

pub type UpdateAccountBody = UpdateAccountRequestInput;

pub type UpdateAccountResponse = AccountResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GroupUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

pub type UpdateGroupBody = GroupUpdateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateGroupResult {
    #[serde(rename = "group", default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
}

pub type UpdateGroupResponse = UpdateGroupResult;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct OrganizationUpdateRequestInput {
    /// Language for organization billing and operational emails, independent of each user's console preference.
    #[serde(rename = "language", default, skip_serializing_if = "Option::is_none")]
    pub language: Option<OrganizationUpdateRequestInputLanguage>,
    /// IANA timezone for formatting organization emails. Does not change billing periods or resource schedules.
    #[serde(rename = "time_zone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Google reCAPTCHA token for bot protection
    #[serde(rename = "captcha_token")]
    pub captcha_token: String,
}
impl std::fmt::Debug for OrganizationUpdateRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("OrganizationUpdateRequestInput");
        d.field("language", &self.language);
        d.field("time_zone", &self.time_zone);
        d.field("name", &self.name);
        d.field("description", &self.description);
        d.field("captcha_token", &"[REDACTED]");
        d.finish()
    }
}
impl OrganizationUpdateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(captcha_token: String) -> Self {
        Self {
            language: None,
            time_zone: None,
            name: None,
            description: None,
            captcha_token,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrganizationUpdateRequestInputLanguage {
    En,
    PtBR,
    Es,
    Unknown(String),
}
impl OrganizationUpdateRequestInputLanguage {
    pub fn as_str(&self) -> &str {
        match self {
            Self::En => "en",
            Self::PtBR => "pt-BR",
            Self::Es => "es",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrganizationUpdateRequestInputLanguage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrganizationUpdateRequestInputLanguage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "en" => Self::En,
            "pt-BR" => Self::PtBR,
            "es" => Self::Es,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateOrganizationBody = OrganizationUpdateRequestInput;

pub type UpdateOrganizationResponse = OrganizationResponse;

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
