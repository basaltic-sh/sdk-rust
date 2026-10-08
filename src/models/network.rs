//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachFloatingIpRequest {
    /// Interface UUID or nested CRN. Bare names have no subnet scope and are rejected.
    #[serde(rename = "interface")]
    pub interface: String,

    #[serde(rename = "address_id")]
    pub address_id: String,
}
impl AttachFloatingIpRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(interface: String, address_id: String) -> Self {
        Self {
            interface,
            address_id,
        }
    }
}

pub type AttachFloatingIpBody = AttachFloatingIpRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct AttachFloatingIpResult {
    #[serde(
        rename = "floating_ip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ip: Option<FloatingIp>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIp {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "family")]
    pub family: IpFamily,
    /// Canonical CRN of the bound interface, instance pool, or load balancer; null when unattached. A pool-owned address names its pool even when the pool has zero members. Only pool-owned addresses may have multiple NIC members. Manage their bindings through the instance pool floating IP endpoints; direct attach and detach are refused.
    #[serde(rename = "attached_to")]
    pub attached_to: crate::Nullable<String>,
    /// The floating IP's bindings. A floating IP fronts 0 members (allocated, unattached), 1 member (the everyday case), or N members for an instance pool — an anycast floating IP, where one public IP is delivered to N VM NICs across hosts (each advertised as a /32 from the host holding it). Members may share a hypervisor. Two of them on one host used to mean one served and the other was silently dark; a member's forwarding rule now names the member, and the host splits connections across the members it holds, so where the members sit is a capacity decision rather than a correctness one. An instance pool's address takes its members from the pool's live replicas — every one of them — so a scale-out joins and a scale-in leaves without a per-replica attach. With more than one member ONE member serves each connection, chosen by hashing the flow's addresses and ports, and every packet of that connection goes to the same one. The members are separate instances that share nothing, so this spreads connections and survives the loss of a host — it is not a load balancer: nothing checks whether the service inside the instance is up, and connections in progress to a member that goes away are not moved, they end. A POOL's address is the exception, and only for booting. A replica joins the address as soon as it is placed, but does not receive traffic until it has reached the instance metadata service — evidence that the guest booted, rather than that its virtual machine was started. Until then it is a member with `health` `unhealthy`. A replica whose image never contacts the metadata service is admitted anyway after a few minutes, so an unusual image delays traffic rather than never getting it.
    #[serde(rename = "members")]
    pub members: Vec<FloatingIpMember>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,
    /// The readiness check applied to this address's members. Absent when none is configured. See `FloatingIpHealthCheck`.
    #[serde(
        rename = "health_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub health_check: Option<FloatingIpHealthCheck>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Allocated public or private address.
    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "visibility")]
    pub visibility: FloatingIpVisibility,
    /// Allocation subnet for private floating IPs.
    #[serde(
        rename = "subnet_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub subnet_id: Option<crate::Nullable<String>>,
    /// Allocation VPC for private floating IPs.
    #[serde(rename = "vpc_id", default, skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IpFamily {
    Ipv4,
    Ipv6,
    Unknown(String),
}
impl IpFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for IpFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IpFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIpMember {
    /// Bound NIC summary; null for a load balancer binding named by attached_to.
    #[serde(rename = "interface")]
    pub interface: crate::Nullable<FloatingIpMemberInterfaceValue>,
    /// What the platform knows about this member. `unknown` — nobody is checking. A member you attached yourself with no health check on the address reads this: you chose the moment of attach, and the platform has no signal about what runs inside the instance. It is advertised. `healthy` — the platform has evidence this member is up (and, if a health check is configured on the address, that the check is passing). `unhealthy` — the platform is waiting for that evidence and has not seen it, or a configured check is failing. The member keeps its place on the address and receives no traffic until it recovers. Without a health check this is liveness only — `healthy` means the guest came up, not that your service is listening. Configure `health_check` on the floating IP to add readiness on top of that.
    #[serde(rename = "health")]
    pub health: FloatingIpMemberHealth,
    /// Why the member reads the `health` it does — so you can tell "your service is not answering" from "the guest has not booted yet". `unprobed` — nobody is checking (no health check, hand-attached). `booting` — the platform has not yet seen the guest come up. `probe_failed` — the configured health check is failing. `passing` — the guest is up and, if a check is configured, it passes.
    #[serde(rename = "reason")]
    pub reason: FloatingIpMemberReason,

