//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::quota as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://quota.basaltic.sh";
static OP_0: Operation = Operation {
    id: "listQuotas",
    method: "GET",
    path: "/v1/quotas",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "region",
        style: "form",
        explode: true,
    }],
};
#[derive(Clone, Debug)]
pub struct QuotaService {
    pub(crate) client: Client,
}
impl QuotaService {
    /// List quotas
    pub fn list_quotas(
        &self,
        query: &m::ListQuotasQuery,
    ) -> PagedRequest<m::ListQuotasResponse, m::ListQuotasItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "quota",
                ENDPOINT,
                &OP_0,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "quotas",
        )
    }
}
