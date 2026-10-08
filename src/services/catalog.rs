//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::catalog as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://catalog.basaltic.sh";
static OP_0: Operation = Operation {
    id: "getRegion",
    method: "GET",
    path: "/v1/regions/{code}",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
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
#[derive(Clone, Debug)]
pub struct CatalogService {
    pub(crate) client: Client,
}
impl CatalogService {
    /// Get a region
    pub fn get_region(&self, code: &str) -> Request<m::GetRegionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "catalog",
            ENDPOINT,
            &OP_0,
            &[("code", code)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_region_by_reference(
        &self,
        reference: &str,
        scope: &m::GetRegionScope,
    ) -> Request<m::GetRegionResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_0,
                &[("code", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_1,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            None,
            "regions",
        );
        Request::reference(core, extract)
    }
    /// List regions
    pub fn list_regions(
        &self,
        query: &m::ListRegionsQuery,
    ) -> PagedRequest<m::ListRegionsResponse, m::ListRegionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_1,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "regions",
        )
    }
}