    #[serde(rename = "created_at")]
    pub created_at: String,
    /// Target child address on the member interface.
    #[serde(
        rename = "address_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub address_id: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIpMemberInterfaceValue {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Owning instance; null when the interface has no owning instance.
    #[serde(rename = "instance")]
    pub instance: crate::Nullable<FloatingIpMemberInterfaceValueInstanceValue>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIpMemberInterfaceValueInstanceValue {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingIpMemberHealth {
    Unknown1,
    Healthy,
    Unhealthy,
    Unknown(String),
}
impl FloatingIpMemberHealth {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unknown1 => "unknown",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FloatingIpMemberHealth {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpMemberHealth {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "unknown" => Self::Unknown1,
            "healthy" => Self::Healthy,
            "unhealthy" => Self::Unhealthy,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingIpMemberReason {
    Unprobed,
    Booting,
    ProbeFailed,
    Passing,
    Unknown(String),
}
impl FloatingIpMemberReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unprobed => "unprobed",
            Self::Booting => "booting",
            Self::ProbeFailed => "probe_failed",
            Self::Passing => "passing",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FloatingIpMemberReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpMemberReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "unprobed" => Self::Unprobed,
            "booting" => Self::Booting,
            "probe_failed" => Self::ProbeFailed,
            "passing" => Self::Passing,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIpHealthCheck {
    /// `tcp` opens a connection; `http`/`https` issue a GET and match the status against `matcher`. There is no `udp`: a readiness probe needs an answer — check a udp service on a tcp health port instead.
    #[serde(rename = "protocol")]
    pub protocol: FloatingIpHealthCheckProtocol,
    /// HTTP path probed; ignored for tcp.
    #[serde(rename = "path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port probed on the member.
    #[serde(rename = "port")]
    pub port: i64,

    #[serde(rename = "interval_sec")]
    pub interval_sec: i64,
    /// Per-probe timeout; must be less than interval_sec.
    #[serde(rename = "timeout_sec")]
    pub timeout_sec: i64,
    /// Consecutive passes before a member flips healthy.
    #[serde(rename = "healthy_threshold")]
    pub healthy_threshold: i64,
    /// Consecutive failures before a member flips unhealthy.
    #[serde(rename = "unhealthy_threshold")]
    pub unhealthy_threshold: i64,
    /// HTTP status or range that counts as passing; ignored for tcp.
    #[serde(rename = "matcher", default, skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingIpHealthCheckProtocol {
    Tcp,
    Http,
    Https,
    Unknown(String),
}
impl FloatingIpHealthCheckProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tcp => "tcp",
            Self::Http => "http",
            Self::Https => "https",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FloatingIpHealthCheckProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpHealthCheckProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "tcp" => Self::Tcp,
            "http" => Self::Http,
            "https" => Self::Https,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingIpVisibility {
    Public,
    Private,
    Unknown(String),
}
impl FloatingIpVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FloatingIpVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => Self::Unknown(value),
        })
    }
}

pub type AttachFloatingIpResponse = AttachFloatingIpResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InternetGatewayAttachRequestInput {
    /// VPC UUID, CRN or exact name in the caller account.
    #[serde(rename = "vpc")]
    pub vpc: String,
}
impl InternetGatewayAttachRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(vpc: String) -> Self {
        Self { vpc }
    }
}

pub type AttachInternetGatewayBody = InternetGatewayAttachRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InternetGatewayResponse {
    #[serde(
        rename = "internet_gateway",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internet_gateway: Option<InternetGateway>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InternetGateway {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// VPC the IGW is currently attached to (null when detached).
    #[serde(
        rename = "attached_vpc_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub attached_vpc_id: Option<crate::Nullable<String>>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type AttachInternetGatewayResponse = InternetGatewayResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EgressOnlyGatewayCreateRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// VPC UUID, CRN or exact name in the caller account.
    #[serde(rename = "vpc")]
    pub vpc: String,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl EgressOnlyGatewayCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, vpc: String) -> Self {
        Self {
            name,
            description: None,
            vpc,
            tags: None,
        }
    }
}

pub type CreateEgressOnlyGatewayBody = EgressOnlyGatewayCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct EgressOnlyGatewayResponse {
    #[serde(
        rename = "egress_only_gateway",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub egress_only_gateway: Option<EgressOnlyGateway>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EgressOnlyGateway {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "vpc")]
    pub vpc: Vpc,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Vpc {
    #[serde(rename = "id")]
    pub id: String,
    /// Cloud Resource Name (name-based, region+account-scoped).
    #[serde(rename = "crn")]
    pub crn: String,
    /// 1-63 chars, lowercase alphanumeric + hyphen Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// IPv4 CIDR block carved up by subnets. Must be private (RFC 1918): within 10.0.0.0/8, 172.16.0.0/12 or 192.168.0.0/16. Immutable after create.
    #[serde(rename = "cidr_ipv4")]
    pub cidr_ipv4: String,
    /// Associated regional GUA or private ULA prefix.
    #[serde(
        rename = "cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub cidr_ipv6: Option<crate::Nullable<String>>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type CreateEgressOnlyGatewayResponse = EgressOnlyGatewayResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct FloatingIpCreateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Which family to allocate in. Fixed for the life of the address — it decides the pool the address comes from, the quota it counts against (`floating_ips_v4` or `floating_ips_v6`) and the SKU it bills as. Omitted means `ipv4`.
    #[serde(rename = "family", default, skip_serializing_if = "Option::is_none")]
    pub family: Option<IpFamilyInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// An optional readiness check for the address's members. Omitted means none — the address behaves exactly as an ordinary floating IP.
    #[serde(
        rename = "health_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub health_check: Option<FloatingIpHealthCheckInput>,

    #[serde(
        rename = "visibility",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility: Option<FloatingIpCreateRequestInputVisibility>,
    /// Required for private floating IPs; subnet UUID or CRN in this account. Targets may be in other subnets of the same VPC.
    #[serde(rename = "subnet", default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IpFamilyInput {
    Ipv4,
    Ipv6,
    Unknown(String),
}
impl IpFamilyInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for IpFamilyInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IpFamilyInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIpHealthCheckInput {
    /// `tcp` opens a connection; `http`/`https` issue a GET and match the status against `matcher`. There is no `udp`: a readiness probe needs an answer — check a udp service on a tcp health port instead.
    #[serde(rename = "protocol")]
    pub protocol: FloatingIpHealthCheckInputProtocol,
    /// HTTP path probed; ignored for tcp.
    #[serde(rename = "path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port probed on the member.
    #[serde(rename = "port")]
    pub port: i64,

