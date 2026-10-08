//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BillingProfile {
    #[serde(
        rename = "customer_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_type: Option<BillingProfileCustomerType>,
    /// Full legal name of the individual or company.
    #[serde(
        rename = "company_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub company_name: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// CPF for a Brazilian individual or CNPJ for a Brazilian company. Check digits are validated.
    #[serde(rename = "tax_id", default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Foreign identifier; not validated as a Brazilian document.
    #[serde(
        rename = "foreign_tax_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub foreign_tax_id: Option<String>,
    /// Required for a foreign recipient without a tax identifier.
    #[serde(
        rename = "no_tax_id_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_tax_id_reason: Option<String>,
    /// Billing email for fiscal invoice delivery. The onboarding form prefills this from the signed-in user's email.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "phone", default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(
        rename = "street_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_name: Option<String>,

    #[serde(
        rename = "street_number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_number: Option<String>,

    #[serde(
        rename = "complement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complement: Option<String>,

    #[serde(
        rename = "neighborhood",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub neighborhood: Option<String>,

    #[serde(rename = "city", default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Seven-digit IBGE municipality code, required for a Brazilian recipient.
    #[serde(
        rename = "municipality_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub municipality_code: Option<String>,
    /// Two-letter UF for Brazil; free-form state/province abroad.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Eight-digit CEP for Brazil; optional international postal code abroad.
    #[serde(
        rename = "postal_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postal_code: Option<String>,

    #[serde(rename = "ready", default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,

    #[serde(
        rename = "missing_fields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub missing_fields: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BillingProfileCustomerType {
    Value,
    Individual,
    Company,
    Unknown(String),
}
impl BillingProfileCustomerType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value => "",
            Self::Individual => "individual",
            Self::Company => "company",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for BillingProfileCustomerType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BillingProfileCustomerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "" => Self::Value,
            "individual" => Self::Individual,
            "company" => Self::Company,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetBillingProfileResponse = BillingProfile;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CurrentUsage {
    /// Unbilled usage accrued this UTC month, 2-decimal string.
    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "period_start")]
    pub period_start: String,
    /// Per-SKU breakdown, ordered by cost.
    #[serde(rename = "items")]
    pub items: Vec<UsageLine>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UsageLine {
    #[serde(rename = "sku")]
    pub sku: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "quantity")]
    pub quantity: String,

    #[serde(rename = "unit")]
    pub unit: String,
    /// Accrued cost at 4-decimal precision (sub-centavo lines stay visible mid-month).
    #[serde(rename = "amount")]
    pub amount: String,
}

pub type GetCurrentUsageResponse = CurrentUsage;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Invoice {
    /// Global organization-scoped invoice identity.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "invoice_number")]
    pub invoice_number: String,
    /// First day of the billed UTC month.
    #[serde(rename = "period_start")]
    pub period_start: String,
    /// Exclusive end (first day of the following month).
    #[serde(rename = "period_end")]
    pub period_end: String,

    #[serde(rename = "subtotal")]
    pub subtotal: String,

    #[serde(rename = "credits_applied")]
    pub credits_applied: String,

    #[serde(rename = "total")]
    pub total: String,

    #[serde(rename = "currency")]
    pub currency: String,
    /// Confirmed refunds less failed-refund reversals, in BRL.
    #[serde(rename = "refunded_amount")]
    pub refunded_amount: String,
    /// Dispute principal withdrawn less funds reinstated; excludes provider fees.
    #[serde(rename = "disputed_amount")]
    pub disputed_amount: String,

    #[serde(rename = "status")]
    pub status: InvoiceStatus,

    #[serde(
        rename = "issued_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub issued_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "due_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub due_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "paid_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub paid_at: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Path of the PDF statement, rendered on demand by GET /v1/invoices/{invoice_id}/pdf under the same authorization as this document.
    #[serde(rename = "pdf_url", default, skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,
    /// Line items; present only on the detail endpoint.
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<InvoiceItem>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvoiceStatus {
    Open,
    Paid,
    PastDue,
    Uncollectible,
    Void,
    Unknown(String),
}
impl InvoiceStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Open => "open",
            Self::Paid => "paid",
            Self::PastDue => "past_due",
            Self::Uncollectible => "uncollectible",
            Self::Void => "void",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InvoiceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "open" => Self::Open,
            "paid" => Self::Paid,
            "past_due" => Self::PastDue,
            "uncollectible" => Self::Uncollectible,
            "void" => Self::Void,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceItem {
    #[serde(rename = "kind")]
    pub kind: InvoiceItemKind,

    #[serde(
        rename = "sku",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub sku: Option<crate::Nullable<String>>,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "quantity")]
    pub quantity: String,

    #[serde(
        rename = "unit",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub unit: Option<crate::Nullable<String>>,

    #[serde(rename = "unit_price")]
    pub unit_price: String,
    /// Rounded line total; negative for credit lines.
    #[serde(rename = "amount")]
    pub amount: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvoiceItemKind {
    Usage,
    Credit,
    Unknown(String),
}
impl InvoiceItemKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Usage => "usage",
            Self::Credit => "credit",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InvoiceItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "usage" => Self::Usage,
            "credit" => Self::Credit,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetInvoiceResponse = Invoice;

