//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::billing as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://billing.basaltic.sh";
static OP_0: Operation = Operation {
    id: "getBillingProfile",
    method: "GET",
    path: "/v1/profile",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "getCurrentUsage",
    method: "GET",
    path: "/v1/usage",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "getFiscalInvoiceXml",
    method: "GET",
    path: "/v1/fiscal-invoices/{document_id}/xml",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "getInvoice",
    method: "GET",
    path: "/v1/invoices/{invoice_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "getInvoicePdf",
    method: "GET",
    path: "/v1/invoices/{invoice_id}/pdf",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "listCredits",
    method: "GET",
    path: "/v1/credits",
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
    id: "listFiscalInvoices",
    method: "GET",
    path: "/v1/fiscal-invoices",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "invoice",
        style: "form",
        explode: true,
    }],
};
static OP_7: Operation = Operation {
    id: "listInvoices",
    method: "GET",
    path: "/v1/invoices",
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
static OP_8: Operation = Operation {
    id: "listPayments",
    method: "GET",
    path: "/v1/payments",
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
static OP_9: Operation = Operation {
    id: "listPrices",
    method: "GET",
    path: "/v1/prices",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "service",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "resource_type",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "sku",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "family",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "at",
            style: "form",
            explode: true,
        },
    ],
};
static OP_10: Operation = Operation {
    id: "listTransactions",
    method: "GET",
    path: "/v1/transactions",
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
static OP_11: Operation = Operation {
    id: "updateBillingProfile",
    method: "PUT",
    path: "/v1/profile",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct BillingService {
    pub(crate) client: Client,
}
impl BillingService {
    /// Read the organization billing profile
    pub fn get_billing_profile(&self) -> Request<m::GetBillingProfileResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "billing",
            ENDPOINT,
            &OP_0,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get month-to-date usage total
    pub fn get_current_usage(&self) -> Request<m::GetCurrentUsageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "billing",
            ENDPOINT,
            &OP_1,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Download issued NFS-e XML
    pub fn get_fiscal_invoice_xml(&self, document_id: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "billing",
            ENDPOINT,
            &OP_2,
            &[("document_id", document_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get an invoice with its line items
    pub fn get_invoice(&self, invoice_id: &str) -> Request<m::GetInvoiceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "billing",
            ENDPOINT,
            &OP_3,
            &[("invoice_id", invoice_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_invoice_by_reference(
        &self,
        reference: &str,
        scope: &m::GetInvoiceScope,
    ) -> Request<m::GetInvoiceResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_3,
                &[("invoice_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_7,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            false,
            None,
            "invoices",
        );
        Request::reference(core, extract)
    }
    /// Download an invoice as a PDF statement
    pub fn get_invoice_pdf(&self, invoice_id: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "billing",
            ENDPOINT,
            &OP_4,
            &[("invoice_id", invoice_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List credit grants
    pub fn list_credits(
        &self,
        query: &m::ListCreditsQuery,
    ) -> PagedRequest<m::ListCreditsResponse, m::ListCreditsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_5,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "credits",
        )
    }
    /// List fiscal invoice issuance and delivery status
    pub fn list_fiscal_invoices(
        &self,
        query: &m::ListFiscalInvoicesQuery,
    ) -> PagedRequest<m::ListFiscalInvoicesResponse, m::ListFiscalInvoicesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_6,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "fiscal_documents",
        )
    }
    /// List invoices
    pub fn list_invoices(
        &self,
        query: &m::ListInvoicesQuery,
    ) -> PagedRequest<m::ListInvoicesResponse, m::ListInvoicesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_7,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "invoices",
        )
    }
    /// List invoice payments
    pub fn list_payments(
        &self,
        query: &m::ListPaymentsQuery,
    ) -> PagedRequest<m::ListPaymentsResponse, m::ListPaymentsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_8,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "payments",
        )
    }
    /// List catalog prices
    pub fn list_prices(
        &self,
        query: &m::ListPricesQuery,
    ) -> PagedRequest<m::ListPricesResponse, m::ListPricesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_9,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "prices",
        )
    }
    /// List ledger transactions
    pub fn list_transactions(
        &self,
        query: &m::ListTransactionsQuery,
    ) -> PagedRequest<m::ListTransactionsResponse, m::ListTransactionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "billing",
                ENDPOINT,
                &OP_10,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "transactions",
        )
    }
    /// Save organization billing details
    pub fn update_billing_profile(
        &self,
        body: &m::UpdateBillingProfileBody,
    ) -> Request<m::UpdateBillingProfileResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "billing",
            ENDPOINT,
            &OP_11,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