    #[serde(rename = "interval_sec")]
    pub interval_sec: i64,
    /// Per-probe timeout; must be less than interval_sec.
    #[serde(rename = "timeout_sec")]
    pub timeout_sec: i64,
    /// Consecutive passes before a member flips healthy.
    #[serde(rename = "healthy_threshold")]
    pub healthy_threshold: i64,
    /// Consecutive failures before a member flips unhealthy.
    #[serde(rename = "unhealthy_threshold")]
    pub unhealthy_threshold: i64,
    /// HTTP status or range that counts as passing; ignored for tcp.
    #[serde(rename = "matcher", default, skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
}
impl FloatingIpHealthCheckInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        protocol: FloatingIpHealthCheckInputProtocol,
        port: i64,
        interval_sec: i64,
        timeout_sec: i64,
        healthy_threshold: i64,
        unhealthy_threshold: i64,
    ) -> Self {
        Self {
            protocol,
            path: None,
            port,
            interval_sec,
            timeout_sec,
            healthy_threshold,
            unhealthy_threshold,
            matcher: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingIpHealthCheckInputProtocol {
    Tcp,
    Http,
    Https,
    Unknown(String),
}
impl FloatingIpHealthCheckInputProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tcp => "tcp",
            Self::Http => "http",
            Self::Https => "https",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FloatingIpHealthCheckInputProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpHealthCheckInputProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "tcp" => Self::Tcp,
            "http" => Self::Http,
            "https" => Self::Https,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingIpCreateRequestInputVisibility {
    Public,
    Private,
    Unknown(String),
}
impl FloatingIpCreateRequestInputVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FloatingIpCreateRequestInputVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIpCreateRequestInputVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "public" => Self::Public,
            "private" => Self::Private,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateFloatingIpBody = FloatingIpCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct FloatingIpResponse {
    #[serde(
        rename = "floating_ip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ip: Option<FloatingIp>,
}

pub type CreateFloatingIpResponse = FloatingIpResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceCreateRequestInput {
    /// Subnet UUID or nested CRN (vpc/&lt;vpc&gt;/subnet/&lt;subnet&gt;). A bare name requires an explicit VPC filter; create requests without a VPC do not accept bare names.
    #[serde(rename = "subnet")]
    pub subnet: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Defaults to a fresh locally-administered EUI-48
    #[serde(
        rename = "mac",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub mac: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Every enabled subnet family is allocated automatically. Entries may request a fixed IPv4 address; omitting a family never disables it. At most one entry per family.
    #[serde(rename = "addresses", default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<AddressRequestInput>>,
}
impl InterfaceCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(subnet: String, name: String) -> Self {
        Self {
            subnet,
            name,
            description: None,
            mac: None,
            tags: None,
            addresses: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddressRequestInput {
    #[serde(rename = "family")]
    pub family: AddressRequestInputFamily,
    /// Optional fixed address when creating an interface or instance NIC. For IPv6, use the first address of an aligned /96 inside the subnet /64 (last 32 bits zero); the first and last /96 ranges are reserved. Omit for automatic allocation. Managed database nodes and the add-address operation require automatic allocation.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
impl AddressRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(family: AddressRequestInputFamily) -> Self {
        Self {
            family,
            address: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddressRequestInputFamily {
    Ipv4,
    Ipv6,
    Unknown(String),
}
impl AddressRequestInputFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AddressRequestInputFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AddressRequestInputFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateInterfaceBody = InterfaceCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InterfaceResponse {
    #[serde(rename = "interface", default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<Interface>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Interface {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "subnet")]
    pub subnet: Subnet,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "mac")]
    pub mac: String,
    /// UUID of the instance holding this interface, including stopped instances. Null when no instance NIC binding exists. Deletion is refused while bound; floating IP attachment is tracked separately.
    #[serde(
        rename = "attached_to",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub attached_to: Option<crate::Nullable<String>>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "addresses")]
    pub addresses: Vec<InterfaceAddress>,

    #[serde(rename = "routed_prefixes")]
    pub routed_prefixes: Vec<RoutedPrefix>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Subnet {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "vpc")]
    pub vpc: Vpc,

    #[serde(rename = "route_table")]
    pub route_table: RouteTableSummary,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "cidr_ipv4")]
    pub cidr_ipv4: String,

    #[serde(rename = "gateway_ipv4")]
    pub gateway_ipv4: String,
    /// The dual-stack IPv6 /64, if the subnet is v6-enabled. Its presence (vs the v4 cidr_ipv4) is how a client tells the subnet's families apart.
    #[serde(
        rename = "cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub cidr_ipv6: Option<crate::Nullable<String>>,

    #[serde(
        rename = "gateway_ipv6",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub gateway_ipv6: Option<crate::Nullable<String>>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type RouteTableSummary = crate::Nullable<RouteTableSummaryValue>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteTableSummaryValue {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceAddress {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "family")]
    pub family: InterfaceAddressFamily,

    #[serde(rename = "address")]
    pub address: String,
    /// Owned allocation, not the guest netmask: IPv4 /32 or IPv6 /96. DHCPv6 configures the first /128.
    #[serde(rename = "prefix")]
    pub prefix: String,

    #[serde(rename = "primary")]
    pub primary: bool,

    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<AddressFloatingIp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterfaceAddressFamily {
    Ipv4,
    Ipv6,
    Unknown(String),
}
impl InterfaceAddressFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InterfaceAddressFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InterfaceAddressFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddressFloatingIp {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "visibility")]
    pub visibility: AddressFloatingIpVisibility,

    #[serde(rename = "address")]
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddressFloatingIpVisibility {
    Public,
    Private,
    Unknown(String),
}
impl AddressFloatingIpVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AddressFloatingIpVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AddressFloatingIpVisibility {
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
pub struct RoutedPrefix {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "pool_id")]
    pub pool_id: String,

    #[serde(rename = "family")]
    pub family: RoutedPrefixFamily,
    /// A routed /28 from a VPC prefix pool.
    #[serde(rename = "prefix")]
    pub prefix: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutedPrefixFamily {
    Ipv4,
    Unknown(String),
}
impl RoutedPrefixFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RoutedPrefixFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RoutedPrefixFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ipv4" => Self::Ipv4,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateInterfaceResponse = InterfaceResponse;

pub type CreateInterfaceAddressBody = AddressRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateInterfaceAddressResult {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<InterfaceAddress>,
}

pub type CreateInterfaceAddressResponse = CreateInterfaceAddressResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateInterfacePrefixRequest {
    #[serde(rename = "pool_id")]
    pub pool_id: String,
}
impl CreateInterfacePrefixRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(pool_id: String) -> Self {
        Self { pool_id }
    }
}

pub type CreateInterfacePrefixBody = CreateInterfacePrefixRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateInterfacePrefixResult {
    #[serde(
        rename = "routed_prefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub routed_prefixes: Option<Vec<RoutedPrefix>>,
}

