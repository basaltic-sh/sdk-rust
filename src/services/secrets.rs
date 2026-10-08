//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::secrets as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://secrets.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "createSecret",
    method: "POST",
    path: "/v1/secrets",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "deleteSecret",
    method: "DELETE",
    path: "/v1/secrets/{secret_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "describeSecret",
    method: "GET",
    path: "/v1/secrets/{secret_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "getSecretValue",
    method: "GET",
    path: "/v1/secrets/{secret_id}/value",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "version",
        style: "form",
        explode: true,
    }],
};
static OP_4: Operation = Operation {
    id: "listSecrets",
    method: "GET",
    path: "/v1/secrets",
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
            name: "include_deleted",
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
static OP_5: Operation = Operation {
    id: "listVersions",
    method: "GET",
    path: "/v1/secrets/{secret_id}/versions",
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
    ],
};
static OP_6: Operation = Operation {
    id: "putSecretValue",
    method: "POST",
    path: "/v1/secrets/{secret_id}/value",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "restoreSecret",
    method: "POST",
    path: "/v1/secrets/{secret_id}/restore",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "updateSecret",
    method: "PATCH",
    path: "/v1/secrets/{secret_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct SecretsService {
    pub(crate) client: Client,
}
impl SecretsService {
    /// Create a new secret with an initial value
    pub fn create_secret(&self, body: &m::CreateSecretBody) -> Request<m::CreateSecretResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "secrets",
            ENDPOINT,
            &OP_0,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Schedule deletion (soft delete with recovery window)
    pub fn delete_secret(
        &self,
        secret_id: &str,
        body: Option<&m::DeleteSecretBody>,
    ) -> Request<m::DeleteSecretResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "secrets",
            ENDPOINT,
            &OP_1,
            &[("secret_id", secret_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a secret (no value)
    pub fn describe_secret(&self, secret_id: &str) -> Request<m::DescribeSecretResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "secrets",
            ENDPOINT,
            &OP_2,
            &[("secret_id", secret_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Read the current value (or a specific version)
    pub fn get_secret_value(
        &self,
        secret_id: &str,
        query: &m::GetSecretValueQuery,
    ) -> Request<m::GetSecretValueResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "secrets",
            ENDPOINT,
            &OP_3,
            &[("secret_id", secret_id)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// List secrets
    pub fn list_secrets(
        &self,
        query: &m::ListSecretsQuery,
    ) -> PagedRequest<m::ListSecretsResponse, m::ListSecretsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "secrets",
                ENDPOINT,
                &OP_4,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "secrets",
        )
    }
    /// List versions
    pub fn list_versions(
        &self,
        secret_id: &str,
        query: &m::ListVersionsQuery,
    ) -> PagedRequest<m::ListVersionsResponse, m::ListVersionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "secrets",
                ENDPOINT,
                &OP_5,
                &[("secret_id", secret_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "versions",
        )
    }
    /// Store a new version (becomes current)
    pub fn put_secret_value(
        &self,
        secret_id: &str,
        body: &m::PutSecretValueBody,
    ) -> Request<m::PutSecretValueResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "secrets",
            ENDPOINT,
            &OP_6,
            &[("secret_id", secret_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Restore a secret from the recovery window
    pub fn restore_secret(&self, secret_id: &str) -> Request<m::RestoreSecretResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "secrets",
            ENDPOINT,
            &OP_7,
            &[("secret_id", secret_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update mutable metadata
    pub fn update_secret(
        &self,
        secret_id: &str,
        body: &m::UpdateSecretBody,
    ) -> Request<m::UpdateSecretResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "secrets",
            ENDPOINT,
            &OP_8,
            &[("secret_id", secret_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