pub type GetInvoiceResource = GetInvoiceResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInvoiceScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListCreditsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListCreditsQuery = ListCreditsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreditListResponse {
    #[serde(rename = "credits")]
    pub credits: Vec<Credit>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Credit {
    /// Global organization-scoped credit identity.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "source")]
    pub source: CreditSource,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "remaining")]
    pub remaining: String,

    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub expires_at: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreditSource {
    Promo,
    Coupon,
    Adjustment,
    Migration,
    Unknown(String),
}
impl CreditSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Promo => "promo",
            Self::Coupon => "coupon",
            Self::Adjustment => "adjustment",
            Self::Migration => "migration",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreditSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "promo" => Self::Promo,
            "coupon" => Self::Coupon,
            "adjustment" => Self::Adjustment,
            "migration" => Self::Migration,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PaginationMeta {
    /// Total number of items
    #[serde(rename = "total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Number of items per page
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Opaque cursor for the next page. Pass it back as the `marker` query parameter; treat it as a token, not a value to parse.
    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// Whether there are more items
    #[serde(rename = "has_more", default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

pub type ListCreditsResponse = CreditListResponse;

pub type ListCreditsItem = Credit;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListFiscalInvoicesParameters {
    #[serde(rename = "invoice", default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
}

pub type ListFiscalInvoicesQuery = ListFiscalInvoicesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListFiscalInvoicesResult {
    #[serde(rename = "fiscal_documents")]
    pub fiscal_documents: Vec<FiscalInvoice>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FiscalInvoice {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "organization_id")]
    pub organization_id: String,

    #[serde(rename = "payment_id")]
    pub payment_id: String,

    #[serde(
        rename = "invoice_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub invoice_id: Option<crate::Nullable<String>>,
    /// Actual amount received in BRL, not the billing invoice total.
    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(rename = "status")]
    pub status: FiscalInvoiceStatus,
    /// Separate delivery state. Queued means durably accepted by the internal email service; it does not assert recipient delivery.
    #[serde(rename = "email_status")]
    pub email_status: FiscalInvoiceEmailStatus,
    /// Sanitized operational error or municipal rejection codes.
    #[serde(
        rename = "last_error",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_error: Option<String>,

    #[serde(rename = "attempts")]
    pub attempts: i64,

    #[serde(rename = "number", default, skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    #[serde(
        rename = "verification_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verification_code: Option<String>,
    /// Municipal view/print link available after issuance.
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "issued_at", default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    /// Refunds preserve the fiscal document and require operator review; cancellation is never inferred automatically.
    #[serde(rename = "requires_review")]
    pub requires_review: bool,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FiscalInvoiceStatus {
    Queued,
    WaitingDetails,
    Retrying,
    Rejected,
    Issued,
    ReviewRequired,
    Unknown(String),
}
impl FiscalInvoiceStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Queued => "queued",
            Self::WaitingDetails => "waiting_details",
            Self::Retrying => "retrying",
            Self::Rejected => "rejected",
            Self::Issued => "issued",
            Self::ReviewRequired => "review_required",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FiscalInvoiceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FiscalInvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "queued" => Self::Queued,
            "waiting_details" => Self::WaitingDetails,
            "retrying" => Self::Retrying,
            "rejected" => Self::Rejected,
            "issued" => Self::Issued,
            "review_required" => Self::ReviewRequired,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FiscalInvoiceEmailStatus {
    Pending,
    Queued,
    Unknown(String),
}
impl FiscalInvoiceEmailStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Queued => "queued",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FiscalInvoiceEmailStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FiscalInvoiceEmailStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "queued" => Self::Queued,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListFiscalInvoicesResponse = ListFiscalInvoicesResult;