pub type CreateInterfacePrefixResponse = CreateInterfacePrefixResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InternetGatewayCreateRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl InternetGatewayCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            tags: None,
        }
    }
}

pub type CreateInternetGatewayBody = InternetGatewayCreateRequestInput;

pub type CreateInternetGatewayResponse = InternetGatewayResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NATGatewayCreateRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Subnet UUID or nested CRN (vpc/&lt;vpc&gt;/subnet/&lt;subnet&gt;). A bare name requires an explicit VPC filter; create requests without a VPC do not accept bare names.
    #[serde(rename = "subnet")]
    pub subnet: String,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl NATGatewayCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, subnet: String) -> Self {
        Self {
            name,
            description: None,
            subnet,
            tags: None,
        }
    }
}

pub type CreateNATGatewayBody = NATGatewayCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct NATGatewayResponse {
    #[serde(
        rename = "nat_gateway",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub nat_gateway: Option<NATGateway>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NATGateway {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "subnet")]
    pub subnet: Subnet,
    /// Public IPv4 allocated from the regional pool at creation. Stable until gateway deletion.
    #[serde(rename = "public_ipv4")]
    pub public_ipv4: String,
    /// Public IPv6 allocated from the regional pool when the gateway's hosting subnet has IPv6. Assigned at gateway creation or when IPv6 is enabled on that subnet, independently of routes. Stable until gateway deletion. Shared source NAT supports both global and ULA subnet addresses.
    #[serde(
        rename = "public_ipv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ipv6: Option<String>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type CreateNATGatewayResponse = NATGatewayResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreatePrefixPoolRequest {
    #[serde(rename = "cidr_ipv4")]
    pub cidr_ipv4: String,
}
impl CreatePrefixPoolRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(cidr_ipv4: String) -> Self {
        Self { cidr_ipv4 }
    }
}

pub type CreatePrefixPoolBody = CreatePrefixPoolRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct CreatePrefixPoolResult {
    #[serde(
        rename = "prefix_pools",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_pools: Option<Vec<PrefixPool>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PrefixPool {
    #[serde(rename = "id")]
    pub id: String,
    /// VPC IPv4 range disjoint from all subnets and other prefix pools.
    #[serde(rename = "cidr_ipv4")]
    pub cidr_ipv4: String,
}

pub type CreatePrefixPoolResponse = CreatePrefixPoolResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteCreateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "destination_cidr")]
    pub destination_cidr: String,
    /// Unicast next hop inside this VPC's CIDR (same IP family as destination_cidr). Not for internet egress — use a gateway target id instead.
    #[serde(
        rename = "next_hop_ip",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub next_hop_ip: Option<crate::Nullable<String>>,
    /// Gateway UUID, CRN or exact account-scoped name. Must belong to the route table VPC and match the destination_cidr address family. Exactly one route target is required.
    #[serde(
        rename = "target_internet_gateway",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_internet_gateway: Option<String>,
    /// Gateway UUID, CRN or exact account-scoped name. Must belong to the route table VPC and match the destination_cidr address family. Exactly one route target is required.
    #[serde(
        rename = "target_nat_gateway",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_nat_gateway: Option<String>,
    /// Gateway UUID, CRN or exact account-scoped name. Must belong to the route table VPC and match the destination_cidr address family. Exactly one route target is required.
    #[serde(
        rename = "target_egress_only_gateway",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_egress_only_gateway: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl RouteCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(destination_cidr: String) -> Self {
        Self {
            description: None,
            destination_cidr,
            next_hop_ip: None,
            target_internet_gateway: None,
            target_nat_gateway: None,
            target_egress_only_gateway: None,
            tags: None,
        }
    }
}

pub type CreateRouteBody = RouteCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RouteResponse {
    #[serde(rename = "route", default, skip_serializing_if = "Option::is_none")]
    pub route: Option<Route>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Route {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "route_table_id")]
    pub route_table_id: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "destination_cidr")]
    pub destination_cidr: String,

    #[serde(rename = "target_type")]
    pub target_type: RouteTargetType,
    /// Set when target_type=ip. Mutex with the target_*_id fields. Must be a unicast address inside this VPC's CIDR (same IP family as destination_cidr); internet egress uses target_internet_gateway_id / target_nat_gateway_id.
    #[serde(
        rename = "next_hop_ip",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub next_hop_ip: Option<crate::Nullable<String>>,
    /// Set when target_type=internet_gateway.
    #[serde(
        rename = "target_internet_gateway_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub target_internet_gateway_id: Option<crate::Nullable<String>>,
    /// Set when target_type=nat_gateway. Supports IPv4 and IPv6; IPv6 requires an IPv6-enabled hosting subnet.
    #[serde(
        rename = "target_nat_gateway_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub target_nat_gateway_id: Option<crate::Nullable<String>>,
    /// Set when target_type=egress_only_gateway (IPv6 only). Gives the subnet outbound v6 with the internet unable to initiate inbound.
    #[serde(
        rename = "target_egress_only_gateway_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub target_egress_only_gateway_id: Option<crate::Nullable<String>>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RouteTargetType {
    Ip,
    InternetGateway,
    NatGateway,
    EgressOnlyGateway,
    Unknown(String),
}
impl RouteTargetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ip => "ip",
            Self::InternetGateway => "internet_gateway",
            Self::NatGateway => "nat_gateway",
            Self::EgressOnlyGateway => "egress_only_gateway",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RouteTargetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RouteTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ip" => Self::Ip,
            "internet_gateway" => Self::InternetGateway,
            "nat_gateway" => Self::NatGateway,
            "egress_only_gateway" => Self::EgressOnlyGateway,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateRouteResponse = RouteResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteTableCreateRequestInput {
    /// VPC UUID, CRN or exact name in the caller account.
    #[serde(rename = "vpc")]
    pub vpc: String,
    /// 1-63 chars, lowercase alphanumeric + hyphen. `main` is reserved. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl RouteTableCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(vpc: String, name: String) -> Self {
        Self {
            vpc,
            name,
            description: None,
            tags: None,
        }
    }
}

