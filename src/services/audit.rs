//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::audit as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://audit.basaltic.sh";
static OP_0: Operation = Operation {
    id: "getAuditLog",
    method: "GET",
    path: "/v1/audit-logs/{log_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "listAuditLogs",
    method: "GET",
    path: "/v1/audit-logs",
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
            name: "actor",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "actor_type",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "action",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "resource_type",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "resource",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "status",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "ip_address",
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
#[derive(Clone, Debug)]
pub struct AuditService {
    pub(crate) client: Client,
}
impl AuditService {
    /// Get audit log entry
    pub fn get_audit_log(&self, log_id: &str) -> Request<m::GetAuditLogResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "audit",
            ENDPOINT,
            &OP_0,
            &[("log_id", log_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_audit_log_by_reference(
        &self,
        reference: &str,
        scope: &m::GetAuditLogScope,
    ) -> Request<m::GetAuditLogResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "audit",
                ENDPOINT,
                &OP_0,
                &[("log_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "audit",
                ENDPOINT,
                &OP_1,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            false,
            Some("audit_log"),
            "audit_logs",
        );
        Request::reference(core, extract)
    }
    /// List audit logs
    pub fn list_audit_logs(
        &self,
        query: &m::ListAuditLogsQuery,
    ) -> PagedRequest<m::ListAuditLogsResponse, m::ListAuditLogsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "audit",
                ENDPOINT,
                &OP_1,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "audit_logs",
        )
    }
}