pub type ListFiscalInvoicesItem = FiscalInvoice;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInvoicesParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListInvoicesQuery = ListInvoicesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceListResponse {
    #[serde(rename = "invoices")]
    pub invoices: Vec<Invoice>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInvoicesResponse = InvoiceListResponse;

pub type ListInvoicesItem = Invoice;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPaymentsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListPaymentsQuery = ListPaymentsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentListResponse {
    #[serde(rename = "payments")]
    pub payments: Vec<Payment>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Payment {
    /// Global organization-scoped payment identity.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,
    /// Current invoice list shape, without items; null when the invoice has been deleted.
    #[serde(rename = "invoice")]
    pub invoice: PaymentInvoice,

    #[serde(rename = "amount")]
    pub amount: String,
    /// Confirmed refunds less failed-refund reversals, in BRL.
    #[serde(rename = "refunded_amount")]
    pub refunded_amount: String,
    /// Dispute principal withdrawn less funds reinstated; excludes provider fees.
    #[serde(rename = "disputed_amount")]
    pub disputed_amount: String,
    /// Settled receipt less refunds and disputed funds. Zero for an unsettled attempt; may be negative if the provider has withdrawn overlapping reversals. Does not change invoice collection status.
    #[serde(rename = "retained_amount")]
    pub retained_amount: String,

    #[serde(rename = "status")]
    pub status: PaymentStatus,
    /// 1-based dunning attempt this payment row belongs to.
    #[serde(rename = "attempt")]
    pub attempt: i64,

    #[serde(
        rename = "completed_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub completed_at: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PaymentInvoice {
    Variant1(Box<Invoice>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Succeeded,
    Failed,
    Refunded,
    Unknown(String),
}
impl PaymentStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Refunded => "refunded",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PaymentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "processing" => Self::Processing,
            "succeeded" => Self::Succeeded,
            "failed" => Self::Failed,
            "refunded" => Self::Refunded,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListPaymentsResponse = PaymentListResponse;

pub type ListPaymentsItem = Payment;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPricesParameters {
    #[serde(rename = "service", default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,

    #[serde(
        rename = "resource_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<String>,

    #[serde(rename = "sku", default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,

    #[serde(rename = "family", default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,

    #[serde(rename = "at", default, skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
}

pub type ListPricesQuery = ListPricesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PriceListResponse {
    #[serde(rename = "prices")]
    pub prices: Vec<Price>,
    /// The instant the catalog was read as of — the `at` that was asked for, or the server's clock when none was.
    #[serde(rename = "as_of")]
    pub as_of: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Price {
    /// Stable catalog key, `{service}.{resource_type}.{variant}`. This is the public identity of a price — the row id is not published.
    #[serde(rename = "sku")]
    pub sku: String,
    /// Which service bills this SKU.
    #[serde(rename = "service")]
    pub service: String,

    #[serde(rename = "resource_type")]
    pub resource_type: String,
    /// Display name. For compute SKUs this is the flavor name.
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,
    /// What one unit of `unit_price` buys.
    #[serde(rename = "unit")]
    pub unit: String,
    /// Price for one `unit`, as an exact decimal string.
    #[serde(rename = "unit_price")]
    pub unit_price: String,

    #[serde(rename = "currency")]
    pub currency: String,
    /// Extra facts about the SKU — `class`, `family`, `vcpus`, `memory_gb`, `storage_type`, … `family` separates the managed products (load balancer replicas, database cluster nodes) from the general compute flavors they share a `resource_type` with.
    #[serde(rename = "metadata")]
    pub metadata: std::collections::BTreeMap<String, serde_json::Value>,
}

pub type ListPricesResponse = PriceListResponse;

pub type ListPricesItem = Price;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListTransactionsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListTransactionsQuery = ListTransactionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TransactionListResponse {
    #[serde(rename = "transactions")]
    pub transactions: Vec<Transaction>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    /// Global organization-scoped transaction identity.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,
    /// Ledger entry type.
    #[serde(rename = "type")]
    pub type_: TransactionType,
    /// Always positive; the direction lives in the type.
    #[serde(rename = "amount")]
    pub amount: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,
    /// Organization-scoped reference to the ledger entry's target: crn:billing:::invoice/&lt;id&gt;, crn:billing:::payment/&lt;id&gt;, or crn:billing:::credit/&lt;id&gt; for a credit grant. Pass this CRN to the corresponding collection's crn filter within the authenticated organization. Null for manual entries, unsupported reference types, or missing references.
    #[serde(
        rename = "reference",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub reference: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Payment,
    Refund,
    RefundReversal,
    Dispute,
    DisputeReversal,
    Adjustment,
    CreditGrant,
    CreditApplied,
    Unknown(String),
}
impl TransactionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Payment => "payment",
            Self::Refund => "refund",
            Self::RefundReversal => "refund_reversal",
            Self::Dispute => "dispute",
            Self::DisputeReversal => "dispute_reversal",
            Self::Adjustment => "adjustment",
            Self::CreditGrant => "credit_grant",
            Self::CreditApplied => "credit_applied",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TransactionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransactionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "payment" => Self::Payment,
            "refund" => Self::Refund,
            "refund_reversal" => Self::RefundReversal,
            "dispute" => Self::Dispute,
            "dispute_reversal" => Self::DisputeReversal,
            "adjustment" => Self::Adjustment,
            "credit_grant" => Self::CreditGrant,
            "credit_applied" => Self::CreditApplied,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListTransactionsResponse = TransactionListResponse;

pub type ListTransactionsItem = Transaction;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BillingProfileInput {
    #[serde(
        rename = "customer_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub customer_type: Option<BillingProfileInputCustomerType>,
    /// Full legal name of the individual or company.
    #[serde(
        rename = "company_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub company_name: Option<String>,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(rename = "country", default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// CPF for a Brazilian individual or CNPJ for a Brazilian company. Check digits are validated.
    #[serde(rename = "tax_id", default, skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// Foreign identifier; not validated as a Brazilian document.
    #[serde(
        rename = "foreign_tax_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub foreign_tax_id: Option<String>,
    /// Required for a foreign recipient without a tax identifier.
    #[serde(
        rename = "no_tax_id_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub no_tax_id_reason: Option<String>,
    /// Billing email for fiscal invoice delivery. The onboarding form prefills this from the signed-in user's email.
    #[serde(rename = "email", default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "phone", default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(
        rename = "street_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_name: Option<String>,

    #[serde(
        rename = "street_number",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_number: Option<String>,

    #[serde(
        rename = "complement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complement: Option<String>,

    #[serde(
        rename = "neighborhood",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub neighborhood: Option<String>,

    #[serde(rename = "city", default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Seven-digit IBGE municipality code, required for a Brazilian recipient.
    #[serde(
        rename = "municipality_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub municipality_code: Option<String>,
    /// Two-letter UF for Brazil; free-form state/province abroad.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Eight-digit CEP for Brazil; optional international postal code abroad.
    #[serde(
        rename = "postal_code",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postal_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BillingProfileInputCustomerType {
    Value,
    Individual,
    Company,
    Unknown(String),
}
impl BillingProfileInputCustomerType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value => "",
            Self::Individual => "individual",
            Self::Company => "company",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for BillingProfileInputCustomerType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BillingProfileInputCustomerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "" => Self::Value,
            "individual" => Self::Individual,
            "company" => Self::Company,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateBillingProfileBody = BillingProfileInput;

pub type UpdateBillingProfileResponse = BillingProfile;