pub type CreateRouteTableBody = RouteTableCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RouteTableResponse {
    #[serde(
        rename = "route_table",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub route_table: Option<RouteTable>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteTable {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "vpc")]
    pub vpc: Vpc,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// True for the per-VPC default table, named &lt;vpc-name&gt;-private-rt. It is created automatically and can't be deleted. Subnets that don't specify a route_table at create time land here.
    #[serde(rename = "is_main")]
    pub is_main: bool,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type CreateRouteTableResponse = RouteTableResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecurityGroupCreateRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl SecurityGroupCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            tags: None,
        }
    }
}

pub type CreateSecurityGroupBody = SecurityGroupCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SecurityGroupResponse {
    #[serde(
        rename = "security_group",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_group: Option<SecurityGroup>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecurityGroup {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

pub type CreateSecurityGroupResponse = SecurityGroupResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecurityGroupRuleCreateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "direction")]
    pub direction: SecurityGroupRuleDirectionInput,

    #[serde(rename = "ethertype", default, skip_serializing_if = "Option::is_none")]
    pub ethertype: Option<SecurityGroupRuleEthertypeInput>,

    #[serde(rename = "protocol")]
    pub protocol: SecurityGroupRuleProtocolInput,

    #[serde(
        rename = "port_min",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub port_min: Option<crate::Nullable<i64>>,

    #[serde(
        rename = "port_max",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub port_max: Option<crate::Nullable<i64>>,

    #[serde(
        rename = "remote_cidr",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub remote_cidr: Option<crate::Nullable<String>>,
    /// Security-group UUID, CRN or exact account-scoped name. Mutually exclusive with remote_cidr.
    #[serde(
        rename = "source_security_group",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_security_group: Option<String>,
}
impl SecurityGroupRuleCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        direction: SecurityGroupRuleDirectionInput,
        protocol: SecurityGroupRuleProtocolInput,
    ) -> Self {
        Self {
            description: None,
            direction,
            ethertype: None,
            protocol,
            port_min: None,
            port_max: None,
            remote_cidr: None,
            source_security_group: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityGroupRuleDirectionInput {
    Ingress,
    Egress,
    Unknown(String),
}
impl SecurityGroupRuleDirectionInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ingress => "ingress",
            Self::Egress => "egress",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SecurityGroupRuleDirectionInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecurityGroupRuleDirectionInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ingress" => Self::Ingress,
            "egress" => Self::Egress,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityGroupRuleEthertypeInput {
    Ipv4,
    Ipv6,
    Unknown(String),
}
impl SecurityGroupRuleEthertypeInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SecurityGroupRuleEthertypeInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecurityGroupRuleEthertypeInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityGroupRuleProtocolInput {
    Tcp,
    Udp,
    Icmp,
    All,
    Unknown(String),
}
impl SecurityGroupRuleProtocolInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Icmp => "icmp",
            Self::All => "all",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SecurityGroupRuleProtocolInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecurityGroupRuleProtocolInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            "icmp" => Self::Icmp,
            "all" => Self::All,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateSecurityGroupRuleBody = SecurityGroupRuleCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SecurityGroupRuleResponse {
    #[serde(rename = "rule", default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<SecurityGroupRule>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecurityGroupRule {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "security_group_id")]
    pub security_group_id: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "direction")]
    pub direction: SecurityGroupRuleDirection,

    #[serde(rename = "ethertype")]
    pub ethertype: SecurityGroupRuleEthertype,

    #[serde(rename = "protocol")]
    pub protocol: SecurityGroupRuleProtocol,
    /// Required when protocol is tcp/udp; ignored otherwise.
    #[serde(
        rename = "port_min",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub port_min: Option<crate::Nullable<i64>>,

    #[serde(
        rename = "port_max",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub port_max: Option<crate::Nullable<i64>>,
    /// Source (ingress) or destination_cidr (egress) CIDR. Must match the rule's ethertype. Mutually exclusive with source_security_group_id.
    #[serde(
        rename = "remote_cidr",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub remote_cidr: Option<crate::Nullable<String>>,
    /// Source (ingress) or destination_cidr (egress) is "any workload in this SG". Traffic is matched by membership in the named security group. Mutually exclusive with remote_cidr.
    #[serde(
        rename = "source_security_group_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub source_security_group_id: Option<crate::Nullable<String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityGroupRuleDirection {
    Ingress,
    Egress,
    Unknown(String),
}
impl SecurityGroupRuleDirection {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ingress => "ingress",
            Self::Egress => "egress",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SecurityGroupRuleDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecurityGroupRuleDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ingress" => Self::Ingress,
            "egress" => Self::Egress,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityGroupRuleEthertype {
    Ipv4,
    Ipv6,
    Unknown(String),
}
impl SecurityGroupRuleEthertype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SecurityGroupRuleEthertype {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecurityGroupRuleEthertype {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SecurityGroupRuleProtocol {
    Tcp,
    Udp,
    Icmp,
    All,
    Unknown(String),
}
impl SecurityGroupRuleProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Icmp => "icmp",
            Self::All => "all",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SecurityGroupRuleProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SecurityGroupRuleProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            "icmp" => Self::Icmp,
            "all" => Self::All,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateSecurityGroupRuleResponse = SecurityGroupRuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubnetCreateRequestInput {
    /// VPC UUID, CRN or exact name in the caller account.
    #[serde(rename = "vpc")]
    pub vpc: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "cidr_ipv4")]
    pub cidr_ipv4: String,
    /// Defaults to the first usable host in the CIDR
    #[serde(
        rename = "gateway_ipv4",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub gateway_ipv4: Option<crate::Nullable<String>>,
    /// Route-table UUID, nested CRN or exact name within the subnet VPC. On PATCH the owned path subnet supplies the VPC. Omission on create selects the default table; an empty reference is invalid.
    #[serde(
        rename = "route_table",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub route_table: Option<String>,
    /// Allocate a free /64 from the VPC IPv6 range. Can be enabled after creation. Every existing and new interface receives an IPv6 /96 and its first /128 automatically. NAT gateways hosted here also receive a public IPv6 address from the regional pool. Updating hosted gateways requires UpdateNATGateway permission and public IPv6 quota.
    #[serde(
        rename = "allocate_cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allocate_cidr_ipv6: Option<bool>,
    /// An aligned /64 inside the VPC IPv6 range. Can be added later; cannot replace an existing range. Mutually exclusive with allocate_cidr_ipv6.
    #[serde(
        rename = "cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub cidr_ipv6: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}
impl SubnetCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(vpc: String, name: String, cidr_ipv4: String) -> Self {
        Self {
            vpc,
            name,
            description: None,
            cidr_ipv4,
            gateway_ipv4: None,
            route_table: None,
            allocate_cidr_ipv6: None,
            cidr_ipv6: None,
            tags: None,
        }
    }
}

pub type CreateSubnetBody = SubnetCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SubnetResponse {
    #[serde(rename = "subnet", default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
}

pub type CreateSubnetResponse = SubnetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VpcCreateRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Must be private (RFC 1918): within 10.0.0.0/8, 172.16.0.0/12 or 192.168.0.0/16.
    #[serde(rename = "cidr_ipv4")]
    pub cidr_ipv4: String,
    /// Allocate a regional GUA /60. Mutually exclusive with cidr_ipv6. Existing IPv6 ranges cannot be replaced.
    #[serde(
        rename = "allocate_cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allocate_cidr_ipv6: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Optional aligned locally assigned ULA (fd00::/8), /48 through /60. May be added after VPC creation.
    #[serde(rename = "cidr_ipv6", default, skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<String>,
}
impl VpcCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, cidr_ipv4: String) -> Self {
        Self {
            name,
            description: None,
            cidr_ipv4,
            allocate_cidr_ipv6: None,
            tags: None,
            cidr_ipv6: None,
        }
    }
}

