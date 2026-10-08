//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachListenerCertificateRequestInput {
    /// The certificate to serve, by CRN, UUID or exact account-scoped name. Certificate CRNs require an empty region. The listener stores a reference — no key material is sent here, and the replicas fetch it from the certificate service under their own identity.
    #[serde(rename = "certificate")]
    pub certificate: String,
    /// When true, demote whatever's currently default and promote this cert in the same transaction.
    #[serde(
        rename = "is_default",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_default: Option<bool>,
}
impl AttachListenerCertificateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(certificate: String) -> Self {
        Self {
            certificate,
            is_default: None,
        }
    }
}

pub type AttachListenerCertificateBody = AttachListenerCertificateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListenerResponse {
    #[serde(rename = "listener", default, skip_serializing_if = "Option::is_none")]
    pub listener: Option<Listener>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Listener {
    /// Parent-scoped CRN with immutable load balancer name and child UUID components.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "load_balancer_id")]
    pub load_balancer_id: String,

    #[serde(rename = "protocol")]
    pub protocol: ListenerProtocol,

    #[serde(rename = "port")]
    pub port: i64,
    /// HTTPS listeners only. One entry per attached certificate; the right cert is picked per-connection by matching the client's SNI against each cert's SAN. The entry flagged is_default serves traffic that doesn't match any other SNI (or clients that omit SNI). PEM material is NOT echoed — the listener stores its own copy fetched at attach time.
    #[serde(
        rename = "certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificates: Option<Vec<ListenerCertificate>>,

    #[serde(
        rename = "default_target_group_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_target_group_id: Option<String>,
    /// Which LB addresses this listener binds. 'public_only' and 'both' require the LB to carry a floating IP; if the FIP is detached later, the listener is dropped until a FIP is re-attached or exposure is flipped to private_only.
    #[serde(rename = "exposure")]
    pub exposure: ListenerExposure,

    #[serde(rename = "tags")]
    pub tags: Tags,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListenerProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Unknown(String),
}
impl ListenerProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListenerProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListenerCertificate {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "certificate_crn")]
    pub certificate_crn: String,

    #[serde(rename = "is_default")]
    pub is_default: bool,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListenerExposure {
    PublicOnly,
    PrivateOnly,
    Both,
    Unknown(String),
}
impl ListenerExposure {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PublicOnly => "public_only",
            Self::PrivateOnly => "private_only",
            Self::Both => "both",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListenerExposure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "public_only" => Self::PublicOnly,
            "private_only" => Self::PrivateOnly,
            "both" => Self::Both,
            _ => Self::Unknown(value),
        })
    }
}

pub type Tags = std::collections::BTreeMap<String, String>;

pub type AttachListenerCertificateResponse = ListenerResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachTargetRequestInput {
    /// Must match the group's target_type: an IP address for `ip`, a compute instance UUID, CRN or exact account-scoped name for `instance`. An `ip` ref has to be a routable unicast address — loopback, link-local (including the 169.254.169.254 metadata endpoint), multicast, and unspecified addresses are rejected.
    #[serde(rename = "target")]
    pub target: String,

    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}
impl AttachTargetRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(target: String) -> Self {
        Self { target, port: None }
    }
}

pub type AttachTargetBody = AttachTargetRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TargetResponse {
    #[serde(rename = "target", default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Target {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "target_group_id")]
    pub target_group_id: String,
    /// IP address (target_type=ip) or compute instance id (target_type=instance). Stored in canonical form, so the spelling here may differ from the one you sent.
    #[serde(rename = "target_ref")]
    pub target_ref: String,

    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    #[serde(rename = "health")]
    pub health: TargetHealth,

    #[serde(
        rename = "last_seen_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_seen_at: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetHealth {
    Initial,
    Healthy,
    Unhealthy,
    Draining,
    Unknown(String),
}
impl TargetHealth {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Initial => "initial",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::Draining => "draining",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TargetHealth {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TargetHealth {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "initial" => Self::Initial,
            "healthy" => Self::Healthy,
            "unhealthy" => Self::Unhealthy,
            "draining" => Self::Draining,
            _ => Self::Unknown(value),
        })
    }
}

