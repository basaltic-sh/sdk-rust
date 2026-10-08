//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::kms as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://kms.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "cancelKeyDeletion",
    method: "POST",
    path: "/v1/keys/{key_id}/cancel-deletion",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "createKey",
    method: "POST",
    path: "/v1/keys",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "decrypt",
    method: "POST",
    path: "/v1/keys/{key_id}/decrypt",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "disableKey",
    method: "POST",
    path: "/v1/keys/{key_id}/disable",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "enableKey",
    method: "POST",
    path: "/v1/keys/{key_id}/enable",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "encrypt",
    method: "POST",
    path: "/v1/keys/{key_id}/encrypt",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "generateDataKey",
    method: "POST",
    path: "/v1/keys/{key_id}/generate-data-key",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "getKey",
    method: "GET",
    path: "/v1/keys/{key_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "listKeys",
    method: "GET",
    path: "/v1/keys",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
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
        QueryEncoding {
            name: "state",
            style: "form",
            explode: true,
        },
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
static OP_9: Operation = Operation {
    id: "scheduleKeyDeletion",
    method: "POST",
    path: "/v1/keys/{key_id}/schedule-deletion",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "sign",
    method: "POST",
    path: "/v1/keys/{key_id}/sign",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "updateKey",
    method: "PATCH",
    path: "/v1/keys/{key_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "verify",
    method: "POST",
    path: "/v1/keys/{key_id}/verify",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct KmsService {
    pub(crate) client: Client,
}
impl KmsService {
    /// Cancel a scheduled deletion
    pub fn cancel_key_deletion(&self, key_id: &str) -> Request<m::CancelKeyDeletionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_0,
            &[("key_id", key_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a KMS key
    pub fn create_key(&self, body: &m::CreateKeyBody) -> Request<m::CreateKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_1,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Decrypt a ciphertext
    pub fn decrypt(&self, key_id: &str, body: &m::DecryptBody) -> Request<m::DecryptResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_2,
            &[("key_id", key_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Disable a key
    pub fn disable_key(&self, key_id: &str) -> Request<m::DisableKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_3,
            &[("key_id", key_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Enable a disabled key
    pub fn enable_key(&self, key_id: &str) -> Request<m::EnableKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_4,
            &[("key_id", key_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Encrypt a payload
    pub fn encrypt(&self, key_id: &str, body: &m::EncryptBody) -> Request<m::EncryptResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_5,
            &[("key_id", key_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Generate a fresh data key
    pub fn generate_data_key(
        &self,
        key_id: &str,
        body: Option<&m::GenerateDataKeyBody>,
    ) -> Request<m::GenerateDataKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_6,
            &[("key_id", key_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get a KMS key
    pub fn get_key(&self, key_id: &str) -> Request<m::GetKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_7,
            &[("key_id", key_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_key_by_reference(
        &self,
        reference: &str,
        scope: &m::GetKeyScope,
    ) -> Request<m::GetKeyResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "kms",
                ENDPOINT,
                &OP_7,
                &[("key_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "kms",
                ENDPOINT,
                &OP_8,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("key"),
            "keys",
        );
        Request::reference(core, extract)
    }
    /// List KMS keys
    pub fn list_keys(
        &self,
        query: &m::ListKeysQuery,
    ) -> PagedRequest<m::ListKeysResponse, m::ListKeysItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "kms",
                ENDPOINT,
                &OP_8,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "keys",
        )
    }
    /// Schedule key for deletion
    pub fn schedule_key_deletion(
        &self,
        key_id: &str,
        body: Option<&m::ScheduleKeyDeletionBody>,
    ) -> Request<m::ScheduleKeyDeletionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_9,
            &[("key_id", key_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Sign a message
    pub fn sign(&self, key_id: &str, body: &m::SignBody) -> Request<m::SignResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_10,
            &[("key_id", key_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update key metadata
    pub fn update_key(
        &self,
        key_id: &str,
        body: &m::UpdateKeyBody,
    ) -> Request<m::UpdateKeyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_11,
            &[("key_id", key_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Verify a signature
    pub fn verify(&self, key_id: &str, body: &m::VerifyBody) -> Request<m::VerifyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "kms",
            ENDPOINT,
            &OP_12,
            &[("key_id", key_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