pub type CreateVpcBody = VpcCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VpcResponse {
    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<Vpc>,
}

pub type CreateVpcResponse = VpcResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DetachFloatingIpRequest {
    /// Interface UUID or nested CRN; bare names, null and empty references are rejected. Omitting the field clears the binding. Naming a NIC that is not a member is a no-op.
    #[serde(rename = "interface", default, skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
}

pub type DetachFloatingIpBody = DetachFloatingIpRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DetachFloatingIpResult {
    #[serde(
        rename = "floating_ip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ip: Option<FloatingIp>,
}

pub type DetachFloatingIpResponse = DetachFloatingIpResult;

pub type DetachInternetGatewayResponse = InternetGatewayResponse;

pub type GetEgressOnlyGatewayResponse = EgressOnlyGatewayResponse;

pub type GetEgressOnlyGatewayResource = EgressOnlyGateway;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetEgressOnlyGatewayScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetFloatingIpResponse = FloatingIpResponse;

pub type GetFloatingIpResource = FloatingIp;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetFloatingIpScope {
    #[serde(
        rename = "attached_to",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attached_to: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetInterfaceResponse = InterfaceResponse;

pub type GetInterfaceResource = Interface;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInterfaceScope {
    #[serde(rename = "subnet", default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,

    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInterfaceAddressResult {
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<InterfaceAddress>,
}

pub type GetInterfaceAddressResponse = GetInterfaceAddressResult;

pub type GetInternetGatewayResponse = InternetGatewayResponse;

pub type GetInternetGatewayResource = InternetGateway;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInternetGatewayScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetNATGatewayResponse = NATGatewayResponse;

pub type GetNATGatewayResource = NATGateway;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetNATGatewayScope {
    #[serde(rename = "subnet", default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,

    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetRouteResponse = RouteResponse;

pub type GetRouteResource = Route;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRouteScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetRouteTableResponse = RouteTableResponse;

pub type GetRouteTableResource = RouteTable;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRouteTableScope {
    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetSecurityGroupResponse = SecurityGroupResponse;

pub type GetSecurityGroupResource = SecurityGroup;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetSecurityGroupScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetSecurityGroupRuleResponse = SecurityGroupRuleResponse;

pub type GetSecurityGroupRuleResource = SecurityGroupRule;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetSecurityGroupRuleScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetSubnetResponse = SubnetResponse;

pub type GetSubnetResource = Subnet;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetSubnetScope {
    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type GetVpcResponse = VpcResponse;

pub type GetVpcResource = Vpc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetVpcScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListEgressOnlyGatewayRoutesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListEgressOnlyGatewayRoutesQuery = ListEgressOnlyGatewayRoutesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GatewayRouteListResponse {
    #[serde(rename = "routes")]
    pub routes: Vec<GatewayRoute>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GatewayRoute {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "route_table_id")]
    pub route_table_id: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "destination_cidr")]
    pub destination_cidr: String,

    #[serde(rename = "target_type")]
    pub target_type: RouteTargetType,
    /// Set when target_type=ip. Mutex with the target_*_id fields. Must be a unicast address inside this VPC's CIDR (same IP family as destination_cidr); internet egress uses target_internet_gateway_id / target_nat_gateway_id.
    #[serde(
        rename = "next_hop_ip",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub next_hop_ip: Option<crate::Nullable<String>>,
    /// Set when target_type=internet_gateway.
    #[serde(
        rename = "target_internet_gateway_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub target_internet_gateway_id: Option<crate::Nullable<String>>,
    /// Set when target_type=nat_gateway. Supports IPv4 and IPv6; IPv6 requires an IPv6-enabled hosting subnet.
    #[serde(
        rename = "target_nat_gateway_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub target_nat_gateway_id: Option<crate::Nullable<String>>,
    /// Set when target_type=egress_only_gateway (IPv6 only). Gives the subnet outbound v6 with the internet unable to initiate inbound.
    #[serde(
        rename = "target_egress_only_gateway_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub target_egress_only_gateway_id: Option<crate::Nullable<String>>,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "route_table")]
    pub route_table: RouteTableSummary,
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

pub type ListEgressOnlyGatewayRoutesResponse = GatewayRouteListResponse;

pub type ListEgressOnlyGatewayRoutesItem = GatewayRoute;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListEgressOnlyGatewaysParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListEgressOnlyGatewaysQuery = ListEgressOnlyGatewaysParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EgressOnlyGatewayListResponse {
    #[serde(rename = "egress_only_gateways")]
    pub egress_only_gateways: Vec<EgressOnlyGateway>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListEgressOnlyGatewaysResponse = EgressOnlyGatewayListResponse;

pub type ListEgressOnlyGatewaysItem = EgressOnlyGateway;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListFloatingIpsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(
        rename = "attached_to",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attached_to: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListFloatingIpsQuery = ListFloatingIpsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIpListResponse {
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<FloatingIp>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListFloatingIpsResponse = FloatingIpListResponse;

pub type ListFloatingIpsItem = FloatingIp;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInterfaceAddressesResult {
    #[serde(rename = "addresses", default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<InterfaceAddress>>,
}

pub type ListInterfaceAddressesResponse = ListInterfaceAddressesResult;

pub type ListInterfaceAddressesItem = InterfaceAddress;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInterfacePrefixesResult {
    #[serde(
        rename = "routed_prefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub routed_prefixes: Option<Vec<RoutedPrefix>>,
}

pub type ListInterfacePrefixesResponse = ListInterfacePrefixesResult;

pub type ListInterfacePrefixesItem = RoutedPrefix;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInterfaceSecurityGroupsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListInterfaceSecurityGroupsQuery = ListInterfaceSecurityGroupsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceSecurityGroupsResponse {
    #[serde(rename = "security_group_ids")]
    pub security_group_ids: Vec<String>,
}

pub type ListInterfaceSecurityGroupsResponse = InterfaceSecurityGroupsResponse;

pub type ListInterfaceSecurityGroupsItem = String;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInterfacesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "subnet", default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,

    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListInterfacesQuery = ListInterfacesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceListResponse {
    #[serde(rename = "interfaces")]
    pub interfaces: Vec<Interface>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInterfacesResponse = InterfaceListResponse;

pub type ListInterfacesItem = Interface;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInternetGatewayRoutesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListInternetGatewayRoutesQuery = ListInternetGatewayRoutesParameters;

pub type ListInternetGatewayRoutesResponse = GatewayRouteListResponse;

pub type ListInternetGatewayRoutesItem = GatewayRoute;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInternetGatewaysParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListInternetGatewaysQuery = ListInternetGatewaysParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InternetGatewayListResponse {
    #[serde(rename = "internet_gateways")]
    pub internet_gateways: Vec<InternetGateway>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInternetGatewaysResponse = InternetGatewayListResponse;

pub type ListInternetGatewaysItem = InternetGateway;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListNATGatewayRoutesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListNATGatewayRoutesQuery = ListNATGatewayRoutesParameters;

pub type ListNATGatewayRoutesResponse = GatewayRouteListResponse;

pub type ListNATGatewayRoutesItem = GatewayRoute;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListNATGatewaysParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "subnet", default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,

    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListNATGatewaysQuery = ListNATGatewaysParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NATGatewayListResponse {
    #[serde(rename = "nat_gateways")]
    pub nat_gateways: Vec<NATGateway>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListNATGatewaysResponse = NATGatewayListResponse;

pub type ListNATGatewaysItem = NATGateway;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPrefixPoolsResult {
    #[serde(
        rename = "prefix_pools",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_pools: Option<Vec<PrefixPool>>,
}

pub type ListPrefixPoolsResponse = ListPrefixPoolsResult;

pub type ListPrefixPoolsItem = PrefixPool;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRouteTablesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListRouteTablesQuery = ListRouteTablesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteTableListResponse {
    #[serde(rename = "route_tables")]
    pub route_tables: Vec<RouteTable>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListRouteTablesResponse = RouteTableListResponse;

pub type ListRouteTablesItem = RouteTable;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRoutesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListRoutesQuery = ListRoutesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RouteListResponse {
    #[serde(rename = "routes")]
    pub routes: Vec<Route>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListRoutesResponse = RouteListResponse;

pub type ListRoutesItem = Route;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSecurityGroupRulesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListSecurityGroupRulesQuery = ListSecurityGroupRulesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecurityGroupRuleListResponse {
    #[serde(rename = "rules")]
    pub rules: Vec<SecurityGroupRule>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListSecurityGroupRulesResponse = SecurityGroupRuleListResponse;

pub type ListSecurityGroupRulesItem = SecurityGroupRule;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSecurityGroupsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListSecurityGroupsQuery = ListSecurityGroupsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SecurityGroupListResponse {
    #[serde(rename = "security_groups")]
    pub security_groups: Vec<SecurityGroup>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListSecurityGroupsResponse = SecurityGroupListResponse;

pub type ListSecurityGroupsItem = SecurityGroup;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSubnetsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "vpc", default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListSubnetsQuery = ListSubnetsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubnetListResponse {
    #[serde(rename = "subnets")]
    pub subnets: Vec<Subnet>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListSubnetsResponse = SubnetListResponse;

pub type ListSubnetsItem = Subnet;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListVpcsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListVpcsQuery = ListVpcsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VpcListResponse {
    #[serde(rename = "vpcs")]
    pub vpcs: Vec<Vpc>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListVpcsResponse = VpcListResponse;

pub type ListVpcsItem = Vpc;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InterfaceSecurityGroupsRequestInput {
    /// Security-group UUIDs, CRNs or account-scoped names. All entries resolve before replacement; duplicate canonical IDs collapse to one membership. An empty array removes all groups.
    #[serde(rename = "security_groups")]
    pub security_groups: Vec<String>,
}
impl InterfaceSecurityGroupsRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(security_groups: Vec<String>) -> Self {
        Self { security_groups }
    }
}

pub type SetInterfaceSecurityGroupsBody = InterfaceSecurityGroupsRequestInput;

pub type SetInterfaceSecurityGroupsResponse = InterfaceSecurityGroupsResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct EgressOnlyGatewayUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateEgressOnlyGatewayBody = EgressOnlyGatewayUpdateRequestInput;

pub type UpdateEgressOnlyGatewayResponse = EgressOnlyGatewayResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct FloatingIpUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Set (an object) or clear (null) the address's readiness check. Omit the field to leave it unchanged.
    #[serde(
        rename = "health_check",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub health_check: Option<crate::Nullable<FloatingIpHealthCheckInput>>,
}

pub type UpdateFloatingIpBody = FloatingIpUpdateRequestInput;

pub type UpdateFloatingIpResponse = FloatingIpResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InterfaceUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateInterfaceBody = InterfaceUpdateRequestInput;

pub type UpdateInterfaceResponse = InterfaceResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InternetGatewayUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateInternetGatewayBody = InternetGatewayUpdateRequestInput;

pub type UpdateInternetGatewayResponse = InternetGatewayResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct NATGatewayUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateNATGatewayBody = NATGatewayUpdateRequestInput;

pub type UpdateNATGatewayResponse = NATGatewayResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RouteUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateRouteBody = RouteUpdateRequestInput;

pub type UpdateRouteResponse = RouteResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RouteTableUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateRouteTableBody = RouteTableUpdateRequestInput;

pub type UpdateRouteTableResponse = RouteTableResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SecurityGroupUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateSecurityGroupBody = SecurityGroupUpdateRequestInput;

pub type UpdateSecurityGroupResponse = SecurityGroupResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SubnetUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,
    /// Route-table UUID, nested CRN or exact name within the subnet VPC. On PATCH the owned path subnet supplies the VPC. Omission on create selects the default table; an empty reference is invalid.
    #[serde(
        rename = "route_table",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub route_table: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Allocate a free /64 from the VPC IPv6 range. Can be enabled after creation. Every existing and new interface receives an IPv6 /96 and its first /128 automatically. NAT gateways hosted here also receive a public IPv6 address from the regional pool. Updating hosted gateways requires UpdateNATGateway permission and public IPv6 quota.
    #[serde(
        rename = "allocate_cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allocate_cidr_ipv6: Option<bool>,
    /// An aligned /64 inside the VPC IPv6 range. Can be added later; cannot replace an existing range. Mutually exclusive with allocate_cidr_ipv6.
    #[serde(
        rename = "cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub cidr_ipv6: Option<crate::Nullable<String>>,
    /// When enabling IPv6, copy equivalent rules in security groups used by this subnet's interfaces. Copies 0.0.0.0/0 to ::/0 and security-group references, preserving protocol, ports and direction. Restricted IPv4 CIDRs are not widened. Existing IPv6 equivalents are not duplicated. Changes affect every interface sharing these groups. Requires CreateSecurityGroupRule permission and available rule quota. Only accepted with allocate_cidr_ipv6 or cidr_ipv6.
    #[serde(
        rename = "copy_ipv4_security_rules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub copy_ipv4_security_rules: Option<bool>,
    /// When enabling IPv6, optionally add ::/0 to the subnet's route table. match_ipv4 follows an IPv4 internet-gateway or NAT-gateway default route, using the same target. A NAT gateway must already have IPv6 enabled on its hosting subnet, or be hosted in the subnet being enabled. No IPv4 default route leaves IPv6 routing unchanged. Existing IPv6 default routes are always preserved. Egress-only gateways cannot provide ULA internet access; use a NAT gateway, or a public IPv6 floating IP with an internet-gateway route. Changes affect every subnet sharing the route table and require CreateRoute permission; creating an egress-only gateway also requires CreateEgressOnlyGateway permission. Only accepted with allocate_cidr_ipv6 or cidr_ipv6.
    #[serde(
        rename = "ipv6_routing",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ipv6_routing: Option<SubnetUpdateRequestInputIpv6Routing>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubnetUpdateRequestInputIpv6Routing {
    MatchIpv4,
    Unchanged,
    InternetGateway,
    NatGateway,
    EgressOnlyGateway,
    Unknown(String),
}
impl SubnetUpdateRequestInputIpv6Routing {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MatchIpv4 => "match_ipv4",
            Self::Unchanged => "unchanged",
            Self::InternetGateway => "internet_gateway",
            Self::NatGateway => "nat_gateway",
            Self::EgressOnlyGateway => "egress_only_gateway",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SubnetUpdateRequestInputIpv6Routing {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubnetUpdateRequestInputIpv6Routing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "match_ipv4" => Self::MatchIpv4,
            "unchanged" => Self::Unchanged,
            "internet_gateway" => Self::InternetGateway,
            "nat_gateway" => Self::NatGateway,
            "egress_only_gateway" => Self::EgressOnlyGateway,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateSubnetBody = SubnetUpdateRequestInput;

pub type UpdateSubnetResponse = SubnetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VpcUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub description: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::BTreeMap<String, String>>,
    /// Allocate a regional GUA /60. Mutually exclusive with cidr_ipv6. Existing IPv6 ranges cannot be replaced.
    #[serde(
        rename = "allocate_cidr_ipv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allocate_cidr_ipv6: Option<bool>,
    /// Optional aligned locally assigned ULA (fd00::/8), /48 through /60. May be added after VPC creation.
    #[serde(rename = "cidr_ipv6", default, skip_serializing_if = "Option::is_none")]
    pub cidr_ipv6: Option<String>,
}

pub type UpdateVpcBody = VpcUpdateRequestInput;

pub type UpdateVpcResponse = VpcResponse;
