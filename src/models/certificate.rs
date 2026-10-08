//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CertificateIssueRequestInput {
    /// Unique per account. Surfaces in the CRN (`crn:certificate::<account>:certificate/<name>`), so it must be URL-safe — letters, digits, dot, dash, underscore. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// Capped at 100 to stay inside the certificate authority's per-order limits.
    #[serde(rename = "domains")]
    pub domains: Vec<String>,

    #[serde(
        rename = "key_algorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_algorithm: Option<CertificateKeyAlgorithmInput>,
    /// Defaults to "acme" — issued by the platform CA. Set to "uploaded" to store customer-supplied PEM material instead; certificate_pem + private_key_pem must then be provided.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<CertificateSourceInput>,
    /// PEM-encoded leaf certificate. Required when source=uploaded.
    #[serde(
        rename = "certificate_pem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_pem: Option<String>,
    /// PEM-encoded intermediate chain (optional when source=uploaded).
    #[serde(rename = "chain_pem", default, skip_serializing_if = "Option::is_none")]
    pub chain_pem: Option<String>,
    /// PEM-encoded private key. Required when source=uploaded.
    #[serde(
        rename = "private_key_pem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_key_pem: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl std::fmt::Debug for CertificateIssueRequestInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("CertificateIssueRequestInput");
        d.field("name", &self.name);
        d.field("domains", &self.domains);
        d.field("key_algorithm", &self.key_algorithm);
        d.field("source", &self.source);
        d.field("certificate_pem", &self.certificate_pem);
        d.field("chain_pem", &self.chain_pem);
        d.field("private_key_pem", &"[REDACTED]");
        d.field("tags", &self.tags);
        d.finish()
    }
}
impl CertificateIssueRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, domains: Vec<String>) -> Self {
        Self {
            name,
            domains,
            key_algorithm: None,
            source: None,
            certificate_pem: None,
            chain_pem: None,
            private_key_pem: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CertificateKeyAlgorithmInput {
    EcdsaP256,
    EcdsaP384,
    Rsa2048,
    Rsa4096,
    Unknown(String),
}
impl CertificateKeyAlgorithmInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EcdsaP256 => "ecdsa-p256",
            Self::EcdsaP384 => "ecdsa-p384",
            Self::Rsa2048 => "rsa-2048",
            Self::Rsa4096 => "rsa-4096",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CertificateKeyAlgorithmInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CertificateKeyAlgorithmInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ecdsa-p256" => Self::EcdsaP256,
            "ecdsa-p384" => Self::EcdsaP384,
            "rsa-2048" => Self::Rsa2048,
            "rsa-4096" => Self::Rsa4096,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CertificateSourceInput {
    Acme,
    Uploaded,
    Unknown(String),
}
impl CertificateSourceInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Acme => "acme",
            Self::Uploaded => "uploaded",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CertificateSourceInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CertificateSourceInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "acme" => Self::Acme,
            "uploaded" => Self::Uploaded,
            _ => Self::Unknown(value),
        })
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateCertificateBody = CertificateIssueRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CertificateResponse {
    #[serde(
        rename = "certificate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate: Option<Certificate>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Certificate {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name-based, so an IAM policy can wildcard a naming convention (`crn:certificate::my-account:certificate/prod-*`). The region slot is empty for compatibility; certificate storage and KMS material are regional.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "domains", default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CertificateStatus>,

    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<CertificateSource>,

    #[serde(
        rename = "key_algorithm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_algorithm: Option<CertificateKeyAlgorithm>,
    /// Per-domain CNAME delegation state — one entry per domain on the cert. While status=pending_dns issuance waits for every challenge's `verified` to flip true; use `expected_cname` and `our_dns` to tell which records need to be added at the registrar.
    #[serde(
        rename = "challenges",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub challenges: Option<Vec<CertificateChallenge>>,
    /// PEM-encoded leaf certificate. Empty until active.
    #[serde(
        rename = "certificate_pem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_pem: Option<String>,
    /// PEM-encoded intermediate chain.
    #[serde(rename = "chain_pem", default, skip_serializing_if = "Option::is_none")]
    pub chain_pem: Option<String>,
    /// Hex SHA-256 of the leaf's DER — the certificate's material version. Changes on every rotation; consumers use it to know when to re-fetch material and to verify they fetched the intended generation. Empty until the cert has a leaf.
    #[serde(
        rename = "fingerprint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fingerprint: Option<String>,

    #[serde(rename = "issued_at", default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,

    #[serde(
        rename = "expires_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expires_at: Option<String>,
    /// Active faults, ordered newest first. Empty when healthy. Renewal failures are warnings while valid certificate material still serves. Codes: CERTIFICATE_ISSUANCE_START_FAILED (issuance could not be scheduled), CERTIFICATE_ISSUANCE_FAILED (the signing request failed), CERTIFICATE_RENEWAL_FAILED (renewal did not complete; a warning while valid material is still serving, an error once it has expired), CERTIFICATE_REVOCATION_FAILED (revocation did not complete).
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CertificateStatus {
    PendingDns,
    Pending,
    Active,
    Error,
    Expired,
    Revoked,
    Unknown(String),
}
impl CertificateStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PendingDns => "pending_dns",
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Error => "error",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CertificateStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CertificateStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending_dns" => Self::PendingDns,
            "pending" => Self::Pending,
            "active" => Self::Active,
            "error" => Self::Error,
            "expired" => Self::Expired,
            "revoked" => Self::Revoked,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CertificateSource {
    Acme,
    Uploaded,
    Unknown(String),
}
impl CertificateSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Acme => "acme",
            Self::Uploaded => "uploaded",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CertificateSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CertificateSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "acme" => Self::Acme,
            "uploaded" => Self::Uploaded,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CertificateKeyAlgorithm {
    EcdsaP256,
    EcdsaP384,
    Rsa2048,
    Rsa4096,
    Unknown(String),
}
impl CertificateKeyAlgorithm {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EcdsaP256 => "ecdsa-p256",
            Self::EcdsaP384 => "ecdsa-p384",
            Self::Rsa2048 => "rsa-2048",
            Self::Rsa4096 => "rsa-4096",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CertificateKeyAlgorithm {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CertificateKeyAlgorithm {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ecdsa-p256" => Self::EcdsaP256,
            "ecdsa-p384" => Self::EcdsaP384,
            "rsa-2048" => Self::Rsa2048,
            "rsa-4096" => Self::Rsa4096,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CertificateChallenge {
    /// The cert SAN this challenge belongs to (as the customer wrote it).
    #[serde(rename = "domain", default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Full LHS of the CNAME record the customer needs to add. For wildcard SANs this is the parent name (`_acme-challenge.example.com.`), not the literal SAN — wildcards validate at their parent under RFC 8555 §8.4.
    #[serde(
        rename = "cname_record_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cname_record_name: Option<String>,
    /// Target FQDN (RHS of the CNAME). Hosted in the platform's validation zone, where the per-order TXT is published during issuance.
    #[serde(
        rename = "expected_cname",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_cname: Option<String>,
    /// True when the domain is hosted on the platform DNS service and the CNAME was created automatically. False means the customer owns the zone and must add the CNAME at their registrar.
    #[serde(rename = "our_dns", default, skip_serializing_if = "Option::is_none")]
    pub our_dns: Option<bool>,
    /// True once the CNAME has resolved to expected_cname; cert issuance only proceeds when every challenge is verified.
    #[serde(rename = "verified", default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    #[serde(
        rename = "verified_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub verified_at: Option<String>,
    /// Active verification warnings only; empty when healthy. Successful verification resolves verification faults while retaining history. verified records a successful observation and remains true if a later renewal observes a DNS failure. A CNAME mismatch records CERTIFICATE_DNS_VERIFICATION_FAILED as a warning; verified does not move. Internal history uses the parent CRN followed by /challenge/&lt;stored-uuid&gt;; no separate endpoint is exposed.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Fault {
    /// Stable machine-readable code owned by the reporting operation.
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "severity")]
    pub severity: FaultSeverity,

    #[serde(rename = "message")]
    pub message: String,
    /// Structured context; legacy strings are preserved in legacy_text.
    #[serde(rename = "details")]
    pub details: crate::Nullable<std::collections::BTreeMap<String, serde_json::Value>>,
    /// First observation in this active occurrence series.
    #[serde(rename = "first_at")]
    pub first_at: String,
    /// Latest observation in this active occurrence series.
    #[serde(rename = "last_at")]
    pub last_at: String,

    #[serde(rename = "occurrences")]
    pub occurrences: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FaultSeverity {
    Error,
    Warning,
    Unknown(String),
}
impl FaultSeverity {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FaultSeverity {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FaultSeverity {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "error" => Self::Error,
            "warning" => Self::Warning,
            _ => Self::Unknown(value),
        })
    }
}

pub type Tags = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CreateCertificateResult {
    Variant1(Box<CertificateResponse>),
}

pub type CreateCertificateResponse = CreateCertificateResult;

pub type GetCertificateResponse = CertificateResponse;

pub type GetCertificateResource = Certificate;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetCertificateScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct MaterialResponse {
    #[serde(rename = "material", default, skip_serializing_if = "Option::is_none")]
    pub material: Option<Material>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Material {
    /// PEM-encoded leaf certificate.
    #[serde(
        rename = "certificate_pem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_pem: Option<String>,
    /// PEM-encoded intermediate chain.
    #[serde(rename = "chain_pem", default, skip_serializing_if = "Option::is_none")]
    pub chain_pem: Option<String>,
    /// PEM-encoded private key (decrypted).
    #[serde(
        rename = "private_key_pem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub private_key_pem: Option<String>,
    /// Hex SHA-256 of the leaf's DER. A caller confirms this matches the version the feed named before installing the material.
    #[serde(
        rename = "fingerprint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fingerprint: Option<String>,
}
impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Material");
        d.field("certificate_pem", &self.certificate_pem);
        d.field("chain_pem", &self.chain_pem);
        d.field("private_key_pem", &"[REDACTED]");
        d.field("fingerprint", &self.fingerprint);
        d.finish()
    }
}

pub type GetCertificateMaterialResponse = MaterialResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListCertificatesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListCertificatesQuery = ListCertificatesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CertificateListResponse {
    #[serde(
        rename = "certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificates: Option<Vec<Certificate>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
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

pub type ListCertificatesResponse = CertificateListResponse;

pub type ListCertificatesItem = Certificate;

pub type RevokeCertificateResponse = CertificateResponse;
