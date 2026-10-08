//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::certificate as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://certificate.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "createCertificate",
    method: "POST",
    path: "/v1/certificates",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "deleteCertificate",
    method: "DELETE",
    path: "/v1/certificates/{certificate_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "getCertificate",
    method: "GET",
    path: "/v1/certificates/{certificate_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "getCertificateMaterial",
    method: "GET",
    path: "/v1/certificates/{certificate_id}/material",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "listCertificates",
    method: "GET",
    path: "/v1/certificates",
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
static OP_5: Operation = Operation {
    id: "revokeCertificate",
    method: "POST",
    path: "/v1/certificates/{certificate_id}/revoke",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct CertificateService {
    pub(crate) client: Client,
}
impl CertificateService {
    /// Create certificate
    pub fn create_certificate(
        &self,
        body: &m::CreateCertificateBody,
    ) -> Request<m::CreateCertificateResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "certificate",
            ENDPOINT,
            &OP_0,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete certificate
    pub fn delete_certificate(&self, certificate_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "certificate",
            ENDPOINT,
            &OP_1,
            &[("certificate_id", certificate_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get certificate
    pub fn get_certificate(&self, certificate_id: &str) -> Request<m::GetCertificateResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "certificate",
            ENDPOINT,
            &OP_2,
            &[("certificate_id", certificate_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_certificate_by_reference(
        &self,
        reference: &str,
        scope: &m::GetCertificateScope,
    ) -> Request<m::GetCertificateResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "certificate",
                ENDPOINT,
                &OP_2,
                &[("certificate_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "certificate",
                ENDPOINT,
                &OP_4,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("certificate"),
            "certificates",
        );
        Request::reference(core, extract)
    }
    /// Fetch certificate material (leaf, chain, private key)
    pub fn get_certificate_material(
        &self,
        certificate_id: &str,
    ) -> Request<m::GetCertificateMaterialResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "certificate",
            ENDPOINT,
            &OP_3,
            &[("certificate_id", certificate_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List certificates
    pub fn list_certificates(
        &self,
        query: &m::ListCertificatesQuery,
    ) -> PagedRequest<m::ListCertificatesResponse, m::ListCertificatesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "certificate",
                ENDPOINT,
                &OP_4,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "certificates",
        )
    }
    /// Revoke certificate
    pub fn revoke_certificate(
        &self,
        certificate_id: &str,
    ) -> Request<m::RevokeCertificateResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "certificate",
            ENDPOINT,
            &OP_5,
            &[("certificate_id", certificate_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
}
