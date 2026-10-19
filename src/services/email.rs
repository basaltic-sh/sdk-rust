//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::email as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://email.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "addSuppressionOverride",
    method: "POST",
    path: "/v1/configuration-sets/{configuration_set_id}/suppression-overrides",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "beginDKIMRotation",
    method: "POST",
    path: "/v1/identities/{identity_id}/dkim/rotate",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "cancelDKIMRotation",
    method: "DELETE",
    path: "/v1/identities/{identity_id}/dkim/rotate",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "createConfigurationSet",
    method: "POST",
    path: "/v1/configuration-sets",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "createIPPool",
    method: "POST",
    path: "/v1/ip-pools",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "createIdentity",
    method: "POST",
    path: "/v1/identities",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "createRecipientRule",
    method: "POST",
    path: "/v1/identities/{identity_id}/recipient-rules",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "createSuppression",
    method: "POST",
    path: "/v1/suppressions",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "createTemplate",
    method: "POST",
    path: "/v1/templates",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "deleteConfigurationSet",
    method: "DELETE",
    path: "/v1/configuration-sets/{configuration_set_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "deleteIPPool",
    method: "DELETE",
    path: "/v1/ip-pools/{ip_pool_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "deleteIdentity",
    method: "DELETE",
    path: "/v1/identities/{identity_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "deleteRecipientRule",
    method: "DELETE",
    path: "/v1/identities/{identity_id}/recipient-rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "deleteSuppression",
    method: "DELETE",
    path: "/v1/suppressions/{suppression_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "deleteTemplate",
    method: "DELETE",
    path: "/v1/templates/{template_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "describeConfigurationSet",
    method: "GET",
    path: "/v1/configuration-sets/{configuration_set_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "describeDedicatedIP",
    method: "GET",
    path: "/v1/dedicated-ips/{dedicated_ip_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "describeIPPool",
    method: "GET",
    path: "/v1/ip-pools/{ip_pool_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "describeIdentity",
    method: "GET",
    path: "/v1/identities/{identity_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_19: Operation = Operation {
    id: "describeReceivedMessage",
    method: "GET",
    path: "/v1/received-messages/{message_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_20: Operation = Operation {
    id: "describeRecipientRule",
    method: "GET",
    path: "/v1/identities/{identity_id}/recipient-rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_21: Operation = Operation {
    id: "describeSentMessage",
    method: "GET",
    path: "/v1/sent-messages/{message_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_22: Operation = Operation {
    id: "describeSuppression",
    method: "GET",
    path: "/v1/suppressions/{suppression_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_23: Operation = Operation {
    id: "describeTemplate",
    method: "GET",
    path: "/v1/templates/{template_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_24: Operation = Operation {
    id: "downloadReceivedAttachment",
    method: "GET",
    path: "/v1/received-messages/{message_id}/attachments/{attachment_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_25: Operation = Operation {
    id: "downloadReceivedMessage",
    method: "GET",
    path: "/v1/received-messages/{message_id}/raw",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_26: Operation = Operation {
    id: "downloadSentAttachment",
    method: "GET",
    path: "/v1/sent-messages/{message_id}/attachments/{attachment_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_27: Operation = Operation {
    id: "downloadSentMessage",
    method: "GET",
    path: "/v1/sent-messages/{message_id}/raw",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_28: Operation = Operation {
    id: "finalizeDKIMRotation",
    method: "POST",
    path: "/v1/identities/{identity_id}/dkim/rotate/finalize",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_29: Operation = Operation {
    id: "getReceivedMessageContent",
    method: "GET",
    path: "/v1/received-messages/{message_id}/content",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_30: Operation = Operation {
    id: "getReceivingConfiguration",
    method: "GET",
    path: "/v1/identities/{identity_id}/receiving",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_31: Operation = Operation {
    id: "getSentMessageContent",
    method: "GET",
    path: "/v1/sent-messages/{message_id}/content",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_32: Operation = Operation {
    id: "getSentMessageEngagement",
    method: "GET",
    path: "/v1/sent-messages/{message_id}/engagement",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_33: Operation = Operation {
    id: "listConfigurationSets",
    method: "GET",
    path: "/v1/configuration-sets",
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
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_34: Operation = Operation {
    id: "listDMARCReports",
    method: "GET",
    path: "/v1/identities/{identity_id}/dmarc-reports",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "from",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "to",
            style: "form",
            explode: true,
        },
    ],
};
static OP_35: Operation = Operation {
    id: "listDedicatedIPs",
    method: "GET",
    path: "/v1/dedicated-ips",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "ip_pool",
            style: "form",
            explode: true,
        },
    ],
};
static OP_36: Operation = Operation {
    id: "listIPPools",
    method: "GET",
    path: "/v1/ip-pools",
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
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_37: Operation = Operation {
    id: "listIdentities",
    method: "GET",
    path: "/v1/identities",
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
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_38: Operation = Operation {
    id: "listReceivedMessages",
    method: "GET",
    path: "/v1/received-messages",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "identity",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "recipient",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "review_reason",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_39: Operation = Operation {
    id: "listRecipientRules",
    method: "GET",
    path: "/v1/identities/{identity_id}/recipient-rules",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_40: Operation = Operation {
    id: "listRejections",
    method: "GET",
    path: "/v1/rejections",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "before",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "identity",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "reason",
            style: "form",
            explode: true,
        },
    ],
};
static OP_41: Operation = Operation {
    id: "listSentMessages",
    method: "GET",
    path: "/v1/sent-messages",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "identity",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "status",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "recipient",
            style: "form",
            explode: true,
        },
    ],
};
static OP_42: Operation = Operation {
    id: "listSuppressions",
    method: "GET",
    path: "/v1/suppressions",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "reason",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_43: Operation = Operation {
    id: "listTemplates",
    method: "GET",
    path: "/v1/templates",
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
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_44: Operation = Operation {
    id: "releaseDedicatedIP",
    method: "DELETE",
    path: "/v1/dedicated-ips/{dedicated_ip_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_45: Operation = Operation {
    id: "removeSuppressionOverride",
    method: "DELETE",
    path: "/v1/configuration-sets/{configuration_set_id}/suppression-overrides/{address}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_46: Operation = Operation {
    id: "requestDedicatedIP",
    method: "POST",
    path: "/v1/dedicated-ips",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_47: Operation = Operation {
    id: "sendMessage",
    method: "POST",
    path: "/v1/sent-messages",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_48: Operation = Operation {
    id: "sendMessageBatch",
    method: "POST",
    path: "/v1/sent-messages/batch",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_49: Operation = Operation {
    id: "setBIMI",
    method: "PATCH",
    path: "/v1/identities/{identity_id}/bimi",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_50: Operation = Operation {
    id: "setMailFromDomain",
    method: "PATCH",
    path: "/v1/identities/{identity_id}/mail-from-domain",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_51: Operation = Operation {
    id: "updateConfigurationSet",
    method: "PATCH",
    path: "/v1/configuration-sets/{configuration_set_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_52: Operation = Operation {
    id: "updateDedicatedIP",
    method: "PATCH",
    path: "/v1/dedicated-ips/{dedicated_ip_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_53: Operation = Operation {
    id: "updateIPPool",
    method: "PATCH",
    path: "/v1/ip-pools/{ip_pool_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_54: Operation = Operation {
    id: "updateIdentity",
    method: "PATCH",
    path: "/v1/identities/{identity_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_55: Operation = Operation {
    id: "updateReceivingConfiguration",
    method: "PATCH",
    path: "/v1/identities/{identity_id}/receiving",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_56: Operation = Operation {
    id: "updateRecipientRule",
    method: "PATCH",
    path: "/v1/identities/{identity_id}/recipient-rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_57: Operation = Operation {
    id: "updateSuppression",
    method: "PATCH",
    path: "/v1/suppressions/{suppression_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_58: Operation = Operation {
    id: "updateTemplate",
    method: "PATCH",
    path: "/v1/templates/{template_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_59: Operation = Operation {
    id: "verifyBIMI",
    method: "POST",
    path: "/v1/identities/{identity_id}/bimi/verify",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_60: Operation = Operation {
    id: "verifyIdentity",
    method: "POST",
    path: "/v1/identities/{identity_id}/verify",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_61: Operation = Operation {
    id: "verifyMailFromDomain",
    method: "POST",
    path: "/v1/identities/{identity_id}/mail-from-domain/verify",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct EmailService {
    pub(crate) client: Client,
}
impl EmailService {
    /// Add a suppression override address
    pub fn add_suppression_override(
        &self,
        configuration_set_id: &str,
        body: &m::AddSuppressionOverrideBody,
    ) -> Request<m::AddSuppressionOverrideResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_0,
            &[("configuration_set_id", configuration_set_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Begin DKIM key rotation
    pub fn begin_dkim_rotation(&self, identity_id: &str) -> Request<m::BeginDKIMRotationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_1,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Cancel in-progress DKIM rotation
    pub fn cancel_dkim_rotation(
        &self,
        identity_id: &str,
    ) -> Request<m::CancelDKIMRotationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_2,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a configuration set
    pub fn create_configuration_set(
        &self,
        body: &m::CreateConfigurationSetBody,
    ) -> Request<m::CreateConfigurationSetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_3,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create an IP pool
    pub fn create_ip_pool(&self, body: &m::CreateIPPoolBody) -> Request<m::CreateIPPoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_4,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a sender identity
    pub fn create_identity(
        &self,
        body: &m::CreateIdentityBody,
    ) -> Request<m::CreateIdentityResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_5,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create recipient rule
    pub fn create_recipient_rule(
        &self,
        identity_id: &str,
        body: &m::CreateRecipientRuleBody,
    ) -> Request<m::CreateRecipientRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_6,
            &[("identity_id", identity_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Add a suppression entry
    pub fn create_suppression(
        &self,
        body: &m::CreateSuppressionBody,
    ) -> Request<m::CreateSuppressionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_7,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a template
    pub fn create_template(
        &self,
        body: &m::CreateTemplateBody,
    ) -> Request<m::CreateTemplateResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_8,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a configuration set
    pub fn delete_configuration_set(&self, configuration_set_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_9,
            &[("configuration_set_id", configuration_set_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete an IP pool
    pub fn delete_ip_pool(&self, ip_pool_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_10,
            &[("ip_pool_id", ip_pool_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete an identity
    pub fn delete_identity(&self, identity_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_11,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete recipient rule
    pub fn delete_recipient_rule(&self, identity_id: &str, rule_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_12,
            &[("identity_id", identity_id), ("rule_id", rule_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete suppression
    pub fn delete_suppression(&self, suppression_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_13,
            &[("suppression_id", suppression_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a template
    pub fn delete_template(&self, template_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_14,
            &[("template_id", template_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a configuration set
    pub fn describe_configuration_set(
        &self,
        configuration_set_id: &str,
    ) -> Request<m::DescribeConfigurationSetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_15,
            &[("configuration_set_id", configuration_set_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a dedicated IP
    pub fn describe_dedicated_ip(
        &self,
        dedicated_ip_id: &str,
    ) -> Request<m::DescribeDedicatedIPResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_16,
            &[("dedicated_ip_id", dedicated_ip_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe an IP pool
    pub fn describe_ip_pool(&self, ip_pool_id: &str) -> Request<m::DescribeIPPoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_17,
            &[("ip_pool_id", ip_pool_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe an identity
    pub fn describe_identity(&self, identity_id: &str) -> Request<m::DescribeIdentityResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_18,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get received message metadata
    pub fn describe_received_message(
        &self,
        message_id: &str,
    ) -> Request<m::DescribeReceivedMessageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_19,
            &[("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get recipient rule
    pub fn describe_recipient_rule(
        &self,
        identity_id: &str,
        rule_id: &str,
    ) -> Request<m::DescribeRecipientRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_20,
            &[("identity_id", identity_id), ("rule_id", rule_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a sent message
    pub fn describe_sent_message(
        &self,
        message_id: &str,
    ) -> Request<m::DescribeSentMessageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_21,
            &[("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get suppression
    pub fn describe_suppression(
        &self,
        suppression_id: &str,
    ) -> Request<m::DescribeSuppressionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_22,
            &[("suppression_id", suppression_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a template
    pub fn describe_template(&self, template_id: &str) -> Request<m::DescribeTemplateResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_23,
            &[("template_id", template_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Download received attachment
    pub fn download_received_attachment(
        &self,
        message_id: &str,
        attachment_id: &str,
    ) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_24,
            &[("message_id", message_id), ("attachment_id", attachment_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Download received message
    pub fn download_received_message(&self, message_id: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_25,
            &[("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Download sent attachment
    pub fn download_sent_attachment(&self, message_id: &str, attachment_id: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_26,
            &[("message_id", message_id), ("attachment_id", attachment_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Download sent message
    pub fn download_sent_message(&self, message_id: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_27,
            &[("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Finalize DKIM key rotation
    pub fn finalize_dkim_rotation(
        &self,
        identity_id: &str,
    ) -> Request<m::FinalizeDKIMRotationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_28,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get received message content
    pub fn get_received_message_content(
        &self,
        message_id: &str,
    ) -> Request<m::GetReceivedMessageContentResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_29,
            &[("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get receiving configuration
    pub fn get_receiving_configuration(
        &self,
        identity_id: &str,
    ) -> Request<m::GetReceivingConfigurationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_30,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get sent message content
    pub fn get_sent_message_content(
        &self,
        message_id: &str,
    ) -> Request<m::GetSentMessageContentResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_31,
            &[("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Sent message engagement
    pub fn get_sent_message_engagement(
        &self,
        message_id: &str,
    ) -> Request<m::GetSentMessageEngagementResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_32,
            &[("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List configuration sets
    pub fn list_configuration_sets(
        &self,
        query: &m::ListConfigurationSetsQuery,
    ) -> PagedRequest<m::ListConfigurationSetsResponse, m::ListConfigurationSetsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_33,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "configuration_sets",
        )
    }
    /// List DMARC report records
    pub fn list_dmarc_reports(
        &self,
        identity_id: &str,
        query: &m::ListDMARCReportsQuery,
    ) -> PagedRequest<m::ListDMARCReportsResponse, m::ListDMARCReportsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_34,
                &[("identity_id", identity_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "records",
        )
    }
    /// List dedicated IPs
    pub fn list_dedicated_i_ps(
        &self,
        query: &m::ListDedicatedIPsQuery,
    ) -> PagedRequest<m::ListDedicatedIPsResponse, m::ListDedicatedIPsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_35,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "dedicated_ips",
        )
    }
    /// List IP pools
    pub fn list_ip_pools(
        &self,
        query: &m::ListIPPoolsQuery,
    ) -> PagedRequest<m::ListIPPoolsResponse, m::ListIPPoolsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_36,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "ip_pools",
        )
    }
    /// List sender identities
    pub fn list_identities(
        &self,
        query: &m::ListIdentitiesQuery,
    ) -> PagedRequest<m::ListIdentitiesResponse, m::ListIdentitiesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_37,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "identities",
        )
    }
    /// List received messages
    pub fn list_received_messages(
        &self,
        query: &m::ListReceivedMessagesQuery,
    ) -> PagedRequest<m::ListReceivedMessagesResponse, m::ListReceivedMessagesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_38,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "messages",
        )
    }
    /// List recipient rules
    pub fn list_recipient_rules(
        &self,
        identity_id: &str,
        query: &m::ListRecipientRulesQuery,
    ) -> PagedRequest<m::ListRecipientRulesResponse, m::ListRecipientRulesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_39,
                &[("identity_id", identity_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "rules",
        )
    }
    /// List API-side send rejections
    pub fn list_rejections(
        &self,
        query: &m::ListRejectionsQuery,
    ) -> PagedRequest<m::ListRejectionsResponse, m::ListRejectionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_40,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "rejections",
        )
    }
    /// List sent messages
    pub fn list_sent_messages(
        &self,
        query: &m::ListSentMessagesQuery,
    ) -> PagedRequest<m::ListSentMessagesResponse, m::ListSentMessagesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_41,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "messages",
        )
    }
    /// List suppression entries
    pub fn list_suppressions(
        &self,
        query: &m::ListSuppressionsQuery,
    ) -> PagedRequest<m::ListSuppressionsResponse, m::ListSuppressionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_42,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "suppressions",
        )
    }
    /// List templates
    pub fn list_templates(
        &self,
        query: &m::ListTemplatesQuery,
    ) -> PagedRequest<m::ListTemplatesResponse, m::ListTemplatesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "email",
                ENDPOINT,
                &OP_43,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "templates",
        )
    }
    /// Release a dedicated IP
    pub fn release_dedicated_ip(&self, dedicated_ip_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_44,
            &[("dedicated_ip_id", dedicated_ip_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Remove a suppression override address
    pub fn remove_suppression_override(
        &self,
        configuration_set_id: &str,
        address: &str,
    ) -> Request<m::RemoveSuppressionOverrideResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_45,
            &[
                ("configuration_set_id", configuration_set_id),
                ("address", address),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Request a dedicated IP
    pub fn request_dedicated_ip(
        &self,
        body: Option<&m::RequestDedicatedIPBody>,
    ) -> Request<m::RequestDedicatedIPResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_46,
            &[],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Send a message
    pub fn send_message(&self, body: &m::SendMessageBody) -> Request<m::SendMessageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_47,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Send message batch
    pub fn send_message_batch(
        &self,
        body: &m::SendMessageBatchBody,
    ) -> Request<m::SendMessageBatchResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_48,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set BIMI logo / VMC URLs
    pub fn set_bimi(
        &self,
        identity_id: &str,
        body: &m::SetBIMIBody,
    ) -> Request<m::SetBIMIResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_49,
            &[("identity_id", identity_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set custom MAIL FROM domain
    pub fn set_mail_from_domain(
        &self,
        identity_id: &str,
        body: &m::SetMailFromDomainBody,
    ) -> Request<m::SetMailFromDomainResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_50,
            &[("identity_id", identity_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update a configuration set
    pub fn update_configuration_set(
        &self,
        configuration_set_id: &str,
        body: &m::UpdateConfigurationSetBody,
    ) -> Request<m::UpdateConfigurationSetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_51,
            &[("configuration_set_id", configuration_set_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update a dedicated IP
    pub fn update_dedicated_ip(
        &self,
        dedicated_ip_id: &str,
        body: &m::UpdateDedicatedIPBody,
    ) -> Request<m::UpdateDedicatedIPResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_52,
            &[("dedicated_ip_id", dedicated_ip_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update an IP pool
    pub fn update_ip_pool(
        &self,
        ip_pool_id: &str,
        body: &m::UpdateIPPoolBody,
    ) -> Request<m::UpdateIPPoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_53,
            &[("ip_pool_id", ip_pool_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update identity
    pub fn update_identity(
        &self,
        identity_id: &str,
        body: &m::UpdateIdentityBody,
    ) -> Request<m::UpdateIdentityResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_54,
            &[("identity_id", identity_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update receiving configuration
    pub fn update_receiving_configuration(
        &self,
        identity_id: &str,
        body: &m::UpdateReceivingConfigurationBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_55,
            &[("identity_id", identity_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update recipient rule
    pub fn update_recipient_rule(
        &self,
        identity_id: &str,
        rule_id: &str,
        body: &m::UpdateRecipientRuleBody,
    ) -> Request<m::UpdateRecipientRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_56,
            &[("identity_id", identity_id), ("rule_id", rule_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update suppression
    pub fn update_suppression(
        &self,
        suppression_id: &str,
        body: &m::UpdateSuppressionBody,
    ) -> Request<m::UpdateSuppressionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_57,
            &[("suppression_id", suppression_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update a template
    pub fn update_template(
        &self,
        template_id: &str,
        body: &m::UpdateTemplateBody,
    ) -> Request<m::UpdateTemplateResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_58,
            &[("template_id", template_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Verify BIMI logo (+ optional VMC)
    pub fn verify_bimi(&self, identity_id: &str) -> Request<m::VerifyBIMIResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_59,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Trigger DNS verification
    pub fn verify_identity(&self, identity_id: &str) -> Request<m::VerifyIdentityResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_60,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Verify custom MAIL FROM domain MX
    pub fn verify_mail_from_domain(
        &self,
        identity_id: &str,
    ) -> Request<m::VerifyMailFromDomainResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "email",
            ENDPOINT,
            &OP_61,
            &[("identity_id", identity_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
}
