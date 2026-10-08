//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::workspace as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://workspace.basaltic.sh";
static OP_0: Operation = Operation {
    id: "addUser",
    method: "POST",
    path: "/v1/users",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "addUserToGroup",
    method: "POST",
    path: "/v1/users/{user_id}/groups",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "assignAccountRole",
    method: "POST",
    path: "/v1/accounts/{account_id}/role-assignments",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "attachGroupPolicy",
    method: "POST",
    path: "/v1/groups/{group_id}/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
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
static OP_5: Operation = Operation {
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
static OP_6: Operation = Operation {
    id: "attachUserPolicy",
    method: "POST",
    path: "/v1/users/{user_id}/policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "cancelInvitation",
    method: "DELETE",
    path: "/v1/invitations/{invitation_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "createAccount",
    method: "POST",
    path: "/v1/accounts",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "createGroup",
    method: "POST",
    path: "/v1/groups",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
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
static OP_11: Operation = Operation {
    id: "deleteAccount",
    method: "DELETE",
    path: "/v1/accounts/{account_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "deleteGroup",
    method: "DELETE",
    path: "/v1/groups/{group_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "deleteGroupInlinePolicy",
    method: "DELETE",
    path: "/v1/groups/{group_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "deleteOrganization",
    method: "DELETE",
    path: "/v1/organizations/{organization_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
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
static OP_16: Operation = Operation {
    id: "deleteUserInlinePolicy",
    method: "DELETE",
    path: "/v1/users/{user_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "detachGroupPolicy",
    method: "DELETE",
    path: "/v1/groups/{group_id}/policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
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
static OP_19: Operation = Operation {
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
static OP_20: Operation = Operation {
    id: "detachUserPolicy",
    method: "DELETE",
    path: "/v1/users/{user_id}/policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_21: Operation = Operation {
    id: "getAccount",
    method: "GET",
    path: "/v1/accounts/{account_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_22: Operation = Operation {
    id: "getAccountResources",
    method: "GET",
    path: "/v1/accounts/{account_id}/resources",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_23: Operation = Operation {
    id: "getGroup",
    method: "GET",
    path: "/v1/groups/{group_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_24: Operation = Operation {
    id: "getGroupInlinePolicy",
    method: "GET",
    path: "/v1/groups/{group_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_25: Operation = Operation {
    id: "getInvitation",
    method: "GET",
    path: "/v1/invitations/{invitation_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_26: Operation = Operation {
    id: "getOrganization",
    method: "GET",
    path: "/v1/organizations/{organization_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_27: Operation = Operation {
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
static OP_28: Operation = Operation {
    id: "getUser",
    method: "GET",
    path: "/v1/users/{user_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_29: Operation = Operation {
    id: "getUserInlinePolicy",
    method: "GET",
    path: "/v1/users/{user_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_30: Operation = Operation {
    id: "getUserPermissionBoundary",
    method: "GET",
    path: "/v1/users/{user_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_31: Operation = Operation {
    id: "listAccountRoleAssignments",
    method: "GET",
    path: "/v1/accounts/{account_id}/role-assignments",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_32: Operation = Operation {
    id: "listAccountRoles",
    method: "GET",
    path: "/v1/account-roles",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_33: Operation = Operation {
    id: "listAccounts",
    method: "GET",
    path: "/v1/accounts",
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
    id: "listGroupInlinePolicies",
    method: "GET",
    path: "/v1/groups/{group_id}/inline-policies",
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
static OP_35: Operation = Operation {
    id: "listGroupPolicies",
    method: "GET",
    path: "/v1/groups/{group_id}/policies",
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
static OP_36: Operation = Operation {
    id: "listGroupUsers",
    method: "GET",
    path: "/v1/groups/{group_id}/users",
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
static OP_37: Operation = Operation {
    id: "listGroups",
    method: "GET",
    path: "/v1/groups",
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
static OP_38: Operation = Operation {
    id: "listInvitations",
    method: "GET",
    path: "/v1/invitations",
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
static OP_39: Operation = Operation {
    id: "listOrganizations",
    method: "GET",
    path: "/v1/organizations",
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
static OP_41: Operation = Operation {
    id: "listPolicyGroups",
    method: "GET",
    path: "/v1/policies/{policy_id}/groups",
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
static OP_42: Operation = Operation {
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
static OP_43: Operation = Operation {
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
static OP_44: Operation = Operation {
    id: "listPolicyUsers",
    method: "GET",
    path: "/v1/policies/{policy_id}/users",
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
static OP_45: Operation = Operation {
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
static OP_46: Operation = Operation {
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
static OP_47: Operation = Operation {
    id: "listUserGroups",
    method: "GET",
    path: "/v1/users/{user_id}/groups",
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
static OP_48: Operation = Operation {
    id: "listUserInlinePolicies",
    method: "GET",
    path: "/v1/users/{user_id}/inline-policies",
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
static OP_49: Operation = Operation {
    id: "listUserPolicies",
    method: "GET",
    path: "/v1/users/{user_id}/policies",
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
static OP_50: Operation = Operation {
    id: "listUsers",
    method: "GET",
    path: "/v1/users",
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
static OP_51: Operation = Operation {
    id: "putGroupInlinePolicy",
    method: "PUT",
    path: "/v1/groups/{group_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_52: Operation = Operation {
    id: "putUserInlinePolicy",
    method: "PUT",
    path: "/v1/users/{user_id}/inline-policies/{policy_name}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_53: Operation = Operation {
    id: "removeAccountRoleAssignment",
    method: "DELETE",
    path: "/v1/accounts/{account_id}/role-assignments/{assignment_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_54: Operation = Operation {
    id: "removeUser",
    method: "DELETE",
    path: "/v1/users/{user_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_55: Operation = Operation {
    id: "removeUserFromGroup",
    method: "DELETE",
    path: "/v1/users/{user_id}/groups/{group_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_56: Operation = Operation {
    id: "removeUserPermissionBoundary",
    method: "DELETE",
    path: "/v1/users/{user_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_57: Operation = Operation {
    id: "setUserPermissionBoundary",
    method: "PUT",
    path: "/v1/users/{user_id}/permission-boundary",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_58: Operation = Operation {
    id: "updateAccount",
    method: "PATCH",
    path: "/v1/accounts/{account_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_59: Operation = Operation {
    id: "updateGroup",
    method: "PATCH",
    path: "/v1/groups/{group_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_60: Operation = Operation {
    id: "updateOrganization",
    method: "PATCH",
    path: "/v1/organizations/{organization_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_61: Operation = Operation {
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
#[derive(Clone, Debug)]
pub struct WorkspaceService {
    pub(crate) client: Client,
}
impl WorkspaceService {
    /// Add user to organization
    pub fn add_user(&self, body: &m::AddUserBody) -> Request<m::AddUserResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_0,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Add user to group
    pub fn add_user_to_group(&self, user_id: &str, body: &m::AddUserToGroupBody) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_1,
            &[("user_id", user_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Assign account role
    pub fn assign_account_role(
        &self,
        account_id: &str,
        body: &m::AssignAccountRoleBody,
    ) -> Request<m::AssignAccountRoleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_2,
            &[("account_id", account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Attach policy to group
    pub fn attach_group_policy(
        &self,
        group_id: &str,
        body: &m::AttachGroupPolicyBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_3,
            &[("group_id", group_id)],
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
            "workspace",
            ENDPOINT,
            &OP_4,
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
            "workspace",
            ENDPOINT,
            &OP_5,
            &[("service_account_id", service_account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Attach policy to user
    pub fn attach_user_policy(
        &self,
        user_id: &str,
        body: &m::AttachUserPolicyBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_6,
            &[("user_id", user_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Cancel invitation
    pub fn cancel_invitation(&self, invitation_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_7,
            &[("invitation_id", invitation_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create account
    pub fn create_account(&self, body: &m::CreateAccountBody) -> Request<m::CreateAccountResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_8,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create group
    pub fn create_group(&self, body: &m::CreateGroupBody) -> Request<m::CreateGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_9,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create policy
    pub fn create_policy(&self, body: &m::CreatePolicyBody) -> Request<m::CreatePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_10,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete account
    pub fn delete_account(&self, account_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_11,
            &[("account_id", account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete group
    pub fn delete_group(&self, group_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_12,
            &[("group_id", group_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a group's inline policy by name
    pub fn delete_group_inline_policy(&self, group_id: &str, policy_name: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_13,
            &[("group_id", group_id), ("policy_name", policy_name)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete organization
    pub fn delete_organization(&self, organization_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_14,
            &[("organization_id", organization_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete policy
    pub fn delete_policy(&self, policy_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_15,
            &[("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a user's inline policy by name
    pub fn delete_user_inline_policy(&self, user_id: &str, policy_name: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_16,
            &[("user_id", user_id), ("policy_name", policy_name)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach policy from group
    pub fn detach_group_policy(&self, group_id: &str, policy_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_17,
            &[("group_id", group_id), ("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach policy from role
    pub fn detach_role_policy(&self, role_id: &str, policy_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_18,
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
            "workspace",
            ENDPOINT,
            &OP_19,
            &[
                ("service_account_id", service_account_id),
                ("policy_id", policy_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach policy from user
    pub fn detach_user_policy(&self, user_id: &str, policy_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_20,
            &[("user_id", user_id), ("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get account
    pub fn get_account(&self, account_id: &str) -> Request<m::GetAccountResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_21,
            &[("account_id", account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_account_by_reference(
        &self,
        reference: &str,
        scope: &m::GetAccountScope,
    ) -> Request<m::GetAccountResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_21,
                &[("account_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_33,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("account"),
            "accounts",
        );
        Request::reference(core, extract)
    }
    /// Check account resource presence
    pub fn get_account_resources(
        &self,
        account_id: &str,
    ) -> Request<m::GetAccountResourcesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_22,
            &[("account_id", account_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get group
    pub fn get_group(&self, group_id: &str) -> Request<m::GetGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_23,
            &[("group_id", group_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_group_by_reference(
        &self,
        reference: &str,
        scope: &m::GetGroupScope,
    ) -> Request<m::GetGroupResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_23,
                &[("group_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_37,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("group"),
            "groups",
        );
        Request::reference(core, extract)
    }
    /// Get a group's inline policy by name
    pub fn get_group_inline_policy(
        &self,
        group_id: &str,
        policy_name: &str,
    ) -> Request<m::GetGroupInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_24,
            &[("group_id", group_id), ("policy_name", policy_name)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_group_inline_policy_by_reference(
        &self,
        group_id: &str,
        reference: &str,
        scope: &m::GetGroupInlinePolicyScope,
    ) -> Request<m::GetGroupInlinePolicyResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_24,
                &[("group_id", group_id), ("policy_name", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_34,
                &[("group_id", group_id)],
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
    /// Get invitation
    pub fn get_invitation(&self, invitation_id: &str) -> Request<m::GetInvitationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_25,
            &[("invitation_id", invitation_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_invitation_by_reference(
        &self,
        reference: &str,
        scope: &m::GetInvitationScope,
    ) -> Request<m::GetInvitationResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_25,
                &[("invitation_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_38,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("invitation"),
            "invitations",
        );
        Request::reference(core, extract)
    }
    /// Get organization
    pub fn get_organization(&self, organization_id: &str) -> Request<m::GetOrganizationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_26,
            &[("organization_id", organization_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_organization_by_reference(
        &self,
        reference: &str,
        scope: &m::GetOrganizationScope,
    ) -> Request<m::GetOrganizationResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_26,
                &[("organization_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_39,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("organization"),
            "organizations",
        );
        Request::reference(core, extract)
    }
    /// Get policy
    pub fn get_policy(&self, policy_id: &str) -> Request<m::GetPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_27,
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
                "workspace",
                ENDPOINT,
                &OP_27,
                &[("policy_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_40,
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
    /// Get user
    pub fn get_user(&self, user_id: &str) -> Request<m::GetUserResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_28,
            &[("user_id", user_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_user_by_reference(
        &self,
        reference: &str,
        scope: &m::GetUserScope,
    ) -> Request<m::GetUserResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_28,
                &[("user_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_50,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("user"),
            "users",
        );
        Request::reference(core, extract)
    }
    /// Get a user's inline policy by name
    pub fn get_user_inline_policy(
        &self,
        user_id: &str,
        policy_name: &str,
    ) -> Request<m::GetUserInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_29,
            &[("user_id", user_id), ("policy_name", policy_name)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_user_inline_policy_by_reference(
        &self,
        user_id: &str,
        reference: &str,
        scope: &m::GetUserInlinePolicyScope,
    ) -> Request<m::GetUserInlinePolicyResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_29,
                &[("user_id", user_id), ("policy_name", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_48,
                &[("user_id", user_id)],
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
    /// Get a user's permission boundary
    pub fn get_user_permission_boundary(
        &self,
        user_id: &str,
    ) -> Request<m::GetUserPermissionBoundaryResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_30,
            &[("user_id", user_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List account role assignments
    pub fn list_account_role_assignments(
        &self,
        account_id: &str,
    ) -> PagedRequest<m::ListAccountRoleAssignmentsResponse, m::ListAccountRoleAssignmentsItem>
    {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_31,
                &[("account_id", account_id)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "role_assignments",
        )
    }
    /// List assigned account roles
    pub fn list_account_roles(
        &self,
    ) -> PagedRequest<m::ListAccountRolesResponse, m::ListAccountRolesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_32,
                &[],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "account_roles",
        )
    }
    /// List accounts
    pub fn list_accounts(
        &self,
        query: &m::ListAccountsQuery,
    ) -> PagedRequest<m::ListAccountsResponse, m::ListAccountsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_33,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "accounts",
        )
    }
    /// List a group's inline policies
    pub fn list_group_inline_policies(
        &self,
        group_id: &str,
        query: &m::ListGroupInlinePoliciesQuery,
    ) -> PagedRequest<m::ListGroupInlinePoliciesResponse, m::ListGroupInlinePoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_34,
                &[("group_id", group_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "inline_policies",
        )
    }
    /// List group policies
    pub fn list_group_policies(
        &self,
        group_id: &str,
        query: &m::ListGroupPoliciesQuery,
    ) -> PagedRequest<m::ListGroupPoliciesResponse, m::ListGroupPoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_35,
                &[("group_id", group_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
        )
    }
    /// List group users
    pub fn list_group_users(
        &self,
        group_id: &str,
        query: &m::ListGroupUsersQuery,
    ) -> PagedRequest<m::ListGroupUsersResponse, m::ListGroupUsersItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_36,
                &[("group_id", group_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "users",
        )
    }
    /// List groups
    pub fn list_groups(
        &self,
        query: &m::ListGroupsQuery,
    ) -> PagedRequest<m::ListGroupsResponse, m::ListGroupsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_37,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "groups",
        )
    }
    /// List invitations
    pub fn list_invitations(
        &self,
        query: &m::ListInvitationsQuery,
    ) -> PagedRequest<m::ListInvitationsResponse, m::ListInvitationsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_38,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "invitations",
        )
    }
    /// List organizations
    pub fn list_organizations(
        &self,
        query: &m::ListOrganizationsQuery,
    ) -> PagedRequest<m::ListOrganizationsResponse, m::ListOrganizationsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_39,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "organizations",
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
                "workspace",
                ENDPOINT,
                &OP_40,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
        )
    }
    /// List groups with policy
    pub fn list_policy_groups(
        &self,
        policy_id: &str,
        query: &m::ListPolicyGroupsQuery,
    ) -> PagedRequest<m::ListPolicyGroupsResponse, m::ListPolicyGroupsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_41,
                &[("policy_id", policy_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "groups",
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
                "workspace",
                ENDPOINT,
                &OP_42,
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
                "workspace",
                ENDPOINT,
                &OP_43,
                &[("policy_id", policy_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "service_accounts",
        )
    }
    /// List users with policy
    pub fn list_policy_users(
        &self,
        policy_id: &str,
        query: &m::ListPolicyUsersQuery,
    ) -> PagedRequest<m::ListPolicyUsersResponse, m::ListPolicyUsersItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_44,
                &[("policy_id", policy_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "users",
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
                "workspace",
                ENDPOINT,
                &OP_45,
                &[("role_id", role_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
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
                "workspace",
                ENDPOINT,
                &OP_46,
                &[("service_account_id", service_account_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
        )
    }
    /// List user groups
    pub fn list_user_groups(
        &self,
        user_id: &str,
        query: &m::ListUserGroupsQuery,
    ) -> PagedRequest<m::ListUserGroupsResponse, m::ListUserGroupsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_47,
                &[("user_id", user_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "groups",
        )
    }
    /// List a user's inline policies
    pub fn list_user_inline_policies(
        &self,
        user_id: &str,
        query: &m::ListUserInlinePoliciesQuery,
    ) -> PagedRequest<m::ListUserInlinePoliciesResponse, m::ListUserInlinePoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_48,
                &[("user_id", user_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "inline_policies",
        )
    }
    /// List user policies
    pub fn list_user_policies(
        &self,
        user_id: &str,
        query: &m::ListUserPoliciesQuery,
    ) -> PagedRequest<m::ListUserPoliciesResponse, m::ListUserPoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_49,
                &[("user_id", user_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "policies",
        )
    }
    /// List users
    pub fn list_users(
        &self,
        query: &m::ListUsersQuery,
    ) -> PagedRequest<m::ListUsersResponse, m::ListUsersItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "workspace",
                ENDPOINT,
                &OP_50,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "users",
        )
    }
    /// Create or replace a group's inline policy
    pub fn put_group_inline_policy(
        &self,
        group_id: &str,
        policy_name: &str,
        body: &m::PutGroupInlinePolicyBody,
    ) -> Request<m::PutGroupInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_51,
            &[("group_id", group_id), ("policy_name", policy_name)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create or replace a user's inline policy
    pub fn put_user_inline_policy(
        &self,
        user_id: &str,
        policy_name: &str,
        body: &m::PutUserInlinePolicyBody,
    ) -> Request<m::PutUserInlinePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_52,
            &[("user_id", user_id), ("policy_name", policy_name)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Remove account role assignment
    pub fn remove_account_role_assignment(
        &self,
        account_id: &str,
        assignment_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_53,
            &[("account_id", account_id), ("assignment_id", assignment_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Remove user from organization
    pub fn remove_user(&self, user_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_54,
            &[("user_id", user_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Remove user from group
    pub fn remove_user_from_group(&self, user_id: &str, group_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_55,
            &[("user_id", user_id), ("group_id", group_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Remove a user's permission boundary
    pub fn remove_user_permission_boundary(&self, user_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_56,
            &[("user_id", user_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set a user's permission boundary
    pub fn set_user_permission_boundary(
        &self,
        user_id: &str,
        body: &m::SetUserPermissionBoundaryBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_57,
            &[("user_id", user_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update account
    pub fn update_account(
        &self,
        account_id: &str,
        body: &m::UpdateAccountBody,
    ) -> Request<m::UpdateAccountResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_58,
            &[("account_id", account_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update group
    pub fn update_group(
        &self,
        group_id: &str,
        body: &m::UpdateGroupBody,
    ) -> Request<m::UpdateGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_59,
            &[("group_id", group_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update organization
    pub fn update_organization(
        &self,
        organization_id: &str,
        body: &m::UpdateOrganizationBody,
    ) -> Request<m::UpdateOrganizationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "workspace",
            ENDPOINT,
            &OP_60,
            &[("organization_id", organization_id)],
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
            "workspace",
            ENDPOINT,
            &OP_61,
            &[("policy_id", policy_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
