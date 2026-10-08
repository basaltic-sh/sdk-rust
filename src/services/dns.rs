//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::dns as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://dns.basaltic.sh";
static OP_0: Operation = Operation {
    id: "associateZoneVPC",
    method: "POST",
    path: "/v1/zones/{zone_id}/vpc-associations",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "createRecord",
    method: "POST",
    path: "/v1/zones/{zone_id}/records",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "createZone",
    method: "POST",
    path: "/v1/zones",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "deleteRecord",
    method: "DELETE",
    path: "/v1/zones/{zone_id}/records/{record_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "deleteZone",
    method: "DELETE",
    path: "/v1/zones/{zone_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "deleteZoneRecordImport",
    method: "DELETE",
    path: "/v1/zones/{zone_id}/record-import",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "dissociateZoneVPC",
    method: "DELETE",
    path: "/v1/zones/{zone_id}/vpc-associations/{vpc_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "exportZoneFile",
    method: "GET",
    path: "/v1/zones/{zone_id}/export",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "getRecord",
    method: "GET",
    path: "/v1/zones/{zone_id}/records/{record_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "getZone",
    method: "GET",
    path: "/v1/zones/{zone_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "getZoneRecordImport",
    method: "GET",
    path: "/v1/zones/{zone_id}/record-import",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "importZoneFile",
    method: "POST",
    path: "/v1/zones/{zone_id}/import",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "listRecords",
    method: "GET",
    path: "/v1/zones/{zone_id}/records",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "type",
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
        QueryEncoding {
            name: "include_managed",
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
static OP_13: Operation = Operation {
    id: "listZoneVPCAssociations",
    method: "GET",
    path: "/v1/zones/{zone_id}/vpc-associations",
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
static OP_14: Operation = Operation {
    id: "listZones",
    method: "GET",
    path: "/v1/zones",
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
static OP_15: Operation = Operation {
    id: "updateRecord",
    method: "PATCH",
    path: "/v1/zones/{zone_id}/records/{record_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "updateZone",
    method: "PATCH",
    path: "/v1/zones/{zone_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "verifyZoneOwnership",
    method: "POST",
    path: "/v1/zones/{zone_id}/verify-ownership",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct DnsService {
    pub(crate) client: Client,
}
impl DnsService {
    /// Associate a VPC with a private zone
    pub fn associate_zone_vpc(
        &self,
        zone_id: &str,
        body: &m::AssociateZoneVPCBody,
    ) -> Request<m::AssociateZoneVPCResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_0,
            &[("zone_id", zone_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create record
    pub fn create_record(
        &self,
        zone_id: &str,
        body: &m::CreateRecordBody,
    ) -> Request<m::CreateRecordResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_1,
            &[("zone_id", zone_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create zone
    pub fn create_zone(&self, body: &m::CreateZoneBody) -> Request<m::CreateZoneResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_2,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete record
    pub fn delete_record(&self, zone_id: &str, record_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_3,
            &[("zone_id", zone_id), ("record_id", record_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete zone
    pub fn delete_zone(&self, zone_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_4,
            &[("zone_id", zone_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Discard the record-import outcome
    pub fn delete_zone_record_import(&self, zone_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_5,
            &[("zone_id", zone_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Dissociate a VPC from a private zone
    pub fn dissociate_zone_vpc(&self, zone_id: &str, vpc_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_6,
            &[("zone_id", zone_id), ("vpc_id", vpc_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Export the zone as a zone file
    pub fn export_zone_file(&self, zone_id: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_7,
            &[("zone_id", zone_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get record
    pub fn get_record(&self, zone_id: &str, record_id: &str) -> Request<m::GetRecordResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_8,
            &[("zone_id", zone_id), ("record_id", record_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_record_by_reference(
        &self,
        zone_id: &str,
        reference: &str,
        scope: &m::GetRecordScope,
    ) -> Request<m::GetRecordResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "dns",
                ENDPOINT,
                &OP_8,
                &[("zone_id", zone_id), ("record_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "dns",
                ENDPOINT,
                &OP_12,
                &[("zone_id", zone_id)],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("record"),
            "records",
        );
        Request::reference(core, extract)
    }
    /// Get zone
    pub fn get_zone(&self, zone_id: &str) -> Request<m::GetZoneResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_9,
            &[("zone_id", zone_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_zone_by_reference(
        &self,
        reference: &str,
        scope: &m::GetZoneScope,
    ) -> Request<m::GetZoneResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "dns",
                ENDPOINT,
                &OP_9,
                &[("zone_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "dns",
                ENDPOINT,
                &OP_14,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("zone"),
            "zones",
        );
        Request::reference(core, extract)
    }
    /// Get the record-import outcome
    pub fn get_zone_record_import(&self, zone_id: &str) -> Request<m::GetZoneRecordImportResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_10,
            &[("zone_id", zone_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Import a zone file
    pub fn import_zone_file(
        &self,
        zone_id: &str,
        body: &m::ImportZoneFileBody,
    ) -> Request<m::ImportZoneFileResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_11,
            &[("zone_id", zone_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// List records
    pub fn list_records(
        &self,
        zone_id: &str,
        query: &m::ListRecordsQuery,
    ) -> PagedRequest<m::ListRecordsResponse, m::ListRecordsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "dns",
                ENDPOINT,
                &OP_12,
                &[("zone_id", zone_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "records",
        )
    }
    /// List VPC associations
    pub fn list_zone_vpc_associations(
        &self,
        zone_id: &str,
        query: &m::ListZoneVPCAssociationsQuery,
    ) -> PagedRequest<m::ListZoneVPCAssociationsResponse, m::ListZoneVPCAssociationsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "dns",
                ENDPOINT,
                &OP_13,
                &[("zone_id", zone_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "vpc_ids",
        )
    }
    /// List zones
    pub fn list_zones(
        &self,
        query: &m::ListZonesQuery,
    ) -> PagedRequest<m::ListZonesResponse, m::ListZonesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "dns",
                ENDPOINT,
                &OP_14,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "zones",
        )
    }
    /// Update record
    pub fn update_record(
        &self,
        zone_id: &str,
        record_id: &str,
        body: &m::UpdateRecordBody,
    ) -> Request<m::UpdateRecordResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_15,
            &[("zone_id", zone_id), ("record_id", record_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update zone
    pub fn update_zone(
        &self,
        zone_id: &str,
        body: &m::UpdateZoneBody,
    ) -> Request<m::UpdateZoneResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_16,
            &[("zone_id", zone_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Verify zone ownership
    pub fn verify_zone_ownership(&self, zone_id: &str) -> Request<m::VerifyZoneOwnershipResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "dns",
            ENDPOINT,
            &OP_17,
            &[("zone_id", zone_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
}
