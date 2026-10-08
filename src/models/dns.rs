//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VPCAssociationRequestInput {
    /// Account-owned VPC UUID or network/vpc CRN to associate with this private zone. Bare names return 400 with "VPC references on DNS zones must be a UUID or a CRN, which carries the region". CRNs resolve in their named region; UUIDs search all regions enabled for DNS. Missing, foreign-account or unconfigured-region VPCs return 404. Incomplete UUID searches or duplicate regional UUID identities fail with a server error. The response contains the canonical VPC UUID.
    #[serde(rename = "vpc")]
    pub vpc: String,
}
impl VPCAssociationRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(vpc: String) -> Self {
        Self { vpc }
    }
}

pub type AssociateZoneVPCBody = VPCAssociationRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VPCAssociationAccepted {
    #[serde(rename = "zone_id")]
    pub zone_id: String,

    #[serde(rename = "vpc_id")]
    pub vpc_id: String,
}

pub type AssociateZoneVPCResponse = VPCAssociationAccepted;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RecordCreateRequestInput {
    /// Record name (FQDN).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub type_: RecordTypeInput,

    #[serde(rename = "ttl", default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,

    #[serde(rename = "values")]
    pub values: Vec<RecordValueInput>,
}
impl RecordCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, type_: RecordTypeInput, values: Vec<RecordValueInput>) -> Self {
        Self {
            name,
            type_,
            ttl: None,
            values,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecordTypeInput {
    A,
    AAAA,
    AFSDB,
    APL,
    CAA,
    CERT,
    CNAME,
    CSYNC,
    DHCID,
    DNAME,
    EUI48,
    EUI64,
    HINFO,
    HTTPS,
    IPSECKEY,
    KX,
    L32,
    L64,
    LOC,
    LP,
    MX,
    NAPTR,
    NID,
    NS,
    OPENPGPKEY,
    PTR,
    RKEY,
    RP,
    SMIMEA,
    SPF,
    SRV,
    SSHFP,
    SVCB,
    TLSA,
    TXT,
    URI,
    Unknown(String),
}
impl RecordTypeInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::A => "A",
            Self::AAAA => "AAAA",
            Self::AFSDB => "AFSDB",
            Self::APL => "APL",
            Self::CAA => "CAA",
            Self::CERT => "CERT",
            Self::CNAME => "CNAME",
            Self::CSYNC => "CSYNC",
            Self::DHCID => "DHCID",
            Self::DNAME => "DNAME",
            Self::EUI48 => "EUI48",
            Self::EUI64 => "EUI64",
            Self::HINFO => "HINFO",
            Self::HTTPS => "HTTPS",
            Self::IPSECKEY => "IPSECKEY",
            Self::KX => "KX",
            Self::L32 => "L32",
            Self::L64 => "L64",
            Self::LOC => "LOC",
            Self::LP => "LP",
            Self::MX => "MX",
            Self::NAPTR => "NAPTR",
            Self::NID => "NID",
            Self::NS => "NS",
            Self::OPENPGPKEY => "OPENPGPKEY",
            Self::PTR => "PTR",
            Self::RKEY => "RKEY",
            Self::RP => "RP",
            Self::SMIMEA => "SMIMEA",
            Self::SPF => "SPF",
            Self::SRV => "SRV",
            Self::SSHFP => "SSHFP",
            Self::SVCB => "SVCB",
            Self::TLSA => "TLSA",
            Self::TXT => "TXT",
            Self::URI => "URI",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RecordTypeInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RecordTypeInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "A" => Self::A,
            "AAAA" => Self::AAAA,
            "AFSDB" => Self::AFSDB,
            "APL" => Self::APL,
            "CAA" => Self::CAA,
            "CERT" => Self::CERT,
            "CNAME" => Self::CNAME,
            "CSYNC" => Self::CSYNC,
            "DHCID" => Self::DHCID,
            "DNAME" => Self::DNAME,
            "EUI48" => Self::EUI48,
            "EUI64" => Self::EUI64,
            "HINFO" => Self::HINFO,
            "HTTPS" => Self::HTTPS,
            "IPSECKEY" => Self::IPSECKEY,
            "KX" => Self::KX,
            "L32" => Self::L32,
            "L64" => Self::L64,
            "LOC" => Self::LOC,
            "LP" => Self::LP,
            "MX" => Self::MX,
            "NAPTR" => Self::NAPTR,
            "NID" => Self::NID,
            "NS" => Self::NS,
            "OPENPGPKEY" => Self::OPENPGPKEY,
            "PTR" => Self::PTR,
            "RKEY" => Self::RKEY,
            "RP" => Self::RP,
            "SMIMEA" => Self::SMIMEA,
            "SPF" => Self::SPF,
            "SRV" => Self::SRV,
            "SSHFP" => Self::SSHFP,
            "SVCB" => Self::SVCB,
            "TLSA" => Self::TLSA,
            "TXT" => Self::TXT,
            "URI" => Self::URI,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RecordValueInput {
    /// RDATA — wire representation per record type.
    #[serde(rename = "content")]
    pub content: String,
    /// Must be `false`. `true` is **refused**. There is nowhere to keep a value that is not served, so a disabled value used to be accepted, echoed back in the response, and then dropped — the staged value was gone by the next read, with the write having reported success. Refusing is the honest version. The field remains on the schema because the console and CLI send it on every value; only `true` is rejected. To take a value out of an RRset, remove it from `values`.
    #[serde(rename = "disabled", default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}
impl RecordValueInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(content: String) -> Self {
        Self {
            content,
            disabled: None,
        }
    }
}

pub type CreateRecordBody = RecordCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordResponse {
    #[serde(rename = "record", default, skip_serializing_if = "Option::is_none")]
    pub record: Option<Record>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Record {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "zone_id", default, skip_serializing_if = "Option::is_none")]
    pub zone_id: Option<String>,
    /// Record name (FQDN).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Record type. Customer records use one of the creatable RecordType values; reads also surface the platform-managed apex/DNSSEC types (SOA, NS, DNSKEY, DS, NSEC, RRSIG).
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "ttl", default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// True for platform-managed records (SOA, apex NS, and the DNSSEC set). Managed records are read-only — they cannot be updated or deleted through the API.
    #[serde(rename = "managed", default, skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,

    #[serde(rename = "values", default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<RecordValue>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RecordValue {
    /// RDATA — wire representation per record type.
    #[serde(rename = "content")]
    pub content: String,
    /// Must be `false`. `true` is **refused**. There is nowhere to keep a value that is not served, so a disabled value used to be accepted, echoed back in the response, and then dropped — the staged value was gone by the next read, with the write having reported success. Refusing is the honest version. The field remains on the schema because the console and CLI send it on every value; only `true` is rejected. To take a value out of an RRset, remove it from `values`.
    #[serde(rename = "disabled", default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

pub type CreateRecordResponse = RecordResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ZoneCreateRequestInput {
    /// Zone FQDN. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// Free-form note stored with and returned on the zone.
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// `private` restricts the zone to the VPCs named in `vpcs` and requires at least one; `public` (the default) rejects `vpcs` outright rather than ignoring them. Cannot be changed afterwards.
    #[serde(
        rename = "visibility",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility: Option<ZoneCreateRequestInputVisibility>,
    /// Sign the zone with DNSSEC. On unless you say otherwise, and almost every zone should leave it on. **Turn it off only if this domain is served by another DNS provider at the same time as us.** A signed zone puts our DS record at the parent, and that DS covers only the answers WE sign — so a validating resolver that happens to ask the other provider gets a signature it cannot verify and fails the lookup. Roughly half your queries, unpredictably, which is worse than either provider on its own. Unsigned is the only configuration that works for that setup today. Fixed at creation. Turning signing off later breaks the domain until the DS is withdrawn at the registrar and that withdrawal has propagated, which is a sequence this API cannot drive for you.
    #[serde(rename = "dnssec", default, skip_serializing_if = "Option::is_none")]
    pub dnssec: Option<bool>,
    /// Read the domain's records from the nameservers that serve it TODAY and copy them into this zone, before you move the delegation here. Worth asking for when you are migrating a live domain. The delegation is the ownership proof, so the moment you point your registrar at this zone is the moment we start answering for it — and an empty zone answers with nothing, which takes the site and the mail down until you have retyped everything. Runs in the background; the zone is created immediately. Poll GET /v1/zones/{zone_id}/record-import for the outcome. Best effort, and the result says how good it was. A zone transfer is exhaustive and almost always refused; the fallback queries a list of common names and cannot find a record it did not think to ask for. Check `record_import.complete` before you switch your old provider off. Records you have already created are never overwritten, and records this platform manages itself — the SOA, the DNSSEC chain, the zone's nameservers — are never imported.
    #[serde(
        rename = "import_existing_records",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_existing_records: Option<bool>,
    /// Account-owned VPC UUIDs or network/vpc CRNs the zone resolves in. Bare names are rejected with 400 because the request fixes no region. CRNs resolve in their named region; UUIDs search all regions enabled for DNS. Missing, foreign-account or unconfigured-region VPCs return 404. Incomplete UUID searches or duplicate regional UUID identities fail with a server error. References are deduplicated by UUID. Required when visibility=private, rejected when visibility=public. More can be associated later via POST /v1/zones/{zone_id}/vpc-associations.
    #[serde(rename = "vpcs", default, skip_serializing_if = "Option::is_none")]
    pub vpcs: Option<Vec<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl ZoneCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            visibility: None,
            dnssec: None,
            import_existing_records: None,
            vpcs: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZoneCreateRequestInputVisibility {
    Public,
    Private,
    Unknown(String),
}
impl ZoneCreateRequestInputVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ZoneCreateRequestInputVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ZoneCreateRequestInputVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => Self::Unknown(value),
        })
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateZoneBody = ZoneCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneResponse {
    #[serde(rename = "zone", default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<Zone>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct Zone {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Zone FQDN. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Free-form description, editable with PATCH.
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Authoritative nameservers for the zone — the apex NS set. Copy this list verbatim into your registrar's nameserver configuration to delegate the zone to the platform. These names are unique to THIS zone: each carries a per-zone label, which is what makes the delegation double as the ownership proof (see `ownership`). Two zones for the same domain get different names, and the one the registrar points at is the one that serves. Use them exactly as written — Basaltic's bare nameserver names will not verify the zone.
    #[serde(
        rename = "nameservers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub nameservers: Option<Vec<String>>,

    #[serde(rename = "soa", default, skip_serializing_if = "Option::is_none")]
    pub soa: Option<SOA>,
    /// `public` zones answer on the internet-facing nameservers; `private` zones answer only inside the VPCs associated with them (see the vpc-associations endpoints). Fixed at creation.
    #[serde(
        rename = "visibility",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility: Option<ZoneVisibility>,

    #[serde(rename = "dnssec", default, skip_serializing_if = "Option::is_none")]
    pub dnssec: Option<ZoneDNSSEC>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(rename = "ownership", default, skip_serializing_if = "Option::is_none")]
    pub ownership: Option<ZoneOwnership>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SOA {
    /// SOA `mname` — first authoritative nameserver for the zone.
    #[serde(
        rename = "primary_ns",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_ns: Option<String>,
    /// SOA `rname` — the platform's DNS-operations contact. Stamped server-side; not customer-configurable.
    #[serde(
        rename = "admin_email",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_email: Option<String>,

    #[serde(rename = "refresh", default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<i64>,

    #[serde(rename = "retry", default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<i64>,

    #[serde(rename = "expire", default, skip_serializing_if = "Option::is_none")]
    pub expire: Option<i64>,
    /// NXDOMAIN cache TTL.
    #[serde(rename = "minimum", default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZoneVisibility {
    Public,
    Private,
    Unknown(String),
}
impl ZoneVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ZoneVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ZoneVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ZoneDNSSEC {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Key tag of the key-signing key — matches the DS records below.
    #[serde(
        rename = "ksk_key_tag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ksk_key_tag: Option<i64>,
    /// Key tag of the zone-signing key.
    #[serde(
        rename = "zsk_key_tag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub zsk_key_tag: Option<i64>,
    /// DNSSEC algorithm number. 13 = ECDSA P-256 SHA-256.
    #[serde(rename = "algorithm", default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<i64>,

