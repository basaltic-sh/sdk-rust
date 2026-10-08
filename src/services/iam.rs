//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::iam as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://iam.basaltic.sh";
static OP_0: Operation = Operation {
    id: "assumeRole",
    method: "POST",
    path: "/v1/assume-role",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "assumeRoleWithWebIdentity",
    method: "POST",
    path: "/v1/assume-role-with-web-identity",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "attachRolePolicy",
    method: "POST",
    path: "/v1/roles/{role_id}/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "attachServiceAccountPolicy",
    method: "POST",
    path: "/v1/service-accounts/{service_account_id}/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "authorizeOAuthClient",
    method: "POST",
    path: "/v1/oauth/authorize",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "createPersonalSSHKey",
    method: "POST",
    path: "/v1/auth/ssh-keys",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "createPolicy",
    method: "POST",
    path: "/v1/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "createRole",
    method: "POST",
    path: "/v1/roles",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "createServiceAccount",
    method: "POST",
    path: "/v1/service-accounts",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "createServiceAccountCredential",
    method: "POST",
    path: "/v1/service-accounts/{service_account_id}/credentials",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "createServiceAccountSSHKey",
    method: "POST",
    path: "/v1/service-accounts/{service_account_id}/ssh-keys",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "deletePersonalSSHKey",
    method: "DELETE",
    path: "/v1/auth/ssh-keys/{ssh_key_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "deletePolicy",
    method: "DELETE",
    path: "/v1/policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "deleteRole",
    method: "DELETE",
    path: "/v1/roles/{role_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "deleteRoleInlinePolicy",
    method: "DELETE",
    path: "/v1/roles/{role_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "deleteServiceAccount",
    method: "DELETE",
    path: "/v1/service-accounts/{service_account_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "deleteServiceAccountCredential",
    method: "DELETE",
    path: "/v1/service-accounts/{service_account_id}/credentials/{credential_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "deleteServiceAccountInlinePolicy",
    method: "DELETE",
    path: "/v1/service-accounts/{service_account_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "deleteServiceAccountSSHKey",
    method: "DELETE",
    path: "/v1/service-accounts/{service_account_id}/ssh-keys/{ssh_key_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_19: Operation = Operation {
    id: "detachRolePolicy",
    method: "DELETE",
    path: "/v1/roles/{role_id}/policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_20: Operation = Operation {
    id: "detachServiceAccountPolicy",
    method: "DELETE",
    path: "/v1/service-accounts/{service_account_id}/policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_21: Operation = Operation {
    id: "getOAuthToken",
    method: "POST",
    path: "/v1/oauth/token",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_22: Operation = Operation {
    id: "getPersonalLinuxIdentity",
    method: "GET",
    path: "/v1/auth/linux-identity",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_23: Operation = Operation {
    id: "getPolicy",
    method: "GET",
    path: "/v1/policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_24: Operation = Operation {
    id: "getRole",
    method: "GET",
    path: "/v1/roles/{role_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_25: Operation = Operation {
    id: "getRoleInlinePolicy",
    method: "GET",
    path: "/v1/roles/{role_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_26: Operation = Operation {
    id: "getRolePermissionBoundary",
    method: "GET",
    path: "/v1/roles/{role_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_27: Operation = Operation {
    id: "getSTSSession",
    method: "GET",
    path: "/v1/sts-sessions/{session_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_28: Operation = Operation {
    id: "getServiceAccount",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_29: Operation = Operation {
    id: "getServiceAccountInlinePolicy",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_30: Operation = Operation {
    id: "getServiceAccountLinuxIdentity",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}/linux-identity",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_31: Operation = Operation {
    id: "getServiceAccountPermissionBoundary",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_32: Operation = Operation {
    id: "listPersonalSSHKeys",
    method: "GET",
    path: "/v1/auth/ssh-keys",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_33: Operation = Operation {
    id: "listPolicies",
    method: "GET",
    path: "/v1/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
    ],
};
static OP_34: Operation = Operation {
    id: "listPolicyRoles",
    method: "GET",
    path: "/v1/policies/{policy_id}/roles",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
    ],
};
static OP_35: Operation = Operation {
    id: "listPolicyServiceAccounts",
    method: "GET",
    path: "/v1/policies/{policy_id}/service-accounts",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
    ],
};
static OP_36: Operation = Operation {
    id: "listRegions",
    method: "GET",
    path: "/v1/regions",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
    ],
};
static OP_37: Operation = Operation {
    id: "listRoleInlinePolicies",
    method: "GET",
    path: "/v1/roles/{role_id}/inline-policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
    ],
};
static OP_38: Operation = Operation {
    id: "listRolePolicies",
    method: "GET",
    path: "/v1/roles/{role_id}/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
    ],
};
static OP_39: Operation = Operation {
    id: "listRoles",
    method: "GET",
    path: "/v1/roles",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
    ],
};
static OP_40: Operation = Operation {
    id: "listSTSSessions",
    method: "GET",
    path: "/v1/sts-sessions",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "role",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "principal",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "principal_type",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "active_only",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
    ],
};
static OP_41: Operation = Operation {
    id: "listServiceAccountCredentials",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}/credentials",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
    ],
};
static OP_42: Operation = Operation {
    id: "listServiceAccountInlinePolicies",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}/inline-policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
    ],
};
static OP_43: Operation = Operation {
    id: "listServiceAccountPolicies",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
    ],
};
static OP_44: Operation = Operation {
    id: "listServiceAccountSSHKeys",
    method: "GET",
    path: "/v1/service-accounts/{service_account_id}/ssh-keys",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_45: Operation = Operation {
    id: "listServiceAccounts",
    method: "GET",
    path: "/v1/service-accounts",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
    ],
};
static OP_46: Operation = Operation {
    id: "putRoleInlinePolicy",
    method: "PUT",
    path: "/v1/roles/{role_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_47: Operation = Operation {
    id: "putServiceAccountInlinePolicy",
    method: "PUT",
    path: "/v1/service-accounts/{service_account_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_48: Operation = Operation {
    id: "removeRolePermissionBoundary",
    method: "DELETE",
    path: "/v1/roles/{role_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_49: Operation = Operation {
    id: "removeServiceAccountPermissionBoundary",
    method: "DELETE",
    path: "/v1/service-accounts/{service_account_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_50: Operation = Operation {
    id: "revokeOAuthToken",
    method: "POST",
    path: "/v1/oauth/revoke",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_51: Operation = Operation {
    id: "revokeSTSSession",
    method: "DELETE",
    path: "/v1/sts-sessions/{session_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_52: Operation = Operation {
    id: "setRolePermissionBoundary",
    method: "PUT",
    path: "/v1/roles/{role_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_53: Operation = Operation {
    id: "setServiceAccountPermissionBoundary",
    method: "PUT",
    path: "/v1/service-accounts/{service_account_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_54: Operation = Operation {
    id: "updatePolicy",
    method: "PATCH",
    path: "/v1/policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_55: Operation = Operation {
    id: "updateRole",
    method: "PATCH",
    path: "/v1/roles/{role_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_56: Operation = Operation {
    id: "updateServiceAccount",
    method: "PATCH",
    path: "/v1/service-accounts/{service_account_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct IamService {
    pub(crate) client: Client,
}
impl IamService {
    /// Assume role
    pub fn assume_role(&self, body: &m::AssumeRoleBody) -> Request<m::AssumeRoleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_0,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Assume role with web identity
    pub fn assume_role_with_web_identity(
        &self,
        body: &m::AssumeRoleWithWebIdentityBody,
    ) -> Request<m::AssumeRoleWithWebIdentityResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_1,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Attach policy to role
    pub fn attach_role_policy(
        &self,
        role_id: &str,
        body: &m::AttachRolePolicyBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_2,
            &[("role_id", role_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Attach policy to service account
    pub fn attach_service_account_policy(
        &self,
        service_account_id: &str,
        body: &m::AttachServiceAccountPolicyBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_3,
            &[("service_account_id", service_account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Approve a CLI login and issue an authorization code
    pub fn authorize_oauth_client(
        &self,
        body: &m::AuthorizeOAuthClientBody,
    ) -> Request<m::AuthorizeOAuthClientResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_4,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Add personal SSH key
    pub fn create_personal_ssh_key(
        &self,
        body: &m::CreatePersonalSSHKeyBody,
    ) -> Request<m::CreatePersonalSSHKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_5,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create policy
    pub fn create_policy(&self, body: &m::CreatePolicyBody) -> Request<m::CreatePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_6,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create role
    pub fn create_role(&self, body: &m::CreateRoleBody) -> Request<m::CreateRoleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_7,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create service account
    pub fn create_service_account(
        &self,
        body: &m::CreateServiceAccountBody,
    ) -> Request<m::CreateServiceAccountResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_8,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create credential
    pub fn create_service_account_credential(
        &self,
        service_account_id: &str,
        body: &m::CreateServiceAccountCredentialBody,
    ) -> Request<m::CreateServiceAccountCredentialResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_9,
            &[("service_account_id", service_account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Add service-account SSH key
    pub fn create_service_account_ssh_key(
        &self,
        service_account_id: &str,
        body: &m::CreateServiceAccountSSHKeyBody,
    ) -> Request<m::CreateServiceAccountSSHKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_10,
            &[("service_account_id", service_account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Revoke personal SSH key
    pub fn delete_personal_ssh_key(&self, ssh_key_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_11,
            &[("ssh_key_id", ssh_key_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete policy
    pub fn delete_policy(&self, policy_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_12,
            &[("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete role
    pub fn delete_role(&self, role_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_13,
            &[("role_id", role_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a role's inline policy by name
    pub fn delete_role_inline_policy(&self, role_id: &str, policy_name: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_14,
            &[("role_id", role_id), ("policy_name", policy_name)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete service account
    pub fn delete_service_account(&self, service_account_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_15,
            &[("service_account_id", service_account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete credential
    pub fn delete_service_account_credential(
        &self,
        service_account_id: &str,
        credential_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_16,
            &[
                ("service_account_id", service_account_id),
                ("credential_id", credential_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a service account's inline policy by name
    pub fn delete_service_account_inline_policy(
        &self,
        service_account_id: &str,
        policy_name: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_17,
            &[
                ("service_account_id", service_account_id),
                ("policy_name", policy_name),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Revoke service-account SSH key
    pub fn delete_service_account_ssh_key(
        &self,
        service_account_id: &str,
        ssh_key_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_18,
            &[
                ("service_account_id", service_account_id),
                ("ssh_key_id", ssh_key_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach policy from role
    pub fn detach_role_policy(&self, role_id: &str, policy_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_19,
            &[("role_id", role_id), ("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach policy from service account
    pub fn detach_service_account_policy(
        &self,
        service_account_id: &str,
        policy_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_20,
            &[
                ("service_account_id", service_account_id),
                ("policy_id", policy_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Exchange an access key for a bearer token
    pub fn get_oauth_token(
        &self,
        body: &m::GetOAuthTokenBody,
    ) -> Request<m::GetOAuthTokenResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_21,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get personal Linux identity
    pub fn get_personal_linux_identity(&self) -> Request<m::GetPersonalLinuxIdentityResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_22,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get policy
    pub fn get_policy(&self, policy_id: &str) -> Request<m::GetPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_23,
            &[("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_policy_by_reference(
        &self,
        reference: &str,
        scope: &m::GetPolicyScope,
    ) -> Request<m::GetPolicyResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_23,
                &[("policy_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_33,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("policy"),
            "policies",
        );
        Request::reference(core, extract)
    }
    /// Get role
    pub fn get_role(&self, role_id: &str) -> Request<m::GetRoleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_24,
            &[("role_id", role_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_role_by_reference(
        &self,
        reference: &str,
        scope: &m::GetRoleScope,
    ) -> Request<m::GetRoleResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_24,
                &[("role_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_39,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("role"),
            "roles",
        );
        Request::reference(core, extract)
    }
    /// Get a role's inline policy by name
    pub fn get_role_inline_policy(
        &self,
        role_id: &str,
        policy_name: &str,
    ) -> Request<m::GetRoleInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_25,
            &[("role_id", role_id), ("policy_name", policy_name)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_role_inline_policy_by_reference(
        &self,
        role_id: &str,
        reference: &str,
        scope: &m::GetRoleInlinePolicyScope,
    ) -> Request<m::GetRoleInlinePolicyResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_25,
                &[("role_id", role_id), ("policy_name", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_37,
                &[("role_id", role_id)],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("inline_policy"),
            "inline_policies",
        );
        Request::reference(core, extract)
    }
    /// Get a role's permission boundary
    pub fn get_role_permission_boundary(
        &self,
        role_id: &str,
    ) -> Request<m::GetRolePermissionBoundaryResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_26,
            &[("role_id", role_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get STS session
    pub fn get_sts_session(&self, session_id: &str) -> Request<m::GetSTSSessionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_27,
            &[("session_id", session_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_sts_session_by_reference(
        &self,
        reference: &str,
        scope: &m::GetSTSSessionScope,
    ) -> Request<m::GetSTSSessionResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_27,
                &[("session_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_40,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("sts_session"),
            "sts_sessions",
        );
        Request::reference(core, extract)
    }
    /// Get service account
    pub fn get_service_account(
        &self,
        service_account_id: &str,
    ) -> Request<m::GetServiceAccountResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_28,
            &[("service_account_id", service_account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_service_account_by_reference(
        &self,
        reference: &str,
        scope: &m::GetServiceAccountScope,
    ) -> Request<m::GetServiceAccountResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_28,
                &[("service_account_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_45,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("service_account"),
            "service_accounts",
        );
        Request::reference(core, extract)
    }
    /// Get a service account's inline policy by name
    pub fn get_service_account_inline_policy(
        &self,
        service_account_id: &str,
        policy_name: &str,
    ) -> Request<m::GetServiceAccountInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_29,
            &[
                ("service_account_id", service_account_id),
                ("policy_name", policy_name),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_service_account_inline_policy_by_reference(
        &self,
        service_account_id: &str,
        reference: &str,
        scope: &m::GetServiceAccountInlinePolicyScope,
    ) -> Request<m::GetServiceAccountInlinePolicyResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_29,
                &[
                    ("service_account_id", service_account_id),
                    ("policy_name", reference),
                ],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_42,
                &[("service_account_id", service_account_id)],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("inline_policy"),
            "inline_policies",
        );
        Request::reference(core, extract)
    }
    /// Get serviceaccount Linux identity
    pub fn get_service_account_linux_identity(
        &self,
        service_account_id: &str,
    ) -> Request<m::GetServiceAccountLinuxIdentityResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_30,
            &[("service_account_id", service_account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get a service account's permission boundary
    pub fn get_service_account_permission_boundary(
        &self,
        service_account_id: &str,
    ) -> Request<m::GetServiceAccountPermissionBoundaryResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_31,
            &[("service_account_id", service_account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List personal SSH keys
    pub fn list_personal_ssh_keys(
        &self,
    ) -> PagedRequest<m::ListPersonalSSHKeysResponse, m::ListPersonalSSHKeysItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_32,
                &[],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "ssh_keys",
        )
    }
    /// List policies
    pub fn list_policies(
        &self,
        query: &m::ListPoliciesQuery,
    ) -> PagedRequest<m::ListPoliciesResponse, m::ListPoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_33,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
        )
    }
    /// List roles with policy
    pub fn list_policy_roles(
        &self,
        policy_id: &str,
        query: &m::ListPolicyRolesQuery,
    ) -> PagedRequest<m::ListPolicyRolesResponse, m::ListPolicyRolesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_34,
                &[("policy_id", policy_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "roles",
        )
    }
    /// List service accounts with policy
    pub fn list_policy_service_accounts(
        &self,
        policy_id: &str,
        query: &m::ListPolicyServiceAccountsQuery,
    ) -> PagedRequest<m::ListPolicyServiceAccountsResponse, m::ListPolicyServiceAccountsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_35,
                &[("policy_id", policy_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "service_accounts",
        )
    }
    /// List regions (legacy IAM)
    pub fn list_regions(
        &self,
        query: &m::ListRegionsQuery,
    ) -> PagedRequest<m::ListRegionsResponse, m::ListRegionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_36,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "regions",
        )
    }
    /// List a role's inline policies
    pub fn list_role_inline_policies(
        &self,
        role_id: &str,
        query: &m::ListRoleInlinePoliciesQuery,
    ) -> PagedRequest<m::ListRoleInlinePoliciesResponse, m::ListRoleInlinePoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_37,
                &[("role_id", role_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "inline_policies",
        )
    }
    /// List role policies
    pub fn list_role_policies(
        &self,
        role_id: &str,
        query: &m::ListRolePoliciesQuery,
    ) -> PagedRequest<m::ListRolePoliciesResponse, m::ListRolePoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_38,
                &[("role_id", role_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
        )
    }
    /// List roles
    pub fn list_roles(
        &self,
        query: &m::ListRolesQuery,
    ) -> PagedRequest<m::ListRolesResponse, m::ListRolesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_39,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "roles",
        )
    }
    /// List STS sessions
    pub fn list_sts_sessions(
        &self,
        query: &m::ListSTSSessionsQuery,
    ) -> PagedRequest<m::ListSTSSessionsResponse, m::ListSTSSessionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_40,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "sts_sessions",
        )
    }
    /// List credentials
    pub fn list_service_account_credentials(
        &self,
        service_account_id: &str,
        query: &m::ListServiceAccountCredentialsQuery,
    ) -> PagedRequest<m::ListServiceAccountCredentialsResponse, m::ListServiceAccountCredentialsItem>
    {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_41,
                &[("service_account_id", service_account_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "credentials",
        )
    }
    /// List a service account's inline policies
    pub fn list_service_account_inline_policies(
        &self,
        service_account_id: &str,
        query: &m::ListServiceAccountInlinePoliciesQuery,
    ) -> PagedRequest<
        m::ListServiceAccountInlinePoliciesResponse,
        m::ListServiceAccountInlinePoliciesItem,
    > {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_42,
                &[("service_account_id", service_account_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "inline_policies",
        )
    }
    /// List service account policies
    pub fn list_service_account_policies(
        &self,
        service_account_id: &str,
        query: &m::ListServiceAccountPoliciesQuery,
    ) -> PagedRequest<m::ListServiceAccountPoliciesResponse, m::ListServiceAccountPoliciesItem>
    {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_43,
                &[("service_account_id", service_account_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
        )
    }
    /// List service-account SSH keys
    pub fn list_service_account_ssh_keys(
        &self,
        service_account_id: &str,
    ) -> PagedRequest<m::ListServiceAccountSSHKeysResponse, m::ListServiceAccountSSHKeysItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_44,
                &[("service_account_id", service_account_id)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "ssh_keys",
        )
    }
    /// List service accounts
    pub fn list_service_accounts(
        &self,
        query: &m::ListServiceAccountsQuery,
    ) -> PagedRequest<m::ListServiceAccountsResponse, m::ListServiceAccountsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "iam",
                ENDPOINT,
                &OP_45,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "service_accounts",
        )
    }
    /// Create or replace a role's inline policy
    pub fn put_role_inline_policy(
        &self,
        role_id: &str,
        policy_name: &str,
        body: &m::PutRoleInlinePolicyBody,
    ) -> Request<m::PutRoleInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_46,
            &[("role_id", role_id), ("policy_name", policy_name)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create or replace a service account's inline policy
    pub fn put_service_account_inline_policy(
        &self,
        service_account_id: &str,
        policy_name: &str,
        body: &m::PutServiceAccountInlinePolicyBody,
    ) -> Request<m::PutServiceAccountInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_47,
            &[
                ("service_account_id", service_account_id),
                ("policy_name", policy_name),
            ],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Remove a role's permission boundary
    pub fn remove_role_permission_boundary(&self, role_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_48,
            &[("role_id", role_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Remove a service account's permission boundary
    pub fn remove_service_account_permission_boundary(
        &self,
        service_account_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_49,
            &[("service_account_id", service_account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Revoke a bearer token
    pub fn revoke_oauth_token(&self, body: &m::RevokeOAuthTokenBody) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_50,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Revoke STS session
    pub fn revoke_sts_session(
        &self,
        session_id: &str,
        body: Option<&m::RevokeSTSSessionBody>,
    ) -> Request<m::RevokeSTSSessionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_51,
            &[("session_id", session_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set a role's permission boundary
    pub fn set_role_permission_boundary(
        &self,
        role_id: &str,
        body: &m::SetRolePermissionBoundaryBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_52,
            &[("role_id", role_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set a service account's permission boundary
    pub fn set_service_account_permission_boundary(
        &self,
        service_account_id: &str,
        body: &m::SetServiceAccountPermissionBoundaryBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_53,
            &[("service_account_id", service_account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update policy
    pub fn update_policy(
        &self,
        policy_id: &str,
        body: &m::UpdatePolicyBody,
    ) -> Request<m::UpdatePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_54,
            &[("policy_id", policy_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update role
    pub fn update_role(
        &self,
        role_id: &str,
        body: &m::UpdateRoleBody,
    ) -> Request<m::UpdateRoleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_55,
            &[("role_id", role_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update service account
    pub fn update_service_account(
        &self,
        service_account_id: &str,
        body: &m::UpdateServiceAccountBody,
    ) -> Request<m::UpdateServiceAccountResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "iam",
            ENDPOINT,
            &OP_56,
            &[("service_account_id", service_account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
