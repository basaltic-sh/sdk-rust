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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListenerCertificateResponse {
    #[serde(rename = "certificate")]
    pub certificate: ResourceRef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceRef {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Display label at response time — the resource's name where it has one, otherwise its human label (an invoice number, an account display name). Absent when the target is gone. Not a lookup key.
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

pub type AttachListenerCertificateResponse = ListenerCertificateResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachTargetRequestInput {
    /// Must match the group's target_type: an IP address for `ip`, a compute instance UUID, CRN or exact account-scoped name for `instance`. An `ip` ref has to be a routable unicast address — loopback, link-local (including the 169.254.169.254 metadata endpoint), multicast, and unspecified addresses are rejected.
    #[serde(rename = "target")]
    pub target: String,
    /// Per-target backend port override. When omitted, the target inherits the target group port. Pool-derived targets always inherit the group port.
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetResponse {
    #[serde(rename = "target")]
    pub target: Target,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Target {
    Variant1(Box<TargetVariant1>),
    Variant2(Box<TargetVariant2>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetVariant1 {
    #[serde(rename = "id")]
    pub id: String,
    /// Compute instance backing an instance target or a pool-derived target. Omitted for IP targets.
    #[serde(rename = "instance")]
    pub instance: ResourceRef,
    /// Canonical IPv4 or IPv6 address for an IP target. Omitted for instance targets.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Per-target backend port override. When omitted, the target inherits the target group port. Pool-derived targets always inherit the group port.
    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    #[serde(rename = "health")]
    pub health: TargetVariant1Health,

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
pub enum TargetVariant1Health {
    Initial,
    Healthy,
    Unhealthy,
    Draining,
    Unknown(String),
}
impl TargetVariant1Health {
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
impl serde::Serialize for TargetVariant1Health {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TargetVariant1Health {
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetVariant2 {
    #[serde(rename = "id")]
    pub id: String,
    /// Compute instance backing an instance target or a pool-derived target. Omitted for IP targets.
    #[serde(rename = "instance", default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<ResourceRef>,
    /// Canonical IPv4 or IPv6 address for an IP target. Omitted for instance targets.
    #[serde(rename = "address")]
    pub address: String,
    /// Per-target backend port override. When omitted, the target inherits the target group port. Pool-derived targets always inherit the group port.
    #[serde(rename = "port", default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    #[serde(rename = "health")]
    pub health: TargetVariant2Health,

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
pub enum TargetVariant2Health {
    Initial,
    Healthy,
    Unhealthy,
    Draining,
    Unknown(String),
}
impl TargetVariant2Health {
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
impl serde::Serialize for TargetVariant2Health {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TargetVariant2Health {
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
    pub certificates: Option<Vec<String>>,

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListenerResponse {
    #[serde(rename = "listener")]
    pub listener: Listener,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Listener {
    /// Parent-scoped CRN with immutable load balancer name and child UUID components.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "protocol")]
    pub protocol: ListenerProtocol,

    #[serde(rename = "port")]
    pub port: i64,
    /// Which LB addresses this listener binds. 'public_only' and 'both' require the LB to carry a floating IP; if the FIP is detached later, the listener is dropped until a FIP is re-attached or exposure is flipped to private_only.
    #[serde(rename = "exposure")]
    pub exposure: ListenerExposure,

    #[serde(rename = "tags")]
    pub tags: Tags,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Default certificate for an HTTPS listener, used when no SNI matches or the client sends none. Null when absent. List all attached certificates through the listener certificates endpoint.
    #[serde(rename = "default_certificate")]
    pub default_certificate: ListenerDefaultCertificate,
    /// Target group used when no routing rule matches, or null when none is configured.
    #[serde(rename = "default_target_group")]
    pub default_target_group: ListenerDefaultTargetGroup,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ListenerDefaultCertificate {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ListenerDefaultTargetGroup {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

pub type CreateListenerResponse = ListenerResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateLoadBalancerRequestInput {
    /// Steady target within min_count and max_count.
    #[serde(
        rename = "desired_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_count: Option<i64>,
    /// Lower capacity bound.
    #[serde(rename = "min_count", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i64>,
    /// Upper capacity bound including rollout surge.
    #[serde(rename = "max_count", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,

    #[serde(
        rename = "autoscaling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub autoscaling: Option<AutoscalingPolicyInput>,
    /// 1..127 chars of \[A-Za-z0-9._-\] Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub type_: CreateLoadBalancerRequestInputType,
    /// VPC the LB will live in. Every selected subnet must belong to this VPC.
    #[serde(rename = "vpc")]
    pub vpc: String,
    /// One or more subnet selectors in the same VPC, account and region. Each instance has one interface; new instances are placed in the least-populated eligible subnet. The first subnet supplies automatic private floating IP allocations. Placement is fixed at creation.
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
    /// Compute flavor for each LB instance.
    #[serde(rename = "flavor")]
    pub flavor: String,
    /// Deprecated input alias of desired_count; send only one. Desired defaults to 1. Omitted bounds default to desired.
    #[serde(
        rename = "replica_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replica_count: Option<i64>,
    /// Existing free floating IPs from this account and region, at most one per family and visibility (private/public, IPv4/IPv6). Private addresses must belong to any selected subnet. Missing private families are allocated automatically from the first selected subnet for each family enabled there. Public addresses are optional and require a matching-family default route to an internet gateway in every selected subnet; NAT and egress-only gateways do not qualify. Public IPv6 requires every selected subnet to support IPv6. Pool-owned or attached addresses are unavailable. On deletion, supplied addresses are detached and retained; automatic private allocations are released.
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
        subnets: Vec<String>,
        flavor: String,
        security_groups: Vec<String>,
    ) -> Self {
        Self {
            desired_count: None,
            min_count: None,
            max_count: None,
            autoscaling: None,
            name,
            type_,
            vpc,
            subnets,
            flavor,
            replica_count: None,
            floating_ips: None,
            security_groups,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutoscalingPolicyInput {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "metrics")]
    pub metrics: Vec<ScalingMetricInput>,

    #[serde(
        rename = "warmup_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warmup_seconds: Option<i64>,

    #[serde(
        rename = "cooldown_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cooldown_seconds: Option<i64>,

    #[serde(
        rename = "scale_down_stabilization_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_stabilization_seconds: Option<i64>,

    #[serde(
        rename = "max_scale_out_step",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_scale_out_step: Option<i64>,

    #[serde(
        rename = "max_scale_in_step",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_scale_in_step: Option<i64>,
    /// Grace period after route withdrawal and proxy acknowledgements, before deleting a retiring member. Long-lived TCP/UDP sessions may end at the deadline; arbitrary application shutdown hooks are not supported.
    #[serde(
        rename = "drain_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drain_seconds: Option<i64>,
}
impl AutoscalingPolicyInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(enabled: bool, metrics: Vec<ScalingMetricInput>) -> Self {
        Self {
            enabled,
            metrics,
            warmup_seconds: None,
            cooldown_seconds: None,
            scale_down_stabilization_seconds: None,
            max_scale_out_step: None,
            max_scale_in_step: None,
            drain_seconds: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ScalingMetricInput {
    #[serde(rename = "source")]
    pub source: ScalingMetricInputSource,

    #[serde(rename = "target_type")]
    pub target_type: ScalingMetricInputTargetType,

    #[serde(rename = "target_value")]
    pub target_value: f64,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Exact-match labels; tenancy labels and __name__ cannot be supplied.
    #[serde(rename = "labels", default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    /// Use last for queue gauges; rate for monotonically increasing counters, with reset handling.
    #[serde(
        rename = "sample_aggregation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_aggregation: Option<ScalingMetricInputSampleAggregation>,

    #[serde(
        rename = "series_aggregation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_aggregation: Option<ScalingMetricInputSeriesAggregation>,
    /// Exact expected cardinality; incomplete or ambiguous selectors are unavailable.
    #[serde(
        rename = "expected_series",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_series: Option<i64>,

    #[serde(
        rename = "window_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_seconds: Option<i64>,
    /// Actual newest observation age per series; must not exceed window_seconds. Defaults to the smaller of 90 and the window.
    #[serde(
        rename = "max_age_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_age_seconds: Option<i64>,
}
impl ScalingMetricInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        source: ScalingMetricInputSource,
        target_type: ScalingMetricInputTargetType,
        target_value: f64,
    ) -> Self {
        Self {
            source,
            target_type,
            target_value,
            name: None,
            labels: None,
            sample_aggregation: None,
            series_aggregation: None,
            expected_series: None,
            window_seconds: None,
            max_age_seconds: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricInputSource {
    Cpu,
    Telemetry,
    Unknown(String),
}
impl ScalingMetricInputSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cpu => "cpu",
            Self::Telemetry => "telemetry",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricInputSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricInputSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "cpu" => Self::Cpu,
            "telemetry" => Self::Telemetry,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricInputTargetType {
    Utilization,
    AverageValue,
    Unknown(String),
}
impl ScalingMetricInputTargetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Utilization => "utilization",
            Self::AverageValue => "average_value",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricInputTargetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricInputTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "utilization" => Self::Utilization,
            "average_value" => Self::AverageValue,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricInputSampleAggregation {
    Last,
    Avg,
    Max,
    Rate,
    Unknown(String),
}
impl ScalingMetricInputSampleAggregation {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Last => "last",
            Self::Avg => "avg",
            Self::Max => "max",
            Self::Rate => "rate",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricInputSampleAggregation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricInputSampleAggregation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "last" => Self::Last,
            "avg" => Self::Avg,
            "max" => Self::Max,
            "rate" => Self::Rate,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricInputSeriesAggregation {
    Sum,
    Avg,
    Max,
    Unknown(String),
}
impl ScalingMetricInputSeriesAggregation {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Sum => "sum",
            Self::Avg => "avg",
            Self::Max => "max",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricInputSeriesAggregation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricInputSeriesAggregation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "sum" => Self::Sum,
            "avg" => Self::Avg,
            "max" => Self::Max,
            _ => Self::Unknown(value),
        })
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancerResponse {
    #[serde(rename = "load_balancer")]
    pub load_balancer: LoadBalancer,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancer {
    /// Temporary extra capacity within max_count; does not change desired_count.
    #[serde(
        rename = "rollout_surge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rollout_surge: Option<bool>,
    /// Steady target within min_count and max_count.
    #[serde(rename = "desired_count")]
    pub desired_count: i64,
    /// Lower capacity bound.
    #[serde(rename = "min_count")]
    pub min_count: i64,
    /// Upper capacity bound including rollout surge.
    #[serde(rename = "max_count")]
    pub max_count: i64,

    #[serde(
        rename = "autoscaling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub autoscaling: Option<AutoscalingPolicy>,

    #[serde(
        rename = "autoscaling_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub autoscaling_status: Option<AutoscalingStatus>,

    #[serde(rename = "id")]
    pub id: String,
    /// IAM resource CRN
    #[serde(rename = "crn")]
    pub crn: String,
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
    /// Eligible subnets for member placement. Each instance has one interface, and new instances use the least-populated eligible subnet.
    #[serde(rename = "subnets")]
    pub subnets: Vec<ResourceRef>,
    /// Deprecated alias of desired_count.
    #[serde(rename = "replica_count")]
    pub replica_count: i64,
    /// Public and private floating IP references, including automatic private allocations. Each includes id, crn, visibility, and address. Infer IPv4 or IPv6 from the address.
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<FloatingIPRef>,
    /// Convenience hostname auto-published for the load balancer, `{name}.{account-handle}.lb.{region}.{base-domain}`. Resolves to the floating IP on an internet-facing LB and to the private VIP otherwise. Omitted in regions where auto-DNS is not configured — the VIP and FIP stay authoritative either way.
    #[serde(rename = "dns_name", default, skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,

    #[serde(rename = "tags")]
    pub tags: Tags,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Regional loadbalancer-family compute flavor assigned to replicas.
    #[serde(rename = "flavor")]
    pub flavor: ResourceRef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutoscalingPolicy {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "metrics")]
    pub metrics: Vec<ScalingMetric>,

    #[serde(
        rename = "warmup_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warmup_seconds: Option<i64>,

    #[serde(
        rename = "cooldown_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cooldown_seconds: Option<i64>,

    #[serde(
        rename = "scale_down_stabilization_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_down_stabilization_seconds: Option<i64>,

    #[serde(
        rename = "max_scale_out_step",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_scale_out_step: Option<i64>,

    #[serde(
        rename = "max_scale_in_step",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_scale_in_step: Option<i64>,
    /// Grace period after route withdrawal and proxy acknowledgements, before deleting a retiring member. Long-lived TCP/UDP sessions may end at the deadline; arbitrary application shutdown hooks are not supported.
    #[serde(
        rename = "drain_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drain_seconds: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ScalingMetric {
    #[serde(rename = "source")]
    pub source: ScalingMetricSource,

    #[serde(rename = "target_type")]
    pub target_type: ScalingMetricTargetType,

    #[serde(rename = "target_value")]
    pub target_value: f64,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Exact-match labels; tenancy labels and __name__ cannot be supplied.
    #[serde(rename = "labels", default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::BTreeMap<String, String>>,
    /// Use last for queue gauges; rate for monotonically increasing counters, with reset handling.
    #[serde(
        rename = "sample_aggregation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sample_aggregation: Option<ScalingMetricSampleAggregation>,

    #[serde(
        rename = "series_aggregation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_aggregation: Option<ScalingMetricSeriesAggregation>,
    /// Exact expected cardinality; incomplete or ambiguous selectors are unavailable.
    #[serde(
        rename = "expected_series",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_series: Option<i64>,

    #[serde(
        rename = "window_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub window_seconds: Option<i64>,
    /// Actual newest observation age per series; must not exceed window_seconds. Defaults to the smaller of 90 and the window.
    #[serde(
        rename = "max_age_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_age_seconds: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricSource {
    Cpu,
    Telemetry,
    Unknown(String),
}
impl ScalingMetricSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cpu => "cpu",
            Self::Telemetry => "telemetry",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "cpu" => Self::Cpu,
            "telemetry" => Self::Telemetry,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricTargetType {
    Utilization,
    AverageValue,
    Unknown(String),
}
impl ScalingMetricTargetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Utilization => "utilization",
            Self::AverageValue => "average_value",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricTargetType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "utilization" => Self::Utilization,
            "average_value" => Self::AverageValue,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricSampleAggregation {
    Last,
    Avg,
    Max,
    Rate,
    Unknown(String),
}
impl ScalingMetricSampleAggregation {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Last => "last",
            Self::Avg => "avg",
            Self::Max => "max",
            Self::Rate => "rate",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricSampleAggregation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricSampleAggregation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "last" => Self::Last,
            "avg" => Self::Avg,
            "max" => Self::Max,
            "rate" => Self::Rate,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScalingMetricSeriesAggregation {
    Sum,
    Avg,
    Max,
    Unknown(String),
}
impl ScalingMetricSeriesAggregation {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Sum => "sum",
            Self::Avg => "avg",
            Self::Max => "max",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ScalingMetricSeriesAggregation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScalingMetricSeriesAggregation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "sum" => Self::Sum,
            "avg" => Self::Avg,
            "max" => Self::Max,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutoscalingStatus {
    #[serde(rename = "status")]
    pub status: AutoscalingStatusStatus,

    #[serde(rename = "reason")]
    pub reason: String,

    #[serde(
        rename = "evaluated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub evaluated_at: Option<String>,

    #[serde(
        rename = "last_scaled_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_scaled_at: Option<String>,

    #[serde(rename = "history")]
    pub history: Vec<AutoscalingStatusHistoryItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutoscalingStatusStatus {
    Pending,
    Disabled,
    Stable,
    Scaling,
    Waiting,
    WarmingUp,
    MetricsUnavailable,
    Stabilizing,
    Cooldown,
    Draining,
    Unknown(String),
}
impl AutoscalingStatusStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Disabled => "disabled",
            Self::Stable => "stable",
            Self::Scaling => "scaling",
            Self::Waiting => "waiting",
            Self::WarmingUp => "warming_up",
            Self::MetricsUnavailable => "metrics_unavailable",
            Self::Stabilizing => "stabilizing",
            Self::Cooldown => "cooldown",
            Self::Draining => "draining",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AutoscalingStatusStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AutoscalingStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "disabled" => Self::Disabled,
            "stable" => Self::Stable,
            "scaling" => Self::Scaling,
            "waiting" => Self::Waiting,
            "warming_up" => Self::WarmingUp,
            "metrics_unavailable" => Self::MetricsUnavailable,
            "stabilizing" => Self::Stabilizing,
            "cooldown" => Self::Cooldown,
            "draining" => Self::Draining,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AutoscalingStatusHistoryItem {
    #[serde(rename = "at")]
    pub at: String,

    #[serde(rename = "from")]
    pub from: i64,

    #[serde(rename = "to")]
    pub to: i64,

    #[serde(rename = "reason")]
    pub reason: String,
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
pub struct FloatingIPRef {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "visibility")]
    pub visibility: FloatingIPRefVisibility,

    #[serde(rename = "address")]
    pub address: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FloatingIPRefVisibility {
    Public,
    Private,
    Unknown(String),
}
impl FloatingIPRefVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FloatingIPRefVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatingIPRefVisibility {
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RuleResponse {
    #[serde(rename = "rule")]
    pub rule: Rule,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Rule {
    /// Parent-scoped CRN with immutable load balancer name and child UUID components.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "conditions")]
    pub conditions: Vec<RuleCondition>,

    #[serde(rename = "target_group")]
    pub target_group: ResourceRef,

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetGroupResponse {
    #[serde(rename = "target_group")]
    pub target_group: TargetGroup,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetGroup {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
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
    /// Where the backend set comes from. `static` routes to the targets attached via POST /v1/target-groups/{id}/targets; `pool` resolves live instance addresses from the compute instance pool named by instance_pool, so scaling the pool moves the backends with it.
    #[serde(rename = "target_mode")]
    pub target_mode: TargetGroupTargetMode,
    /// Compute instance pool backing the group. Omitted for static target groups.
    #[serde(
        rename = "instance_pool",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_pool: Option<ResourceRef>,

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListenerCertificateListResponse {
    #[serde(rename = "certificates")]
    pub certificates: Vec<Certificate>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
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

pub type ListListenerCertificatesResponse = ListenerCertificateListResponse;

pub type ListListenerCertificatesItem = Certificate;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListListenersParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListListenersQuery = ListListenersParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListenerListResponse {
    #[serde(rename = "listeners")]
    pub listeners: Vec<Listener>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListListenersResponse = ListenerListResponse;

pub type ListListenersItem = Listener;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListLoadBalancerInstancesParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "current_state",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub current_state: Option<CurrentStateInput>,

    #[serde(rename = "flavor", default, skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,

    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurrentStateInput {
    Pending,
    Building,
    Running,
    Stopping,
    Stopped,
    Rebooting,
    Migrating,
    Deleting,
    Deleted,
    Error,
    Crashed,
    Paused,
    Suspended,
    Unknown(String),
}
impl CurrentStateInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Building => "building",
            Self::Running => "running",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Rebooting => "rebooting",
            Self::Migrating => "migrating",
            Self::Deleting => "deleting",
            Self::Deleted => "deleted",
            Self::Error => "error",
            Self::Crashed => "crashed",
            Self::Paused => "paused",
            Self::Suspended => "suspended",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CurrentStateInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CurrentStateInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "building" => Self::Building,
            "running" => Self::Running,
            "stopping" => Self::Stopping,
            "stopped" => Self::Stopped,
            "rebooting" => Self::Rebooting,
            "migrating" => Self::Migrating,
            "deleting" => Self::Deleting,
            "deleted" => Self::Deleted,
            "error" => Self::Error,
            "crashed" => Self::Crashed,
            "paused" => Self::Paused,
            "suspended" => Self::Suspended,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListLoadBalancerInstancesQuery = ListLoadBalancerInstancesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancerInstancesResponse {
    #[serde(rename = "instances")]
    pub instances: Vec<LoadBalancerInstance>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancerInstance {
    #[serde(rename = "id")]
    pub id: String,
    /// Cloud Resource Name
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
    /// In-flight transition, if any; null when settled.
    #[serde(
        rename = "task_state",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub task_state: Option<crate::Nullable<String>>,
    /// Flavor identity. Every instance has a flavor; query the flavor resource for CPU and RAM.
    #[serde(rename = "flavor")]
    pub flavor: ResourceRef,
    /// Pinned source image identity, or null for a volume-only boot.
    #[serde(rename = "image")]
    pub image: LoadBalancerInstanceImage,
    /// Base64-encoded cloud-init user-data supplied at launch.
    #[serde(rename = "user_data", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Attached IAM role identity, or null when no role is attached. Deleted roles retain their identity without name.
    #[serde(rename = "iam_role")]
    pub iam_role: LoadBalancerInstanceIamRole,

    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    /// Active faults ordered by last_at descending, then internal history id descending for a stable tie-breaker. Healthy resources return \[\].
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(
        rename = "launched_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub launched_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "terminated_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub terminated_at: Option<crate::Nullable<String>>,
    /// What was asked for. Only three values, because there are only three things you can ask an instance to be: Create/Start/Reboot ask for running, Stop for stopped, Delete for deleted.
    #[serde(rename = "desired_state")]
    pub desired_state: LoadBalancerInstanceDesiredState,
    /// Where the instance actually is. Read this one to answer "is it up" — the transitional states live here, not on desired_state, because nobody asks for `stopping`. desired_state=running with current_state=stopped is an instance that was asked to start and has not come up yet.
    #[serde(rename = "current_state")]
    pub current_state: CurrentState,

    #[serde(
        rename = "retirement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retirement: Option<Retirement>,
    /// Proxy health: initializing before the first report, healthy when the last report says the proxy is serving, unhealthy when it is down, and draining when the instance is retiring. See last_seen for the age of the last observation.
    #[serde(rename = "status")]
    pub status: LoadBalancerInstanceStatus,
    /// Whether the proxy reported ready at the last health report
    #[serde(rename = "proxy_ok")]
    pub proxy_ok: bool,

    #[serde(
        rename = "agent_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_version: Option<String>,
    /// Omitted when this instance has not reported yet
    #[serde(rename = "last_seen", default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LoadBalancerInstanceImage {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum LoadBalancerInstanceIamRole {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

pub type Metadata = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadBalancerInstanceDesiredState {
    Running,
    Stopped,
    Deleted,
    Unknown(String),
}
impl LoadBalancerInstanceDesiredState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Running => "running",
            Self::Stopped => "stopped",
            Self::Deleted => "deleted",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LoadBalancerInstanceDesiredState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LoadBalancerInstanceDesiredState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "running" => Self::Running,
            "stopped" => Self::Stopped,
            "deleted" => Self::Deleted,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurrentState {
    Pending,
    Building,
    Running,
    Stopping,
    Stopped,
    Rebooting,
    Migrating,
    Deleting,
    Deleted,
    Error,
    Crashed,
    Paused,
    Suspended,
    Unknown(String),
}
impl CurrentState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Building => "building",
            Self::Running => "running",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Rebooting => "rebooting",
            Self::Migrating => "migrating",
            Self::Deleting => "deleting",
            Self::Deleted => "deleted",
            Self::Error => "error",
            Self::Crashed => "crashed",
            Self::Paused => "paused",
            Self::Suspended => "suspended",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CurrentState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CurrentState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "building" => Self::Building,
            "running" => Self::Running,
            "stopping" => Self::Stopping,
            "stopped" => Self::Stopped,
            "rebooting" => Self::Rebooting,
            "migrating" => Self::Migrating,
            "deleting" => Self::Deleting,
            "deleted" => Self::Deleted,
            "error" => Self::Error,
            "crashed" => Self::Crashed,
            "paused" => Self::Paused,
            "suspended" => Self::Suspended,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Retirement {
    #[serde(rename = "requested_at")]
    pub requested_at: String,

    #[serde(rename = "drain_seconds")]
    pub drain_seconds: i64,

    #[serde(
        rename = "agent_acknowledged_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_acknowledged_at: Option<String>,
    /// Earliest deletion time; absent while withdrawal is pending.
    #[serde(
        rename = "drain_until",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub drain_until: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadBalancerInstanceStatus {
    Initializing,
    Healthy,
    Unhealthy,
    Draining,
    Unknown(String),
}
impl LoadBalancerInstanceStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Initializing => "initializing",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::Draining => "draining",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LoadBalancerInstanceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LoadBalancerInstanceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "initializing" => Self::Initializing,
            "healthy" => Self::Healthy,
            "unhealthy" => Self::Unhealthy,
            "draining" => Self::Draining,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListLoadBalancerInstancesResponse = LoadBalancerInstancesResponse;

pub type ListLoadBalancerInstancesItem = LoadBalancerInstance;

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancerListResponse {
    #[serde(rename = "load_balancers")]
    pub load_balancers: Vec<LoadBalancer>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RuleListResponse {
    #[serde(rename = "rules")]
    pub rules: Vec<Rule>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetGroupListResponse {
    #[serde(rename = "target_groups")]
    pub target_groups: Vec<TargetGroup>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TargetListResponse {
    #[serde(rename = "targets")]
    pub targets: Vec<Target>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListTargetsResponse = TargetListResponse;

pub type ListTargetsItem = Target;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateListenerRequestInput {
    /// UUID, CRN or exact account-scoped target group name. Null clears the default; omission leaves it unchanged.
    #[serde(
        rename = "default_target_group",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub default_target_group: Option<crate::Nullable<String>>,
    /// Mutate which addresses are bound. Omit to leave unchanged.
    #[serde(rename = "exposure", default, skip_serializing_if = "Option::is_none")]
    pub exposure: Option<UpdateListenerRequestInputExposure>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
    /// UUID, CRN, or exact name of a certificate already attached to this listener. Makes it the default and demotes the previous default atomically. Does not attach new certificates.
    #[serde(
        rename = "default_certificate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_certificate: Option<String>,
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
    /// Steady target within min_count and max_count.
    #[serde(
        rename = "desired_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_count: Option<i64>,
    /// Lower capacity bound.
    #[serde(rename = "min_count", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i64>,
    /// Upper capacity bound including rollout surge.
    #[serde(rename = "max_count", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,

    #[serde(
        rename = "autoscaling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub autoscaling: Option<AutoscalingPolicyInput>,
    /// Deprecated alias of desired_count; send only one. Bounds are preserved. With desired_count omitted, it is clamped into the resulting bounds. Scale-in withdraws and drains members before deletion.
    #[serde(
        rename = "replica_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub replica_count: Option<i64>,
    /// Resize each replica to a different compute flavor. Must be a loadbalancer-family flavor. A running instance cannot change size in place, so the request records the new size and returns; the replicas already up are then replaced one at a time in the background. The load balancer temporarily runs one replica over desired_count, within max_count while it does: the extra replica comes up on the new flavor and starts serving before any replica on the old one is retired, so the number serving never drops below desired_count — a resize does not cost you capacity, at any replica count. Expect it to take several minutes, and poll GET /v1/load-balancers/{id}/instances to watch: a replica has been replaced when its id changes, and the resize is done when every flavor.id there matches the requested flavor. A resize requires max_count above desired_count for surge headroom. A rollout waits if headroom is removed while it is in progress. Rejected up front if the account does not have the compute quota for the replacement replica, so a resize cannot half-apply and leave the load balancer short.
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