    #[serde(
        rename = "ds_records",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ds_records: Option<Vec<ZoneDSRecord>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ZoneDSRecord {
    #[serde(rename = "key_tag")]
    pub key_tag: i64,

    #[serde(rename = "algorithm")]
    pub algorithm: i64,
    /// 2 = SHA-256.
    #[serde(rename = "digest_type")]
    pub digest_type: i64,

    #[serde(rename = "digest")]
    pub digest: String,
    /// Full zone-file form — `<key_tag> <algorithm> <digest_type> <digest>`.
    #[serde(rename = "rdata")]
    pub rdata: String,
}

pub type Tags = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneOwnership {
    /// Whether the zone has proved ownership. Unverified zones do not resolve.
    #[serde(rename = "verified", default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// When ownership was first proved. Absent while unverified.
    #[serde(
        rename = "verified_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub verified_at: Option<crate::Nullable<String>>,
    /// When the proof was last re-confirmed. `verified_at` keeps meaning "first demonstrated" and does not move; this does, on every pass that passes. Absent until the zone has been re-confirmed at least once.
    #[serde(
        rename = "checked_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub checked_at: Option<String>,
    /// Present ONLY while the periodic re-proof is failing: the instant the zone stops answering unless it passes again. Absent means healthy. Each pass re-checks a failing zone, so reaching this date takes sustained failure, not one bad afternoon — a transient resolver problem clears itself on the next pass. To clear it deliberately, point the domain's delegation back at this zone's `nameservers` and POST /v1/zones/{zone_id}/verify-ownership. The organization's owner is emailed when this date is set, and again if the zone does stop resolving. A zone is never taken off the air before that message has gone out, so the date here is the earliest the zone can stop answering and never the only warning.
    #[serde(
        rename = "recheck_deadline",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recheck_deadline: Option<String>,
}

pub type CreateZoneResponse = ZoneResponse;

pub type GetRecordResponse = RecordResponse;

pub type GetRecordResource = Record;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRecordScope {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(
        rename = "include_managed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_managed: Option<bool>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetZoneResponse = ZoneResponse;

pub type GetZoneResource = Zone;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetZoneScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneRecordImportResponse {
    #[serde(
        rename = "record_import",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub record_import: Option<ZoneRecordImport>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneRecordImport {
    /// `pending` while the background job runs. `complete` means the scan ran and what it found was applied — not that everything the domain has is now here; see `complete`. `pending` is bounded. A scan is capped well below the point at which this stops reporting it, so a job that dies without recording an outcome is reported as `failed` rather than staying `pending` for the life of the zone. There is no state that means "still importing" after that bound, and nothing you can poll for that would ever change.
    #[serde(rename = "state", default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ZoneRecordImportState>,
    /// How the records were found, in descending order of how much the result is worth. `axfr` is a zone transfer: the whole zone, exactly. `nsec-walk` follows the zone's own DNSSEC NSEC chain, which names every record set in it — also exact, and available on signed zones whose provider refuses transfers. `query` is a list of common names, which finds what it thought to ask for and cannot know what it missed. Absent while pending.
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<ZoneRecordImportSource>,
    /// True for the two sources that enumerate the zone — `axfr` and `nsec-walk` — and false for `query`. **This is the field to read before switching your old provider off.** A false here means records may exist that we did not find, not that none do.
    #[serde(rename = "complete", default, skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Record sets the scan turned up.
    #[serde(rename = "found", default, skip_serializing_if = "Option::is_none")]
    pub found: Option<i64>,
    /// Record sets actually written. Lower than `found` for records you had already created — yours win — and for the ones this platform manages itself.
    #[serde(rename = "imported", default, skip_serializing_if = "Option::is_none")]
    pub imported: Option<i64>,
    /// What could not be established, and what was deliberately not imported.
    #[serde(rename = "notes", default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<String>>,
    /// Present only when state is `failed`, and says what could not be done.
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZoneRecordImportState {
    Pending,
    Complete,
    Failed,
    Unknown(String),
}
impl ZoneRecordImportState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ZoneRecordImportState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ZoneRecordImportState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "complete" => Self::Complete,
            "failed" => Self::Failed,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZoneRecordImportSource {
    Axfr,
    NsecWalk,
    Query,
    Unknown(String),
}
impl ZoneRecordImportSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Axfr => "axfr",
            Self::NsecWalk => "nsec-walk",
            Self::Query => "query",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ZoneRecordImportSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ZoneRecordImportSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "axfr" => Self::Axfr,
            "nsec-walk" => Self::NsecWalk,
            "query" => Self::Query,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetZoneRecordImportResponse = ZoneRecordImportResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ZoneImportRequestInput {
    /// The zone file, as text. `$ORIGIN`, `$TTL`, `$GENERATE`, relative names and parenthesised multi-line records are all honoured; `$INCLUDE` is refused, because the path it names would be read on our filesystem rather than yours.
    #[serde(rename = "zone_file")]
    pub zone_file: String,
}
impl ZoneImportRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(zone_file: String) -> Self {
        Self { zone_file }
    }
}

pub type ImportZoneFileBody = ZoneImportRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneImportResponse {
    #[serde(rename = "import", default, skip_serializing_if = "Option::is_none")]
    pub import: Option<ZoneImportResult>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneImportResult {
    /// RRsets in the file that the zone did not already have.
    #[serde(
        rename = "records_created",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub records_created: Option<i64>,
    /// RRsets that existed at the same name and type and were replaced wholesale by the file's values.
    #[serde(
        rename = "records_replaced",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub records_replaced: Option<i64>,
    /// Imported RRset count per record type.
    #[serde(
        rename = "records_by_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub records_by_type: Option<std::collections::BTreeMap<String, i64>>,

    #[serde(rename = "skipped", default, skip_serializing_if = "Option::is_none")]
    pub skipped: Option<Vec<ZoneImportSkipped>>,
    /// Records that were imported, but not exactly as written — an RRset the file gave more than one TTL, for instance.
    #[serde(rename = "warnings", default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneImportSkipped {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

pub type ImportZoneFileResponse = ZoneImportResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRecordsParameters {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(
        rename = "include_managed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_managed: Option<bool>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListRecordsQuery = ListRecordsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordListResponse {
    #[serde(rename = "records", default, skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,

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

pub type ListRecordsResponse = RecordListResponse;

pub type ListRecordsItem = Record;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListZoneVPCAssociationsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListZoneVPCAssociationsQuery = ListZoneVPCAssociationsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VPCAssociationsResponse {
    #[serde(rename = "vpc_ids")]
    pub vpc_ids: Vec<String>,
}

pub type ListZoneVPCAssociationsResponse = VPCAssociationsResponse;

pub type ListZoneVPCAssociationsItem = String;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListZonesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListZonesQuery = ListZonesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneListResponse {
    #[serde(rename = "zones", default, skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<Zone>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListZonesResponse = ZoneListResponse;

pub type ListZonesItem = Zone;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RecordUpdateRequestInput {
    #[serde(rename = "ttl", default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,

    #[serde(rename = "values", default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<RecordValueInput>>,
}

pub type UpdateRecordBody = RecordUpdateRequestInput;

pub type UpdateRecordResponse = RecordResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ZoneUpdateRequestInput {
    /// Omit to preserve the description; send an empty string to clear it.
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, serde_json::Value>>,
}

pub type UpdateZoneBody = ZoneUpdateRequestInput;

pub type UpdateZoneResponse = ZoneResponse;

pub type VerifyZoneOwnershipResponse = ZoneResponse;