pub type AttachTargetResponse = TargetResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateListenerRequestInput {
    #[serde(rename = "protocol")]
    pub protocol: CreateListenerRequestInputProtocol,

    #[serde(rename = "port")]
    pub port: i64,

    #[serde(
        rename = "certificates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificates: Option<Vec<CreateListenerCertificateInput>>,

    #[serde(
        rename = "default_target_group",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_target_group: Option<String>,
    /// Which LB addresses are bound. Defaults to 'both'; pick private_only when the LB has no FIP yet.
    #[serde(rename = "exposure", default, skip_serializing_if = "Option::is_none")]
    pub exposure: Option<CreateListenerRequestInputExposure>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl CreateListenerRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(protocol: CreateListenerRequestInputProtocol, port: i64) -> Self {
        Self {
            protocol,
            port,
            certificates: None,
            default_target_group: None,
            exposure: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateListenerRequestInputProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Unknown(String),
}
impl CreateListenerRequestInputProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateListenerRequestInputProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateListenerRequestInputProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateListenerCertificateInput {
    /// Certificate CRN, UUID or exact name in the caller's account. Certificate CRNs require an empty region. No key material is accepted.
    #[serde(rename = "certificate")]
    pub certificate: String,
}
impl CreateListenerCertificateInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(certificate: String) -> Self {
        Self { certificate }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateListenerRequestInputExposure {
    PublicOnly,
    PrivateOnly,
    Both,
    Unknown(String),
}
impl CreateListenerRequestInputExposure {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PublicOnly => "public_only",
            Self::PrivateOnly => "private_only",
            Self::Both => "both",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateListenerRequestInputExposure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateListenerRequestInputExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "public_only" => Self::PublicOnly,
            "private_only" => Self::PrivateOnly,
            "both" => Self::Both,
            _ => Self::Unknown(value),
        })
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateListenerBody = CreateListenerRequestInput;

pub type CreateListenerResponse = ListenerResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateLoadBalancerRequestInput {
    /// 1..127 chars of \[A-Za-z0-9._-\] Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub type_: CreateLoadBalancerRequestInputType,
    /// VPC the LB will live in. Must match subnet's VPC.
    #[serde(rename = "vpc")]
    pub vpc: String,
    /// Subnet the LB instances attach to. The virtual IP is allocated from this subnet.
    #[serde(rename = "subnet")]
    pub subnet: String,
    /// Compute flavor for each LB instance.
    #[serde(rename = "flavor")]
    pub flavor: String,
    /// Number of LB compute instances. Defaults to 1; pick &gt;=2 for HA.
    #[serde(
        rename = "replica_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replica_count: Option<i64>,
    /// Public IPv4 shorthand. Cannot be combined with floating_ips. Does not allocate public IPv6.
    #[serde(
        rename = "floating_ip",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ip: Option<String>,
    /// Existing free floating IPs from this account and region, at most one per family and visibility (private/public, IPv4/IPv6). Private addresses must belong to the selected subnet. Missing private families are allocated automatically for each family enabled on that subnet. Public addresses are optional and require a matching-family default route to an internet gateway; NAT and egress-only gateways do not qualify. IPv6 requires an IPv6-enabled subnet. Pool-owned or attached addresses are unavailable. On deletion, supplied addresses are detached and retained; automatic private allocations are released. Cannot be combined with floating_ip.
    #[serde(
        rename = "floating_ips",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ips: Option<Vec<String>>,
    /// Security groups attached to every replica NIC (AWS ALB shape). A VPC NIC with no security group denies all data traffic, so the listener port(s) must be opened by a security group listed here. Re-applied to replacement replicas. The LB's own control-plane path (agent config + heartbeat via the metadata endpoint) is always-allowed and needs none.
    #[serde(rename = "security_groups")]
    pub security_groups: Vec<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl CreateLoadBalancerRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        type_: CreateLoadBalancerRequestInputType,
        vpc: String,
        subnet: String,
        flavor: String,
        security_groups: Vec<String>,
    ) -> Self {
        Self {
            name,
            type_,
            vpc,
            subnet,
            flavor,
            replica_count: None,
            floating_ip: None,
            floating_ips: None,
            security_groups,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateLoadBalancerRequestInputType {
    Application,
    Network,
    Unknown(String),
}
impl CreateLoadBalancerRequestInputType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Application => "application",
            Self::Network => "network",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateLoadBalancerRequestInputType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateLoadBalancerRequestInputType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "application" => Self::Application,
            "network" => Self::Network,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateLoadBalancerBody = CreateLoadBalancerRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LoadBalancerResponse {
    #[serde(
        rename = "load_balancer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancer: Option<LoadBalancer>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancer {
    #[serde(rename = "id")]
    pub id: String,
    /// IAM resource CRN
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// ALB-shape (L7) vs NLB-shape (L4)
    #[serde(rename = "type")]
    pub type_: LoadBalancerType,

    #[serde(rename = "status")]
    pub status: LoadBalancerStatus,
    /// Active faults; status is error exactly when an active error fault remains.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,
    /// Subnet placement; null when the referenced subnet no longer exists.
    #[serde(rename = "subnet")]
    pub subnet: LoadBalancerSubnet,
    /// Compute flavor each LB instance runs on. Must be a loadbalancer-family flavor.
    #[serde(rename = "flavor_id")]
    pub flavor_id: String,
    /// Number of LB compute instances. &gt;=2 for HA.
    #[serde(rename = "replica_count")]
    pub replica_count: i64,
    /// Virtual IP for the load balancer; traffic is distributed to backends per connection.
    #[serde(
        rename = "internal_ipv4",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_ipv4: Option<String>,
    /// Internal IPv6 VIP (set when the subnet is dual-stack).
    #[serde(
        rename = "internal_ipv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_ipv6: Option<String>,
    /// Explicitly selected public IPv6 floating IP, translated to replica IPv6 addresses in a GUA or ULA subnet.
    #[serde(
        rename = "public_ipv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_ipv6: Option<String>,
    /// Optional public IPv4 floating IP. Public IPv6 is independent.
    #[serde(
        rename = "floating_ip_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ip_id: Option<String>,
    /// All attached public and private floating IPs, including automatic private allocations.
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<FloatingIp>,
    /// Convenience hostname auto-published for the load balancer, `{name}.{account-handle}.lb.{region}.{base-domain}`. Resolves to the floating IP on an internet-facing LB and to the private VIP otherwise. Omitted in regions where auto-DNS is not configured — the VIP and FIP stay authoritative either way.
    #[serde(rename = "dns_name", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Tags,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadBalancerType {
    Application,
    Network,
    Unknown(String),
}
impl LoadBalancerType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Application => "application",
            Self::Network => "network",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LoadBalancerType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LoadBalancerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "application" => Self::Application,
            "network" => Self::Network,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadBalancerStatus {
    Provisioning,
    Active,
    Error,
    Deleting,
    Unknown(String),
}
impl LoadBalancerStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Provisioning => "provisioning",
            Self::Active => "active",
            Self::Error => "error",
            Self::Deleting => "deleting",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LoadBalancerStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LoadBalancerStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "provisioning" => Self::Provisioning,
            "active" => Self::Active,
            "error" => Self::Error,
            "deleting" => Self::Deleting,
            _ => Self::Unknown(value),
        })
    }
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LoadBalancerSubnet {
    Variant1(Box<Subnet>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
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

pub type CreateLoadBalancerResponse = LoadBalancerResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateRuleRequestInput {
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "conditions")]
    pub conditions: Vec<RuleConditionInput>,

    #[serde(rename = "target_group")]
    pub target_group: String,
}
impl CreateRuleRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(priority: i64, conditions: Vec<RuleConditionInput>, target_group: String) -> Self {
        Self {
            priority,
            conditions,
            target_group,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RuleConditionInput {
    #[serde(rename = "field")]
    pub field: RuleConditionInputField,

    #[serde(rename = "op")]
    pub op: RuleConditionInputOp,
    /// header or query key name
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "values")]
    pub values: Vec<String>,
}
impl RuleConditionInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        field: RuleConditionInputField,
        op: RuleConditionInputOp,
        values: Vec<String>,
    ) -> Self {
        Self {
            field,
            op,
            name: None,
            values,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleConditionInputField {
    Host,
    Path,
    Header,
    Query,
    Method,
    Unknown(String),
}
impl RuleConditionInputField {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Host => "host",
            Self::Path => "path",
            Self::Header => "header",
            Self::Query => "query",
            Self::Method => "method",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RuleConditionInputField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RuleConditionInputField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "host" => Self::Host,
            "path" => Self::Path,
            "header" => Self::Header,
            "query" => Self::Query,
            "method" => Self::Method,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleConditionInputOp {
    Exact,
    Prefix,
    Glob,
    Regex,
    Unknown(String),
}
impl RuleConditionInputOp {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
            Self::Prefix => "prefix",
            Self::Glob => "glob",
            Self::Regex => "regex",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RuleConditionInputOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RuleConditionInputOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "exact" => Self::Exact,
            "prefix" => Self::Prefix,
            "glob" => Self::Glob,
            "regex" => Self::Regex,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateRuleBody = CreateRuleRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RuleResponse {
    #[serde(rename = "rule", default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Rule {
    /// Parent-scoped CRN with immutable load balancer name and child UUID components.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "listener_id")]
    pub listener_id: String,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "conditions")]
    pub conditions: Vec<RuleCondition>,

    #[serde(rename = "target_group_id")]
    pub target_group_id: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RuleCondition {
    #[serde(rename = "field")]
    pub field: RuleConditionField,

    #[serde(rename = "op")]
    pub op: RuleConditionOp,
    /// header or query key name
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "values")]
    pub values: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleConditionField {
    Host,
    Path,
    Header,
    Query,
    Method,
    Unknown(String),
}
impl RuleConditionField {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Host => "host",
            Self::Path => "path",
            Self::Header => "header",
            Self::Query => "query",
            Self::Method => "method",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RuleConditionField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RuleConditionField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "host" => Self::Host,
            "path" => Self::Path,
            "header" => Self::Header,
            "query" => Self::Query,
            "method" => Self::Method,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleConditionOp {
    Exact,
    Prefix,
    Glob,
    Regex,
    Unknown(String),
}
impl RuleConditionOp {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
            Self::Prefix => "prefix",
            Self::Glob => "glob",
            Self::Regex => "regex",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RuleConditionOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RuleConditionOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "exact" => Self::Exact,
            "prefix" => Self::Prefix,
            "glob" => Self::Glob,
            "regex" => Self::Regex,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateRuleResponse = RuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTargetGroupRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "protocol")]
    pub protocol: CreateTargetGroupRequestInputProtocol,

    #[serde(
        rename = "target_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_type: Option<CreateTargetGroupRequestInputTargetType>,

    #[serde(rename = "port")]
    pub port: i64,

    #[serde(
        rename = "health_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub health_check: Option<HealthCheckInput>,

    #[serde(
        rename = "proxy_protocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_protocol: Option<bool>,

    #[serde(
        rename = "session_affinity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_affinity: Option<SessionAffinityInput>,
    /// `static` (the default) takes the backends you attach as targets. `pool` takes them from a compute instance pool and requires instance_pool; the group is forced to target_type=instance, and attaching targets to it is rejected.
    #[serde(
        rename = "target_mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_mode: Option<CreateTargetGroupRequestInputTargetMode>,
    /// Compute instance pool to draw backends from. Required when target_mode=pool and must belong to the calling account; ignored otherwise.
    #[serde(
        rename = "instance_pool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_pool: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl CreateTargetGroupRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, protocol: CreateTargetGroupRequestInputProtocol, port: i64) -> Self {
        Self {
            name,
            protocol,
            target_type: None,
            port,
            health_check: None,
            proxy_protocol: None,
            session_affinity: None,
            target_mode: None,
            instance_pool: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTargetGroupRequestInputProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Unknown(String),
}
impl CreateTargetGroupRequestInputProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateTargetGroupRequestInputProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateTargetGroupRequestInputProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTargetGroupRequestInputTargetType {
    Ip,
    Instance,
    Function,
    Unknown(String),
}
impl CreateTargetGroupRequestInputTargetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ip => "ip",
            Self::Instance => "instance",
            Self::Function => "function",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateTargetGroupRequestInputTargetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateTargetGroupRequestInputTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ip" => Self::Ip,
            "instance" => Self::Instance,
            "function" => Self::Function,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct HealthCheckInput {
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Omitted or empty uses the target group protocol. UDP uses a TCP connect probe. HTTPS probes use TLS.
    #[serde(rename = "protocol", default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<HealthCheckInputProtocol>,

    #[serde(rename = "path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "interval_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_sec: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "timeout_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_sec: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "healthy_threshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub healthy_threshold: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "unhealthy_threshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unhealthy_threshold: Option<i64>,
    /// HTTP status codes from 100 to 599; comma-separated codes or inclusive ranges. Empty uses 200.
    #[serde(rename = "matcher", default, skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HealthCheckInputProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Value,
    Unknown(String),
}
impl HealthCheckInputProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Value => "",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for HealthCheckInputProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckInputProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            "" => Self::Value,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionAffinityInput {
    /// `none` balances every request. `cookie` sets an opaque cookie on the first response and routes every later request carrying it to the same backend — http and https groups only. `source_ip` hashes the client address, works on every protocol and is the only option for tcp and udp, but a NAT gateway makes every client behind it a single key.
    #[serde(rename = "type")]
    pub type_: SessionAffinityInputType,
    /// Cookie the load balancer sets and hashes. `type=cookie` only; the field is dropped for the other types.
    #[serde(
        rename = "cookie_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_name: Option<String>,
    /// How long that cookie lives, up to 7 days. `type=cookie` only.
    #[serde(
        rename = "duration_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_sec: Option<i64>,
}
impl SessionAffinityInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(type_: SessionAffinityInputType) -> Self {
        Self {
            type_,
            cookie_name: None,
            duration_sec: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionAffinityInputType {
    None,
    Cookie,
    SourceIp,
    Unknown(String),
}
impl SessionAffinityInputType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Cookie => "cookie",
            Self::SourceIp => "source_ip",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SessionAffinityInputType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionAffinityInputType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "none" => Self::None,
            "cookie" => Self::Cookie,
            "source_ip" => Self::SourceIp,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateTargetGroupRequestInputTargetMode {
    Static,
    Pool,
    Unknown(String),
}
impl CreateTargetGroupRequestInputTargetMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Static => "static",
            Self::Pool => "pool",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreateTargetGroupRequestInputTargetMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreateTargetGroupRequestInputTargetMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "static" => Self::Static,
            "pool" => Self::Pool,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateTargetGroupBody = CreateTargetGroupRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TargetGroupResponse {
    #[serde(
        rename = "target_group",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_group: Option<TargetGroup>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetGroup {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "protocol")]
    pub protocol: TargetGroupProtocol,

    #[serde(rename = "target_type")]
    pub target_type: TargetGroupTargetType,

    #[serde(rename = "port")]
    pub port: i64,

    #[serde(rename = "health_check")]
    pub health_check: HealthCheck,
    /// When true, upstream connections are wrapped in the PROXY v2 header so backends see the original client IP + port. HTTP backends already get X-Forwarded-For; PROXY is the right pick for TCP/UDP target groups or HTTP backends that prefer the framed envelope.
    #[serde(rename = "proxy_protocol")]
    pub proxy_protocol: bool,

    #[serde(rename = "session_affinity")]
    pub session_affinity: SessionAffinity,
    /// Where the backend set comes from. `static` routes to the targets attached via POST /v1/target-groups/{id}/targets; `pool` resolves live instance addresses from the compute instance pool named by instance_pool_id, so scaling the pool moves the backends with it.
    #[serde(rename = "target_mode")]
    pub target_mode: TargetGroupTargetMode,
    /// Compute instance pool backing the group. Set iff target_mode=pool.
    #[serde(
        rename = "instance_pool_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_pool_id: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Tags,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetGroupProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Unknown(String),
}
impl TargetGroupProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TargetGroupProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TargetGroupProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetGroupTargetType {
    Ip,
    Instance,
    Function,
    Unknown(String),
}
impl TargetGroupTargetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ip => "ip",
            Self::Instance => "instance",
            Self::Function => "function",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TargetGroupTargetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TargetGroupTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ip" => Self::Ip,
            "instance" => Self::Instance,
            "function" => Self::Function,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct HealthCheck {
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Omitted or empty uses the target group protocol. UDP uses a TCP connect probe. HTTPS probes use TLS.
    #[serde(rename = "protocol", default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<HealthCheckProtocol>,

    #[serde(rename = "path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "interval_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_sec: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "timeout_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_sec: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "healthy_threshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub healthy_threshold: Option<i64>,
    /// Zero uses the default.
    #[serde(
        rename = "unhealthy_threshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unhealthy_threshold: Option<i64>,
    /// HTTP status codes from 100 to 599; comma-separated codes or inclusive ranges. Empty uses 200.
    #[serde(rename = "matcher", default, skip_serializing_if = "Option::is_none")]
    pub matcher: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HealthCheckProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Value,
    Unknown(String),
}
impl HealthCheckProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Value => "",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for HealthCheckProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HealthCheckProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            "" => Self::Value,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionAffinity {
    /// `none` balances every request. `cookie` sets an opaque cookie on the first response and routes every later request carrying it to the same backend — http and https groups only. `source_ip` hashes the client address, works on every protocol and is the only option for tcp and udp, but a NAT gateway makes every client behind it a single key.
    #[serde(rename = "type")]
    pub type_: SessionAffinityType,
    /// Cookie the load balancer sets and hashes. `type=cookie` only; the field is dropped for the other types.
    #[serde(
        rename = "cookie_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cookie_name: Option<String>,
    /// How long that cookie lives, up to 7 days. `type=cookie` only.
    #[serde(
        rename = "duration_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_sec: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionAffinityType {
    None,
    Cookie,
    SourceIp,
    Unknown(String),
}
impl SessionAffinityType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Cookie => "cookie",
            Self::SourceIp => "source_ip",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SessionAffinityType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionAffinityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "none" => Self::None,
            "cookie" => Self::Cookie,
            "source_ip" => Self::SourceIp,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TargetGroupTargetMode {
    Static,
    Pool,
    Unknown(String),
}
impl TargetGroupTargetMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Static => "static",
            Self::Pool => "pool",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TargetGroupTargetMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TargetGroupTargetMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "static" => Self::Static,
            "pool" => Self::Pool,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateTargetGroupResponse = TargetGroupResponse;

pub type GetListenerResponse = ListenerResponse;

pub type GetListenerResource = Listener;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetListenerScope {}

pub type GetLoadBalancerResponse = LoadBalancerResponse;

pub type GetLoadBalancerResource = LoadBalancer;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetLoadBalancerScope {
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetLoadBalancerScopeStatus>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetLoadBalancerScopeStatus {
    Provisioning,
    Active,
    Error,
    Deleting,
    Unknown(String),
}
impl GetLoadBalancerScopeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Provisioning => "provisioning",
            Self::Active => "active",
            Self::Error => "error",
            Self::Deleting => "deleting",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetLoadBalancerScopeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetLoadBalancerScopeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "provisioning" => Self::Provisioning,
            "active" => Self::Active,
            "error" => Self::Error,
            "deleting" => Self::Deleting,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetRuleResponse = RuleResponse;

pub type GetRuleResource = Rule;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetRuleScope {}

pub type GetTargetResponse = TargetResponse;

pub type GetTargetResource = Target;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetTargetScope {}

pub type GetTargetGroupResponse = TargetGroupResponse;

pub type GetTargetGroupResource = TargetGroup;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetTargetGroupScope {
    #[serde(rename = "protocol", default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<GetTargetGroupScopeProtocol>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetTargetGroupScopeProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Unknown(String),
}
impl GetTargetGroupScopeProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetTargetGroupScopeProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetTargetGroupScopeProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListListenersParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListListenersQuery = ListListenersParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListenerListResponse {
    #[serde(rename = "listeners", default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<Listener>>,
}

pub type ListListenersResponse = ListenerListResponse;

pub type ListListenersItem = Listener;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListLoadBalancerReplicasParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListLoadBalancerReplicasQuery = ListLoadBalancerReplicasParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LoadBalancerReplicasResponse {
    #[serde(rename = "replicas", default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<Vec<LoadBalancerReplica>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancerReplica {
    #[serde(rename = "instance_id")]
    pub instance_id: String,

    #[serde(rename = "replica_index")]
    pub replica_index: i64,

    #[serde(rename = "created_at")]
    pub created_at: String,
    /// The size this replica actually booted on. Matches the load balancer's flavor_id except mid-resize, when the replicas not yet replaced still report the old one.
    #[serde(rename = "flavor_id")]
    pub flavor_id: String,
    /// The liveness view folded into one word: 'initializing' (the agent has never reported — boot still in flight), 'healthy' (heartbeating and the proxy is serving), 'unhealthy' (heartbeating but the proxy is down).
    #[serde(rename = "status")]
    pub status: LoadBalancerReplicaStatus,
    /// Whether the proxy reported ready at the last health report
    #[serde(rename = "proxy_ok")]
    pub proxy_ok: bool,

    #[serde(
        rename = "agent_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_version: Option<String>,
    /// Omitted when this replica hasn't reported yet
    #[serde(rename = "last_seen", default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadBalancerReplicaStatus {
    Initializing,
    Healthy,
    Unhealthy,
    Unknown(String),
}
impl LoadBalancerReplicaStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Initializing => "initializing",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LoadBalancerReplicaStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LoadBalancerReplicaStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "initializing" => Self::Initializing,
            "healthy" => Self::Healthy,
            "unhealthy" => Self::Unhealthy,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListLoadBalancerReplicasResponse = LoadBalancerReplicasResponse;

pub type ListLoadBalancerReplicasItem = LoadBalancerReplica;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListLoadBalancersParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ListLoadBalancersParametersStatus>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListLoadBalancersParametersStatus {
    Provisioning,
    Active,
    Error,
    Deleting,
    Unknown(String),
}
impl ListLoadBalancersParametersStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Provisioning => "provisioning",
            Self::Active => "active",
            Self::Error => "error",
            Self::Deleting => "deleting",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListLoadBalancersParametersStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListLoadBalancersParametersStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "provisioning" => Self::Provisioning,
            "active" => Self::Active,
            "error" => Self::Error,
            "deleting" => Self::Deleting,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListLoadBalancersQuery = ListLoadBalancersParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LoadBalancerListResponse {
    #[serde(
        rename = "load_balancers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub load_balancers: Option<Vec<LoadBalancer>>,

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

pub type ListLoadBalancersResponse = LoadBalancerListResponse;

pub type ListLoadBalancersItem = LoadBalancer;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListRulesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListRulesQuery = ListRulesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct RuleListResponse {
    #[serde(rename = "rules", default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

pub type ListRulesResponse = RuleListResponse;

pub type ListRulesItem = Rule;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListTargetGroupsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "protocol", default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ListTargetGroupsParametersProtocol>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListTargetGroupsParametersProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Unknown(String),
}
impl ListTargetGroupsParametersProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListTargetGroupsParametersProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListTargetGroupsParametersProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListTargetGroupsQuery = ListTargetGroupsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TargetGroupListResponse {
    #[serde(
        rename = "target_groups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_groups: Option<Vec<TargetGroup>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListTargetGroupsResponse = TargetGroupListResponse;

pub type ListTargetGroupsItem = TargetGroup;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListTargetsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListTargetsQuery = ListTargetsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct TargetListResponse {
    #[serde(rename = "targets", default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

pub type ListTargetsResponse = TargetListResponse;

pub type ListTargetsItem = Target;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateListenerRequestInput {
    #[serde(
        rename = "certificate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate: Option<String>,

    #[serde(
        rename = "default_target_group",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_target_group: Option<String>,

    #[serde(
        rename = "clear_default_target_group",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub clear_default_target_group: Option<bool>,
    /// Mutate which addresses are bound. Omit to leave unchanged.
    #[serde(rename = "exposure", default, skip_serializing_if = "Option::is_none")]
    pub exposure: Option<UpdateListenerRequestInputExposure>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateListenerRequestInputExposure {
    PublicOnly,
    PrivateOnly,
    Both,
    Unknown(String),
}
impl UpdateListenerRequestInputExposure {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PublicOnly => "public_only",
            Self::PrivateOnly => "private_only",
            Self::Both => "both",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for UpdateListenerRequestInputExposure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UpdateListenerRequestInputExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "public_only" => Self::PublicOnly,
            "private_only" => Self::PrivateOnly,
            "both" => Self::Both,
            _ => Self::Unknown(value),
        })
    }
}

pub type UpdateListenerBody = UpdateListenerRequestInput;

pub type UpdateListenerResponse = ListenerResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateLoadBalancerRequestInput {
    /// Resize the set of load balancer instances. Scale-out provisions the new replicas in sequence; scale-in removes the highest-indexed replicas best-effort. 1..10.
    #[serde(
        rename = "replica_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replica_count: Option<i64>,
    /// Resize each replica to a different compute flavor. Must be a loadbalancer-family flavor. A running instance cannot change size in place, so the request records the new size and returns; the replicas already up are then replaced one at a time in the background. The load balancer temporarily runs one replica over replica_count while it does: the extra replica comes up on the new flavor and starts serving before any replica on the old one is retired, so the number serving never drops below replica_count — a resize does not cost you capacity, at any replica count. Expect it to take several minutes, and poll GET /v1/load-balancers/{id}/replicas to watch: a replica has been replaced when its instance_id changes, and the resize is done when every flavor there matches this one. The one exception is a load balancer already at the maximum of 10 replicas, which has nowhere to grow. There the replicas are replaced in place and 9 serve while each replacement boots. Rejected up front if the account does not have the compute quota for the replacement replica, so a resize cannot half-apply and leave the load balancer short.
    #[serde(rename = "flavor", default, skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateLoadBalancerBody = UpdateLoadBalancerRequestInput;

pub type UpdateLoadBalancerResponse = LoadBalancerResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateRuleRequestInput {
    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "conditions")]
    pub conditions: Vec<RuleConditionInput>,

    #[serde(rename = "target_group")]
    pub target_group: String,
}
impl UpdateRuleRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(priority: i64, conditions: Vec<RuleConditionInput>, target_group: String) -> Self {
        Self {
            priority,
            conditions,
            target_group,
        }
    }
}

pub type UpdateRuleBody = UpdateRuleRequestInput;

pub type UpdateRuleResponse = RuleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateTargetGroupRequestInput {
    #[serde(
        rename = "health_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub health_check: Option<HealthCheckPatchInput>,
    /// Toggle PROXY v2 framing on upstream connections. Omitting the field leaves the current setting; setting true/false flips it explicitly.
    #[serde(
        rename = "proxy_protocol",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_protocol: Option<bool>,
    /// Replaces the stickiness config. Omitting the field leaves it alone; turning it off is an explicit `{"type": "none"}`.
    #[serde(
        rename = "session_affinity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_affinity: Option<SessionAffinityInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct HealthCheckPatchInput {
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(rename = "protocol", default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ProtocolInput>,

    #[serde(rename = "path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<PathInput>,
    /// Zero removes the probe port override.
    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    #[serde(
        rename = "interval_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_sec: Option<IntervalSecInput>,

    #[serde(
        rename = "timeout_sec",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_sec: Option<TimeoutSecInput>,

    #[serde(
        rename = "healthy_threshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub healthy_threshold: Option<HealthyThresholdInput>,

    #[serde(
        rename = "unhealthy_threshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unhealthy_threshold: Option<UnhealthyThresholdInput>,

    #[serde(rename = "matcher", default, skip_serializing_if = "Option::is_none")]
    pub matcher: Option<MatcherInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProtocolInput {
    Http,
    Https,
    Tcp,
    Udp,
    Value,
    Unknown(String),
}
impl ProtocolInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Value => "",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ProtocolInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ProtocolInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            "tcp" => Self::Tcp,
            "udp" => Self::Udp,
            "" => Self::Value,
            _ => Self::Unknown(value),
        })
    }
}

pub type PathInput = String;

pub type IntervalSecInput = i64;

pub type TimeoutSecInput = i64;

pub type HealthyThresholdInput = i64;

pub type UnhealthyThresholdInput = i64;

pub type MatcherInput = String;

pub type UpdateTargetGroupBody = UpdateTargetGroupRequestInput;

pub type UpdateTargetGroupResponse = TargetGroupResponse;
