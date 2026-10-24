//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachInstanceInterfaceRequest {
    /// Existing standalone interface UUID or complete VPC/subnet/interface CRN. Bare names lack the subnet parent and are rejected. It keeps its address, MAC, and security groups; detach returns it to standalone instead of destroying it.
    #[serde(rename = "interface")]
    pub interface: String,
}
impl AttachInstanceInterfaceRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(interface: String) -> Self {
        Self { interface }
    }
}

pub type AttachInstanceInterfaceBody = AttachInstanceInterfaceRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceInterfaceResponse {
    #[serde(rename = "interface")]
    pub interface: AttachedInterface,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachedInterface {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Subnet containing this interface.
    #[serde(rename = "subnet")]
    pub subnet: ResourceRef,
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
    /// Instance holding this interface, including a stopped instance. Null when unbound. Floating IP attachments are tracked separately.
    #[serde(rename = "attached_to")]
    pub attached_to: AttachedInterfaceAttachedTo,

    #[serde(rename = "tags")]
    pub tags: std::collections::BTreeMap<String, String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "boot_index")]
    pub boot_index: i64,
    /// The lowest-boot-index NIC — the one carrying the guest's default and metadata routes.
    #[serde(rename = "primary")]
    pub primary: bool,
    /// A customer-attached standalone interface — detach unbinds it instead of destroying it.
    #[serde(rename = "external")]
    pub external: bool,
    /// The guest does not carry the interface yet and a hard reboot is what delivers it — an interface past the first is a network on the instance's launcher, and a launcher's networks are fixed for its lifetime. Set when the instance was running in a region that cannot attach to a running guest. Absent for a stopped instance, which comes up with the device, and absent where the region attaches live, where the running guest is given the device without a restart.
    #[serde(
        rename = "restart_required",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub restart_required: Option<bool>,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum AttachedInterfaceAttachedTo {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

pub type AttachInstanceInterfaceResponse = InstanceInterfaceResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolFloatingIpAttachRequestInput {
    /// An account-scoped floating IP UUID or CRN (bare names are not accepted), currently attached to nothing. This binds it to the pool; it does not allocate one.
    #[serde(rename = "floating_ip")]
    pub floating_ip: String,
}
impl InstancePoolFloatingIpAttachRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(floating_ip: String) -> Self {
        Self { floating_ip }
    }
}

pub type AttachInstancePoolFloatingIpBody = InstancePoolFloatingIpAttachRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolFloatingIpResponse {
    #[serde(rename = "floating_ip")]
    pub floating_ip: FloatingIp,
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
    /// Instance pool or load balancer that owns the address. Null for direct interface attachments and unattached addresses. An empty pool retains ownership.
    #[serde(rename = "attached_to")]
    pub attached_to: FloatingIpAttachedTo,

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
    /// Allocation subnet for a private floating IP; null for public addresses.
    #[serde(rename = "subnet")]
    pub subnet: FloatingIpSubnet,
    /// Allocation VPC for a private floating IP; null for public addresses.
    #[serde(rename = "vpc")]
    pub vpc: FloatingIpVpc,
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
#[serde(untagged)]
pub enum FloatingIpAttachedTo {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum FloatingIpSubnet {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum FloatingIpVpc {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

pub type AttachInstancePoolFloatingIpResponse = InstancePoolFloatingIpResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachInstanceVolumeRequest {
    /// Account-scoped volume reference (UUID, CRN or exact name).
    #[serde(rename = "volume")]
    pub volume: String,
    /// Optional device-name override; auto-picks the next free slot (vdb/vdc/…) when omitted.
    #[serde(rename = "device", default, skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// When set, the in-guest agent formats the disk (only if blank) and mounts it at this path. Empty attaches the block device only.
    #[serde(
        rename = "mount_path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_path: Option<String>,
    /// Filesystem the in-guest agent formats the disk with, and only when `mount_path` is set and the disk is blank. Rejected with 400 if it is neither value.
    #[serde(rename = "fstype", default, skip_serializing_if = "Option::is_none")]
    pub fstype: Option<AttachInstanceVolumeRequestFstype>,
}
impl AttachInstanceVolumeRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(volume: String) -> Self {
        Self {
            volume,
            device: None,
            mount_path: None,
            fstype: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttachInstanceVolumeRequestFstype {
    Ext4,
    Xfs,
    Unknown(String),
}
impl AttachInstanceVolumeRequestFstype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ext4 => "ext4",
            Self::Xfs => "xfs",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AttachInstanceVolumeRequestFstype {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AttachInstanceVolumeRequestFstype {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ext4" => Self::Ext4,
            "xfs" => Self::Xfs,
            _ => Self::Unknown(value),
        })
    }
}

pub type AttachInstanceVolumeBody = AttachInstanceVolumeRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceVolumeResponse {
    #[serde(rename = "volume")]
    pub volume: AttachedVolume,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AttachedVolume {
    #[serde(rename = "id")]
    pub id: String,
    /// Cloud Resource Name (name-based, region-scoped).
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
    pub tags: Tags,

    #[serde(rename = "volume_type")]
    pub volume_type: VolumeTypeName,

    #[serde(rename = "size_gb")]
    pub size_gb: i64,

    #[serde(rename = "performance")]
    pub performance: VolumePerformance,

    #[serde(rename = "included_io_limits")]
    pub included_io_limits: IncludedIOLimits,

    #[serde(rename = "status")]
    pub status: VolumeStatus,
    /// Set at create time when the volume is provisioned from a bootable image. Immutable after creation.
    #[serde(rename = "bootable")]
    pub bootable: bool,
    /// Active faults, newest first. Empty when healthy. Status is error if and only if an active error-severity fault exists. Resolved history is retained internally.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
    /// Source image identity, or null when this volume was not created from an image.
    #[serde(rename = "source_image")]
    pub source_image: AttachedVolumeSourceImage,
    /// Source snapshot identity, or null when not created from a snapshot.
    #[serde(rename = "source_snapshot")]
    pub source_snapshot: AttachedVolumeSourceSnapshot,

    #[serde(rename = "device")]
    pub device: String,

    #[serde(rename = "boot_index")]
    pub boot_index: i64,

    #[serde(rename = "delete_on_termination")]
    pub delete_on_termination: bool,

    #[serde(
        rename = "mount_path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_path: Option<String>,
    /// Filesystem the in-guest agent formatted the volume with, or absent when the attachment did not name one and the agent used the ext4 default. Every write path — instance create, pool template and attach — refuses anything else, so this is the whole set the field can hold.
    #[serde(rename = "fstype", default, skip_serializing_if = "Option::is_none")]
    pub fstype: Option<AttachedVolumeFstype>,

    #[serde(rename = "mount", default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<VolumeMount>,
}

pub type Tags = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VolumeTypeName {
    Hdd,
    Ssd,
    Nvme,
    Unknown(String),
}
impl VolumeTypeName {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Hdd => "hdd",
            Self::Ssd => "ssd",
            Self::Nvme => "nvme",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for VolumeTypeName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VolumeTypeName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "hdd" => Self::Hdd,
            "ssd" => Self::Ssd,
            "nvme" => Self::Nvme,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumePerformance {
    #[serde(rename = "requested")]
    pub requested: IncludedIOLimits,

    #[serde(rename = "applied")]
    pub applied: IncludedIOLimits,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IncludedIOLimits {
    #[serde(rename = "iops")]
    pub iops: i64,

    #[serde(rename = "bytes_per_sec")]
    pub bytes_per_sec: i64,
    /// Always zero for SSD and NVMe volumes; IOPS bursting is disabled.
    #[serde(rename = "burst_iops")]
    pub burst_iops: i64,
    /// Always zero for SSD and NVMe volumes; throughput bursting is disabled.
    #[serde(rename = "burst_bytes_per_sec")]
    pub burst_bytes_per_sec: i64,
    /// Always zero for SSD and NVMe volumes; no burst duration applies.
    #[serde(rename = "burst_seconds")]
    pub burst_seconds: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VolumeStatus {
    Creating,
    Available,
    InUse,
    Extending,
    Updating,
    Deleting,
    Error,
    Unknown(String),
}
impl VolumeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Creating => "creating",
            Self::Available => "available",
            Self::InUse => "in_use",
            Self::Extending => "extending",
            Self::Updating => "updating",
            Self::Deleting => "deleting",
            Self::Error => "error",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for VolumeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VolumeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "creating" => Self::Creating,
            "available" => Self::Available,
            "in_use" => Self::InUse,
            "extending" => Self::Extending,
            "updating" => Self::Updating,
            "deleting" => Self::Deleting,
            "error" => Self::Error,
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
pub enum AttachedVolumeSourceImage {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum AttachedVolumeSourceSnapshot {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttachedVolumeFstype {
    Ext4,
    Xfs,
    Unknown(String),
}
impl AttachedVolumeFstype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ext4 => "ext4",
            Self::Xfs => "xfs",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AttachedVolumeFstype {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AttachedVolumeFstype {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ext4" => Self::Ext4,
            "xfs" => Self::Xfs,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumeMount {
    /// - `unknown` — no guest agent has ever reported on this volume. The agent may predate this feature, may have been removed (which is supported), or the guest may never have booted. - `pending` — the agent cannot mount it yet and expects that to change. The ordinary state for the first seconds after an attach, while the hot-plugged disk appears in the guest. - `mounted` — mounted at `mount_path`. May still carry a `code`. - `failed` — it will not mount until something changes. Either the agent reported a refusal that waiting cannot fix, or it has been unable to make progress for long enough that waiting is no longer the explanation. `code` says which.
    #[serde(rename = "state")]
    pub state: VolumeMountState,
    /// Why the volume is in this state. Absent when there is nothing to say. Independent of `state` rather than something only a failure carries: `fstab_write_failed` accompanies a **mounted** volume whose fstab entry could not be written, which works now and will be gone after the next reboot. The commonest one to act on is `signatures_no_filesystem` — the disk carries a partition table or other signatures but no mountable filesystem, so the agent will not format it, because formatting would destroy what is there. A volume cloned from a boot disk and attached with a `mount_path` lands here. Partition and format it inside the guest, or attach it without a `mount_path` and mount it yourself. `unknown_error` is a code this platform does not recognise, reported by a guest agent newer than the region.
    #[serde(rename = "code", default, skip_serializing_if = "Option::is_none")]
    pub code: Option<VolumeMountCode>,
    /// Human-readable detail from inside the guest — the failing command's output, the partition table type it found. Free text originating in the customer's own VM: sanitised and capped, but display it as text, never as markup.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// When the volume entered this condition. Absent when `state` is `unknown`.
    #[serde(rename = "since", default, skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// When the guest agent last reported, whether or not anything had changed. A `reported_at` far in the past means the agent has stopped talking to us, and the state beside it is what it last said rather than what is true now. Absent when `state` is `unknown`.
    #[serde(
        rename = "reported_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reported_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VolumeMountState {
    Unknown1,
    Pending,
    Mounted,
    Failed,
    Unknown(String),
}
impl VolumeMountState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unknown1 => "unknown",
            Self::Pending => "pending",
            Self::Mounted => "mounted",
            Self::Failed => "failed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for VolumeMountState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VolumeMountState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "unknown" => Self::Unknown1,
            "pending" => Self::Pending,
            "mounted" => Self::Mounted,
            "failed" => Self::Failed,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VolumeMountCode {
    UnsafeSerial,
    DeviceAbsent,
    ProbeFailed,
    SignaturesNoFilesystem,
    UnsupportedFstype,
    MkfsFailed,
    UnsafeMountPath,
    MkdirFailed,
    MountFailed,
    FstabWriteFailed,
    UnknownError,
    Unknown(String),
}
impl VolumeMountCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::UnsafeSerial => "unsafe_serial",
            Self::DeviceAbsent => "device_absent",
            Self::ProbeFailed => "probe_failed",
            Self::SignaturesNoFilesystem => "signatures_no_filesystem",
            Self::UnsupportedFstype => "unsupported_fstype",
            Self::MkfsFailed => "mkfs_failed",
            Self::UnsafeMountPath => "unsafe_mount_path",
            Self::MkdirFailed => "mkdir_failed",
            Self::MountFailed => "mount_failed",
            Self::FstabWriteFailed => "fstab_write_failed",
            Self::UnknownError => "unknown_error",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for VolumeMountCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VolumeMountCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "unsafe_serial" => Self::UnsafeSerial,
            "device_absent" => Self::DeviceAbsent,
            "probe_failed" => Self::ProbeFailed,
            "signatures_no_filesystem" => Self::SignaturesNoFilesystem,
            "unsupported_fstype" => Self::UnsupportedFstype,
            "mkfs_failed" => Self::MkfsFailed,
            "unsafe_mount_path" => Self::UnsafeMountPath,
            "mkdir_failed" => Self::MkdirFailed,
            "mount_failed" => Self::MountFailed,
            "fstab_write_failed" => Self::FstabWriteFailed,
            "unknown_error" => Self::UnknownError,
            _ => Self::Unknown(value),
        })
    }
}

pub type AttachInstanceVolumeResponse = InstanceVolumeResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageCreateRequestInput {
    /// Immutable image name (e.g. debian-13) whose current-version pointer can move; the new image becomes its current version. Names are shared across a tag's builds — one build is identified by owner, name, architecture and version. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// Presigned https GET URL to the disk in an object store you control. Fetched once by the import worker (which rejects private/link-local targets). The worker detects qcow2, raw, vmdk, vhd, vhdx or vdi and converts it to raw storage. Unreadable or unsupported sources and images declaring backing files fail asynchronously with status error and an active conversion fault. Not retained after import.
    #[serde(rename = "source_url")]
    pub source_url: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Operating system distribution. Use linux for another or generic Linux distribution; os_version specifies the release separately.
    #[serde(rename = "os", default, skip_serializing_if = "Option::is_none")]
    pub os: Option<ImageCreateRequestInputOs>,

    #[serde(
        rename = "os_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub os_version: Option<String>,
    /// CPU architecture of the source image. Only amd64 (x86-64) is supported.
    #[serde(
        rename = "architecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture: Option<ImageCreateRequestInputArchitecture>,
    /// Identifies this build within `name`, and must be unique there — re-publishing a version that a tag already carries is a 409. Omit it and the server stamps a UTC timestamp, so every build is addressable as `name:version` whether or not you labelled it.
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Make this the current version for its (name, architecture) once active.
    #[serde(rename = "current", default, skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    /// The day this release stops receiving free security updates. Omit it and the image inherits the date the name's current version carries, so re-publishing a tag can't quietly stop tracking its release.
    #[serde(rename = "eol_date", default, skip_serializing_if = "Option::is_none")]
    pub eol_date: Option<String>,

    #[serde(
        rename = "min_disk_gb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_disk_gb: Option<i64>,

    #[serde(
        rename = "min_ram_mb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_ram_mb: Option<i64>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
}
impl ImageCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, source_url: String) -> Self {
        Self {
            name,
            source_url,
            description: None,
            os: None,
            os_version: None,
            architecture: None,
            version: None,
            current: None,
            eol_date: None,
            min_disk_gb: None,
            min_ram_mb: None,
            tags: None,
            attributes: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageCreateRequestInputOs {
    Almalinux,
    Alpine,
    Arch,
    Centos,
    Debian,
    Fedora,
    Opensuse,
    Rhel,
    Rocky,
    Ubuntu,
    Linux,
    Unknown(String),
}
impl ImageCreateRequestInputOs {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Almalinux => "almalinux",
            Self::Alpine => "alpine",
            Self::Arch => "arch",
            Self::Centos => "centos",
            Self::Debian => "debian",
            Self::Fedora => "fedora",
            Self::Opensuse => "opensuse",
            Self::Rhel => "rhel",
            Self::Rocky => "rocky",
            Self::Ubuntu => "ubuntu",
            Self::Linux => "linux",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ImageCreateRequestInputOs {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ImageCreateRequestInputOs {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "almalinux" => Self::Almalinux,
            "alpine" => Self::Alpine,
            "arch" => Self::Arch,
            "centos" => Self::Centos,
            "debian" => Self::Debian,
            "fedora" => Self::Fedora,
            "opensuse" => Self::Opensuse,
            "rhel" => Self::Rhel,
            "rocky" => Self::Rocky,
            "ubuntu" => Self::Ubuntu,
            "linux" => Self::Linux,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageCreateRequestInputArchitecture {
    Amd64,
    Unknown(String),
}
impl ImageCreateRequestInputArchitecture {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Amd64 => "amd64",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ImageCreateRequestInputArchitecture {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ImageCreateRequestInputArchitecture {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "amd64" => Self::Amd64,
            _ => Self::Unknown(value),
        })
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateImageBody = ImageCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageResponse {
    #[serde(rename = "image")]
    pub image: Image,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Image {
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

    #[serde(rename = "os", default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(
        rename = "os_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub os_version: Option<String>,

    #[serde(rename = "architecture")]
    pub architecture: String,
    /// The build's identity within its name. Unique there: a name is a movable tag, so it can't also be what tells two builds apart. Stamped as a UTC timestamp when the uploader didn't choose one.
    #[serde(rename = "version")]
    pub version: String,
    /// Whether this is the version resolve-by-name returns for its (name, architecture) — i.e. the name's current tag target.
    #[serde(
        rename = "is_current",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_current: Option<bool>,
    /// The day this image's OS release stops receiving free security updates for a default install. Absent when nobody has recorded one — which means unknown, not "supported indefinitely". Platform images are withdrawn from the catalog a grace period after this date. They stay bootable by id until then, and the date is published well ahead of it so you can plan the move.
    #[serde(rename = "eol_date", default, skip_serializing_if = "Option::is_none")]
    pub eol_date: Option<String>,

    #[serde(
        rename = "size_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub size_bytes: Option<i64>,

    #[serde(
        rename = "min_disk_gb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_disk_gb: Option<i64>,

    #[serde(
        rename = "min_ram_mb",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_ram_mb: Option<i64>,
    /// Error exactly while an active error fault exists; import and deletion progress remain independently retryable.
    #[serde(rename = "status")]
    pub status: ImageStatus,
    /// Why this image was withdrawn. Present for withdrawn images, including those with an independent error fault: end_of_life for platform release withdrawal (see eol_date), or legacy for a migrated withdrawal whose original reason is unknown. Withdrawn images retain their data but cannot be launched.
    #[serde(
        rename = "withdrawal_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub withdrawal_reason: Option<String>,
    /// Present in owner list/detail responses while image deletion is waiting for existing instance, source-reservation, or instance-pool references. Counts include all referencing accounts without disclosing their identities. The backing data remains intact; cleanup resumes when references are gone. Independent faults may still set status to error.
    #[serde(
        rename = "deletion_retention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deletion_retention: Option<ImageDeletionRetention>,
    /// Active faults, newest first. Empty for a healthy image. Error faults set status to error without disabling an eligible import or cleanup retry.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageStatus {
    Pending,
    Importing,
    Active,
    Error,
    Deleting,
    Withdrawn,
    Unknown(String),
}
impl ImageStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Importing => "importing",
            Self::Active => "active",
            Self::Error => "error",
            Self::Deleting => "deleting",
            Self::Withdrawn => "withdrawn",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ImageStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ImageStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "importing" => Self::Importing,
            "active" => Self::Active,
            "error" => Self::Error,
            "deleting" => Self::Deleting,
            "withdrawn" => Self::Withdrawn,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageDeletionRetention {
    #[serde(rename = "reason")]
    pub reason: ImageDeletionRetentionReason,

    #[serde(rename = "instances")]
    pub instances: i64,

    #[serde(rename = "instance_pools")]
    pub instance_pools: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImageDeletionRetentionReason {
    InUse,
    Unknown(String),
}
impl ImageDeletionRetentionReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::InUse => "in_use",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ImageDeletionRetentionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ImageDeletionRetentionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "in_use" => Self::InUse,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateImageResponse = ImageResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceCreateRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Regional flavor reference (UUID, CRN or exact name)
    #[serde(rename = "flavor")]
    pub flavor: String,
    /// Architecture for image names and name:version tags (default amd64); a CRN pins its own architecture and version.
    #[serde(
        rename = "architecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture: Option<String>,
    /// Image to clone the boot disk from. Required unless volumes contains an existing boot volume; cannot be combined with an existing boot volume. Four forms are accepted: a complete image/name/architecture/arch/version/version CRN; an image id; `name:version`, which pins one build and is how you opt out of the tag moving under you; or a bare `name`, which follows the tag to whichever build is current when the instance is created. Names prefer a usable caller-owned build over a tagged platform catalog build for the requested architecture (default amd64). A CRN pins owner, name, architecture and version. Resolution never retries another reference kind; responses and stored templates retain the resolved image UUID.
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Interfaces to attach, at least one. index 0 is the primary NIC. Required because an instance with no interface boots with no network at all, and nothing inside it can add one afterwards.
    #[serde(rename = "networks")]
    pub networks: Vec<NetworkConfigInput>,
    /// New or existing disks bound with the instance, the boot disk included — mark it with `boot: true`. At most one entry may. Omit the boot entry to take the image's minimum size and the region's default tier.
    #[serde(rename = "volumes", default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<InstanceLaunchVolumeInput>>,

    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
    /// Base64-encoded user data (cloud-init)
    #[serde(rename = "user_data", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Attach an IAM role from the same account by UUID, CRN or exact name. The role's trust policy must permit `crn:compute:*:*:instance/*` (or the specific instance CRN). The instance's IMDS endpoint (169.254.169.254) mints short-lived STS credentials for this role from inside the VM.
    #[serde(rename = "iam_role", default, skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
}
impl InstanceCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, flavor: String, networks: Vec<NetworkConfigInput>) -> Self {
        Self {
            name,
            description: None,
            flavor,
            architecture: None,
            image: None,
            networks,
            volumes: None,
            metadata: None,
            tags: None,
            user_data: None,
            iam_role: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfigInput {
    /// Subnet UUID or complete VPC/subnet CRN. Bare names require a VPC parent and are rejected here.
    #[serde(rename = "subnet")]
    pub subnet: String,
    /// Optional MAC address. Must be locally-administered (`X2:`, `X6:`, `XA:`, `XE:` in the first octet). Generated when omitted.
    #[serde(rename = "mac", default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    /// Account-scoped security group references (UUID, CRN or name) to attach to this NIC. Each must be owned by the same account. Empty list = no per-NIC ACLs (the platform's default-allow stays in force).
    #[serde(
        rename = "security_groups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_groups: Option<Vec<String>>,
    /// Allocate public floating IPs for this NIC at launch. Explicit families require matching guest addresses and internet routes. Detach leaves the FIP reserved. No ordinary public IPv4 mapping exists.
    #[serde(
        rename = "floating_ip_assignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ip_assignment: Option<NetworkConfigInputFloatingIpAssignment>,

    #[serde(rename = "addresses", default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<AddressRequestInput>>,
}
impl NetworkConfigInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(subnet: String) -> Self {
        Self {
            subnet,
            mac: None,
            security_groups: None,
            floating_ip_assignment: None,
            addresses: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkConfigInputFloatingIpAssignment {
    None,
    Ipv4,
    Ipv6,
    DualStack,
    Auto,
    Unknown(String),
}
impl NetworkConfigInputFloatingIpAssignment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::DualStack => "dual_stack",
            Self::Auto => "auto",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for NetworkConfigInputFloatingIpAssignment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NetworkConfigInputFloatingIpAssignment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "none" => Self::None,
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            "dual_stack" => Self::DualStack,
            "auto" => Self::Auto,
            _ => Self::Unknown(value),
        })
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InstanceLaunchVolumeInput {
    Variant1(Box<InstanceLaunchVolumeInputVariant1>),
    Variant2(Box<InstanceLaunchVolumeInputVariant2>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceLaunchVolumeInputVariant1 {
    /// Select the one boot disk. Boot disks cannot specify mount_path or fstype.
    #[serde(rename = "boot", default, skip_serializing_if = "Option::is_none")]
    pub boot: Option<bool>,
    /// Existing available volume UUID, name, or CRN. Mutually exclusive with new-disk settings.
    #[serde(rename = "volume")]
    pub volume: String,
    /// New disk capacity. Required for new data disks; boot disks default to the image minimum.
    #[serde(rename = "size_gb", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i64>,
    /// New disk tier; omitted uses the region default.
    #[serde(
        rename = "volume_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_type: Option<InstanceLaunchVolumeInputVariant1VolumeType>,

    #[serde(
        rename = "performance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub performance: Option<VolumePerformanceRequestInput>,
    /// Optional data disk mount path. The guest agent formats only blank disks.
    #[serde(
        rename = "mount_path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_path: Option<String>,
    /// Optional filesystem for blank data disks; defaults to ext4.
    #[serde(rename = "fstype", default, skip_serializing_if = "Option::is_none")]
    pub fstype: Option<String>,

    #[serde(
        rename = "delete_on_termination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_on_termination: Option<serde_json::Value>,
    /// Independent schedules for a new disk. Names must be unique across the account and this launch. Requires storage:CreateSnapshotPolicy. Existing disks keep their schedules and cannot specify this field.
    #[serde(
        rename = "snapshot_schedules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_schedules: Option<Vec<SnapshotScheduleSettingsInput>>,
}
impl InstanceLaunchVolumeInputVariant1 {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(volume: String) -> Self {
        Self {
            boot: None,
            volume,
            size_gb: None,
            volume_type: None,
            performance: None,
            mount_path: None,
            fstype: None,
            delete_on_termination: None,
            snapshot_schedules: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceLaunchVolumeInputVariant1VolumeType {
    Ssd,
    Nvme,
    Unknown(String),
}
impl InstanceLaunchVolumeInputVariant1VolumeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ssd => "ssd",
            Self::Nvme => "nvme",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InstanceLaunchVolumeInputVariant1VolumeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InstanceLaunchVolumeInputVariant1VolumeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ssd" => Self::Ssd,
            "nvme" => Self::Nvme,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VolumePerformanceRequestInput {
    #[serde(rename = "iops", default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,

    #[serde(
        rename = "throughput_mib_s",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub throughput_mib_s: Option<f64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SnapshotScheduleSettingsInput {
    /// Account-unique snapshot policy name, subject to resource-name validation.
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "interval_minutes")]
    pub interval_minutes: SnapshotIntervalMinutesInput,

    #[serde(rename = "retention_count")]
    pub retention_count: SnapshotRetentionCountInput,

    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_days: Option<SnapshotRetentionDaysInput>,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl SnapshotScheduleSettingsInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        interval_minutes: SnapshotIntervalMinutesInput,
        retention_count: SnapshotRetentionCountInput,
    ) -> Self {
        Self {
            name,
            description: None,
            interval_minutes,
            retention_count,
            retention_days: None,
            enabled: None,
            tags: None,
        }
    }
}

pub type SnapshotIntervalMinutesInput = i64;

pub type SnapshotRetentionCountInput = i64;

pub type SnapshotRetentionDaysInput = i64;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InstanceLaunchVolumeInputVariant2 {
    /// Select the one boot disk. Boot disks cannot specify mount_path or fstype.
    #[serde(rename = "boot", default, skip_serializing_if = "Option::is_none")]
    pub boot: Option<bool>,
    /// Existing available volume UUID, name, or CRN. Mutually exclusive with new-disk settings.
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    /// New disk capacity. Required for new data disks; boot disks default to the image minimum.
    #[serde(rename = "size_gb", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i64>,
    /// New disk tier; omitted uses the region default.
    #[serde(
        rename = "volume_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_type: Option<InstanceLaunchVolumeInputVariant2VolumeType>,

    #[serde(
        rename = "performance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub performance: Option<VolumePerformanceRequestInput>,
    /// Optional data disk mount path. The guest agent formats only blank disks.
    #[serde(
        rename = "mount_path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_path: Option<String>,
    /// Optional filesystem for blank data disks; defaults to ext4.
    #[serde(rename = "fstype", default, skip_serializing_if = "Option::is_none")]
    pub fstype: Option<String>,
    /// Defaults true for new disks. Existing disks require false or omission and are retained.
    #[serde(
        rename = "delete_on_termination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_on_termination: Option<bool>,
    /// Independent schedules for a new disk. Names must be unique across the account and this launch. Requires storage:CreateSnapshotPolicy. Existing disks keep their schedules and cannot specify this field.
    #[serde(
        rename = "snapshot_schedules",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_schedules: Option<Vec<SnapshotScheduleSettingsInput>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceLaunchVolumeInputVariant2VolumeType {
    Ssd,
    Nvme,
    Unknown(String),
}
impl InstanceLaunchVolumeInputVariant2VolumeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ssd => "ssd",
            Self::Nvme => "nvme",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InstanceLaunchVolumeInputVariant2VolumeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InstanceLaunchVolumeInputVariant2VolumeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ssd" => Self::Ssd,
            "nvme" => Self::Nvme,
            _ => Self::Unknown(value),
        })
    }
}

pub type MetadataInput = std::collections::BTreeMap<String, String>;

pub type CreateInstanceBody = InstanceCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateInstanceResult {
    #[serde(rename = "instance")]
    pub instance: Instance,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Instance {
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
    pub image: InstanceImage,
    /// Base64-encoded cloud-init user-data supplied at launch.
    #[serde(rename = "user_data", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// Attached IAM role identity, or null when no role is attached. Deleted roles retain their identity without name.
    #[serde(rename = "iam_role")]
    pub iam_role: InstanceIamRole,

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
    pub desired_state: InstanceDesiredState,
    /// Where the instance actually is. Read this one to answer "is it up" — the transitional states live here, not on desired_state, because nobody asks for `stopping`. desired_state=running with current_state=stopped is an instance that was asked to start and has not come up yet.
    #[serde(rename = "current_state")]
    pub current_state: CurrentState,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InstanceImage {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InstanceIamRole {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

pub type Metadata = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceDesiredState {
    Running,
    Stopped,
    Deleted,
    Unknown(String),
}
impl InstanceDesiredState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Running => "running",
            Self::Stopped => "stopped",
            Self::Deleted => "deleted",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InstanceDesiredState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InstanceDesiredState {
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

pub type CreateInstanceResponse = CreateInstanceResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolCreateRequestInput {
    /// Subnet UUIDs or CRNs in the same VPC.
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,

    #[serde(
        rename = "autoscaling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub autoscaling: Option<AutoscalingPolicyInput>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Labels on the pool resource, for IAM conditions (`basalt:RequestTag/<key>` here, `basalt:ResourceTag/<key>` on later operations) and cost attribution. They are not propagated to the instances the pool launches; `template.tags` is that set. A pool field, sent beside `template`. Replica tags are only reachable through `template.tags`.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,

    #[serde(rename = "template")]
    pub template: InstancePoolTemplateRequestInput,

    #[serde(
        rename = "desired_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_count: Option<i64>,

    #[serde(rename = "min_count", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i64>,
    /// A value of 0 means the pool holds no members until max_count is raised.
    #[serde(rename = "max_count", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
}
impl InstancePoolCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        subnets: Vec<String>,
        name: String,
        template: InstancePoolTemplateRequestInput,
    ) -> Self {
        Self {
            subnets,
            autoscaling: None,
            name,
            description: None,
            tags: None,
            template,
            desired_count: None,
            min_count: None,
            max_count: None,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolTemplateRequestInput {
    /// Regional flavor reference (UUID, CRN or exact name).
    #[serde(rename = "flavor")]
    pub flavor: String,

    #[serde(
        rename = "architecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture: Option<String>,
    /// Image to clone each replica's boot disk from. Accepts the same four forms instance create does: an architecture-qualified CRN, an image id, `name:version`, or a bare `name`. Unlike instance create, the reference is resolved ONCE, when the pool is created, and the resulting image id is what every replica boots — including replacements spawned months later. A tag re-resolved per replica would let a heal boot a newer build than its siblings, and a pool whose members are quietly not identical is the premise of the primitive breaking silently. To move a pool to a new build, change the template.
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Address families allocated per member; fixed addresses are not allowed.
    #[serde(rename = "addresses", default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<AddressRequestInput>>,
    /// Security group selectors applied to every member interface.
    #[serde(
        rename = "security_groups",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub security_groups: Option<Vec<String>>,
    /// Per-member public floating IP assignment. Explicit families require addresses and internet routes in every selected subnet.
    #[serde(
        rename = "floating_ip_assignment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub floating_ip_assignment: Option<InstancePoolTemplateRequestInputFloatingIpAssignment>,
    /// Base64-encoded user data (cloud-init), stamped on every replica.
    #[serde(rename = "user_data", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,

    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataInput>,
    /// Tags stamped on every instance this template launches. These are the replicas' tags, not the pool's — the pool's own labels are the top-level `tags`, and the two are independent. Changing them affects FUTURE launches only. The instances already running keep the tags they were launched with, so between the change and a refresh the pool holds members carrying two different tag sets; `stale_instance_count` is how many are still on the old one. POST /v1/instance-pools/{pool_id}/refresh rolls them onto the current template.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
    /// IAM role reference from the same account (UUID, CRN or exact name). PassRole and instance trust authorization apply.
    #[serde(rename = "iam_role", default, skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    /// Per-replica disks, the boot disk included — mark it with `boot: true`. Each new replica receives the configured provisioned performance. Omitted performance uses the included allowance. Existing volumes and snapshot schedules are not supported in pool templates.
    #[serde(rename = "volumes", default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<InstanceVolumeInput>>,
}
impl InstancePoolTemplateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(flavor: String) -> Self {
        Self {
            flavor,
            architecture: None,
            image: None,
            addresses: None,
            security_groups: None,
            floating_ip_assignment: None,
            user_data: None,
            metadata: None,
            tags: None,
            iam_role: None,
            volumes: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstancePoolTemplateRequestInputFloatingIpAssignment {
    None,
    Ipv4,
    Ipv6,
    DualStack,
    Auto,
    Unknown(String),
}
impl InstancePoolTemplateRequestInputFloatingIpAssignment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::DualStack => "dual_stack",
            Self::Auto => "auto",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InstancePoolTemplateRequestInputFloatingIpAssignment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InstancePoolTemplateRequestInputFloatingIpAssignment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "none" => Self::None,
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            "dual_stack" => Self::DualStack,
            "auto" => Self::Auto,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceVolumeInput {
    /// Marks the boot disk. It takes no mount_path or fstype — both come from the image — and sending either is refused rather than ignored.
    #[serde(rename = "boot", default, skip_serializing_if = "Option::is_none")]
    pub boot: Option<bool>,

    #[serde(rename = "size_gb")]
    pub size_gb: i64,
    /// Tier; omitted = the region default.
    #[serde(
        rename = "volume_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_type: Option<String>,

    #[serde(
        rename = "performance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub performance: Option<VolumePerformanceRequestInput>,

    #[serde(
        rename = "mount_path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_path: Option<String>,
    /// Filesystem the in-guest agent formats the volume with.
    #[serde(rename = "fstype", default, skip_serializing_if = "Option::is_none")]
    pub fstype: Option<String>,
    /// Destroyed with the instance unless set false.
    #[serde(
        rename = "delete_on_termination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_on_termination: Option<bool>,
}
impl InstanceVolumeInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(size_gb: i64) -> Self {
        Self {
            boot: None,
            size_gb,
            volume_type: None,
            performance: None,
            mount_path: None,
            fstype: None,
            delete_on_termination: None,
        }
    }
}

pub type CreateInstancePoolBody = InstancePoolCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolResponse {
    #[serde(rename = "instance_pool")]
    pub instance_pool: InstancePool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePool {
    /// Eligible placement subnets in the same account, region and VPC.
    #[serde(rename = "subnets")]
    pub subnets: Vec<ResourceRef>,

    #[serde(rename = "vpc")]
    pub vpc: InstancePoolVpc,

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
    /// Temporary rollout capacity; desired_count remains the steady target.
    #[serde(
        rename = "rollout_surge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rollout_surge: Option<bool>,
    /// Instance UUIDs protected from manual and automatic scale-in. Protection does not prevent refresh, failed-instance replacement, or explicit deletion.
    #[serde(
        rename = "scale_in_protected_instances",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_in_protected_instances: Option<Vec<String>>,
    /// Why scale-in protection currently prevents the requested reduction. Omitted when not blocked.
    #[serde(
        rename = "scale_in_blocked_reason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_in_blocked_reason: Option<String>,

    #[serde(
        rename = "retiring_instances",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retiring_instances: Option<Vec<RetiringPoolMember>>,

    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name. This is the value an IAM policy statement must name to scope a permission to this pool alone; a policy written against anything else will not match.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(
        rename = "desired_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_count: Option<i64>,

    #[serde(rename = "min_count", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i64>,
    /// A value of 0 means the pool holds no members until max_count is raised.
    #[serde(rename = "max_count", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
    /// How many members are UP — bound instances whose current_state is `running`.
    #[serde(
        rename = "live_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub live_count: Option<i64>,
    /// How many instances the pool holds, running or not. This is what the reconciler converges toward desired_count and what `status` reflects, so member_count == desired_count with live_count below it means the pool has the members it was asked for and some of them are not up.
    #[serde(
        rename = "member_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub member_count: Option<i64>,
    /// True while a rolling replacement requested through POST /v1/instance-pools/{pool_id}/refresh is still running. It clears itself once every member is on the current template. The pool reads `scaling` for the duration, since it runs one instance over its target while a replacement comes up.
    #[serde(
        rename = "refresh_in_progress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_in_progress: Option<bool>,
    /// How many members were launched from a template other than the pool's current one — that is, how many a refresh would replace. Non-zero after editing `template` and before refreshing, which is the signal that a template change has not been rolled out yet.
    #[serde(
        rename = "stale_instance_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stale_instance_count: Option<i64>,
    /// Where the pool is against its target. `active` means member_count == desired_count — the pool holds the members it was asked for. It is not a claim that all of them are up; read live_count for that. `scaling` means it does not, and the reconciler is converging it: after a create, after a desired_count change, and for the length of an instance refresh, which runs the pool one instance over its target while a replacement comes up. `error` means an active error fault exists. Capacity failures remain eligible for reconciliation; failed deletion retains its teardown intent and never recreates members. `deleting` is teardown without an active error.
    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstancePoolStatus>,
    /// Active faults, newest first. Empty for a healthy pool. Recovery resolves only the successful operation's codes.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(
        rename = "managed_by",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_by: Option<String>,
    /// Labels on the POOL itself, for IAM conditions (`basalt:ResourceTag/<key>`) and cost attribution. They are attached to nothing else: no instance the pool launches carries them. The tags a replica is launched with are `template.tags`. Unlike the other top-level fields beside this one, `tags` is not a projection of the launch template — it is the pool's own set, and PATCHable on its own.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    /// The pool's launch config, in the shape instance create takes. The only place it appears: a flat copy of it beside this was two spellings of one thing, and two spellings drift.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<InstancePoolTemplate>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InstancePoolVpc {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RetiringPoolMember {
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

    #[serde(rename = "instance_id")]
    pub instance_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstancePoolStatus {
    Active,
    Scaling,
    Error,
    Deleting,
    Unknown(String),
}
impl InstancePoolStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Scaling => "scaling",
            Self::Error => "error",
            Self::Deleting => "deleting",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InstancePoolStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InstancePoolStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "active" => Self::Active,
            "scaling" => Self::Scaling,
            "error" => Self::Error,
            "deleting" => Self::Deleting,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolTemplate {
    /// Address families allocated on the member's sole interface. Fixed addresses are not allowed.
    #[serde(rename = "addresses", default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<AddressRequest>>,

    #[serde(rename = "security_groups")]
    pub security_groups: Vec<ResourceRef>,
    /// Per-member public floating IP assignment. Explicit families require addresses and internet routes in every selected subnet.
    #[serde(rename = "floating_ip_assignment")]
    pub floating_ip_assignment: InstancePoolTemplateFloatingIpAssignment,
    /// Base64-encoded user data (cloud-init), stamped on every replica.
    #[serde(rename = "user_data", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,

    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Tags stamped on every instance this template launches. These are the replicas' tags, not the pool's — the pool's own labels are the top-level `tags`, and the two are independent. Changing them affects FUTURE launches only. The instances already running keep the tags they were launched with, so between the change and a refresh the pool holds members carrying two different tag sets; `stale_instance_count` is how many are still on the old one. POST /v1/instance-pools/{pool_id}/refresh rolls them onto the current template.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    /// Role assigned to replicas, or null when none is configured.
    #[serde(rename = "iam_role")]
    pub iam_role: InstancePoolTemplateIamRole,
    /// Per-replica disks, the boot disk included — mark it with `boot: true`. Each new replica receives the configured provisioned performance. Omitted performance uses the included allowance. Existing volumes and snapshot schedules are not supported in pool templates.
    #[serde(rename = "volumes", default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<InstanceVolume>>,

    #[serde(rename = "flavor")]
    pub flavor: ResourceRef,
    /// Source image pinned until the template is replaced, or null when no image is selected.
    #[serde(rename = "image")]
    pub image: InstancePoolTemplateImage,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AddressRequest {
    #[serde(rename = "family")]
    pub family: AddressRequestFamily,
    /// Optional fixed address when creating an interface or instance NIC. For IPv6, use the first address of an aligned /96 inside the subnet /64 (last 32 bits zero); the first and last /96 ranges are reserved. Omit for automatic allocation. Managed database nodes and the add-address operation require automatic allocation.
    #[serde(rename = "address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddressRequestFamily {
    Ipv4,
    Ipv6,
    Unknown(String),
}
impl AddressRequestFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AddressRequestFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AddressRequestFamily {
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
pub enum InstancePoolTemplateFloatingIpAssignment {
    None,
    Ipv4,
    Ipv6,
    DualStack,
    Auto,
    Unknown(String),
}
impl InstancePoolTemplateFloatingIpAssignment {
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Ipv4 => "ipv4",
            Self::Ipv6 => "ipv6",
            Self::DualStack => "dual_stack",
            Self::Auto => "auto",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for InstancePoolTemplateFloatingIpAssignment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InstancePoolTemplateFloatingIpAssignment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "none" => Self::None,
            "ipv4" => Self::Ipv4,
            "ipv6" => Self::Ipv6,
            "dual_stack" => Self::DualStack,
            "auto" => Self::Auto,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InstancePoolTemplateIamRole {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceVolume {
    /// Marks the boot disk. It takes no mount_path or fstype — both come from the image — and sending either is refused rather than ignored.
    #[serde(rename = "boot", default, skip_serializing_if = "Option::is_none")]
    pub boot: Option<bool>,

    #[serde(rename = "size_gb")]
    pub size_gb: i64,
    /// Tier; omitted = the region default.
    #[serde(
        rename = "volume_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_type: Option<String>,

    #[serde(
        rename = "performance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub performance: Option<VolumePerformanceRequest>,

    #[serde(
        rename = "mount_path",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_path: Option<String>,
    /// Filesystem the in-guest agent formats the volume with.
    #[serde(rename = "fstype", default, skip_serializing_if = "Option::is_none")]
    pub fstype: Option<String>,
    /// Destroyed with the instance unless set false.
    #[serde(
        rename = "delete_on_termination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delete_on_termination: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VolumePerformanceRequest {
    #[serde(rename = "iops", default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,

    #[serde(
        rename = "throughput_mib_s",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub throughput_mib_s: Option<f64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum InstancePoolTemplateImage {
    Variant1(Box<ResourceRef>),
    Variant2(Box<crate::Nullable<serde_json::Value>>),
}

pub type CreateInstancePoolResponse = InstancePoolResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SerialConsoleTicket {
    /// The credential. Opaque — do not parse it. Good for one instance and one minute; mint a new one per connection rather than storing it.
    #[serde(rename = "ticket")]
    pub ticket: String,

    #[serde(rename = "expires_at")]
    pub expires_at: String,
    /// Seconds until it expires.
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
}

pub type CreateSerialConsoleTicketResponse = SerialConsoleTicket;

pub type DeleteImageResponse = ImageResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetConsoleOutputParameters {
    #[serde(rename = "max_bytes", default, skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<i64>,
}

pub type GetConsoleOutputQuery = GetConsoleOutputParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetConsoleOutputResult {
    /// The transcript as plain text, newlines included. Empty when the instance has not booted yet.
    #[serde(rename = "output")]
    pub output: String,
    /// True when the transcript was longer than the requested size and its BEGINNING was dropped to fit. The end is always kept.
    #[serde(rename = "truncated")]
    pub truncated: bool,
}

pub type GetConsoleOutputResponse = GetConsoleOutputResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetFlavorResult {
    #[serde(rename = "flavor")]
    pub flavor: Flavor,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Flavor {
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
    /// Number of virtual CPUs
    #[serde(rename = "vcpus")]
    pub vcpus: i64,
    /// RAM in MB
    #[serde(rename = "ram_mb")]
    pub ram_mb: i64,
    /// Host-pool routing. "shared" oversubscribes CPU for higher density; "dedicated" pins each vCPU 1:1 to a physical core.
    #[serde(rename = "class")]
    pub class: FlavorClass,
    /// Which product can book the flavor. "general" flavors are for regular instances and instance pools; "loadbalancer" and "database" flavors are reserved for the managed products (their nodes are platform- operated and priced accordingly) and cannot be used for regular instances.
    #[serde(rename = "family")]
    pub family: FlavorFamily,
    /// Aggregate instance network throughput limit in megabits per second. Omitted when uncapped.
    #[serde(rename = "net_mbps", default, skip_serializing_if = "Option::is_none")]
    pub net_mbps: Option<i64>,
    /// Guaranteed CPU floor as a percentage of each vCPU. Omitted when no floor is guaranteed.
    #[serde(
        rename = "cpu_baseline_pct",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cpu_baseline_pct: Option<i64>,
    /// CPU ceiling as a percentage of each vCPU. A value of 100 or an omitted field allows the full vCPU count.
    #[serde(
        rename = "cpu_burst_pct",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cpu_burst_pct: Option<i64>,
    /// Active when capacity can currently place this flavor; unavailable when no capacity is available. Disabled flavors are excluded from the public catalog.
    #[serde(rename = "status")]
    pub status: FlavorStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlavorClass {
    Shared,
    Dedicated,
    Unknown(String),
}
impl FlavorClass {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Shared => "shared",
            Self::Dedicated => "dedicated",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FlavorClass {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FlavorClass {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "shared" => Self::Shared,
            "dedicated" => Self::Dedicated,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlavorFamily {
    General,
    Loadbalancer,
    Database,
    Unknown(String),
}
impl FlavorFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::General => "general",
            Self::Loadbalancer => "loadbalancer",
            Self::Database => "database",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FlavorFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FlavorFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "general" => Self::General,
            "loadbalancer" => Self::Loadbalancer,
            "database" => Self::Database,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlavorStatus {
    Active,
    Unavailable,
    Unknown(String),
}
impl FlavorStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Unavailable => "unavailable",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FlavorStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FlavorStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "active" => Self::Active,
            "unavailable" => Self::Unavailable,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetFlavorResponse = GetFlavorResult;

pub type GetFlavorResource = Flavor;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetFlavorScope {
    #[serde(rename = "family", default, skip_serializing_if = "Option::is_none")]
    pub family: Option<GetFlavorScopeFamily>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetFlavorScopeFamily {
    General,
    Loadbalancer,
    Database,
    Unknown(String),
}
impl GetFlavorScopeFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::General => "general",
            Self::Loadbalancer => "loadbalancer",
            Self::Database => "database",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetFlavorScopeFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetFlavorScopeFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "general" => Self::General,
            "loadbalancer" => Self::Loadbalancer,
            "database" => Self::Database,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetImageResponse = ImageResponse;

pub type GetImageResource = Image;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetImageScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "os", default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(
        rename = "architecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetImageScopeStatus>,

    #[serde(
        rename = "all_versions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_versions: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GetImageScopeStatus {
    Pending,
    Importing,
    Active,
    Error,
    Deleting,
    Withdrawn,
    Unknown(String),
}
impl GetImageScopeStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Importing => "importing",
            Self::Active => "active",
            Self::Error => "error",
            Self::Deleting => "deleting",
            Self::Withdrawn => "withdrawn",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for GetImageScopeStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GetImageScopeStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "importing" => Self::Importing,
            "active" => Self::Active,
            "error" => Self::Error,
            "deleting" => Self::Deleting,
            "withdrawn" => Self::Withdrawn,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GetInstanceResult {
    #[serde(rename = "instance")]
    pub instance: Instance,
}

pub type GetInstanceResponse = GetInstanceResult;

pub type GetInstanceResource = Instance;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInstanceScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

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

pub type GetInstancePoolResponse = InstancePoolResponse;

pub type GetInstancePoolResource = InstancePool;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetInstancePoolScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListFlavorsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "family", default, skip_serializing_if = "Option::is_none")]
    pub family: Option<ListFlavorsParametersFamily>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListFlavorsParametersFamily {
    General,
    Loadbalancer,
    Database,
    Unknown(String),
}
impl ListFlavorsParametersFamily {
    pub fn as_str(&self) -> &str {
        match self {
            Self::General => "general",
            Self::Loadbalancer => "loadbalancer",
            Self::Database => "database",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListFlavorsParametersFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListFlavorsParametersFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "general" => Self::General,
            "loadbalancer" => Self::Loadbalancer,
            "database" => Self::Database,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListFlavorsQuery = ListFlavorsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FlavorListResponse {
    #[serde(rename = "flavors")]
    pub flavors: Vec<Flavor>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
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

pub type ListFlavorsResponse = FlavorListResponse;

pub type ListFlavorsItem = Flavor;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListImageCatalogParameters {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "os", default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(
        rename = "architecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture: Option<String>,
}

pub type ListImageCatalogQuery = ListImageCatalogParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageCatalogResponse {
    #[serde(rename = "categories")]
    pub categories: Vec<ImageCatalogCategory>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageCatalogCategory {
    /// Catalog category. Currently platform or account; future categories may include apps.
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "images")]
    pub images: Vec<CatalogImage>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CatalogImage {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "os", default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(
        rename = "os_version",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub os_version: Option<String>,

    #[serde(rename = "architecture")]
    pub architecture: String,

    #[serde(rename = "min_disk_gb")]
    pub min_disk_gb: i64,

    #[serde(rename = "min_ram_mb")]
    pub min_ram_mb: i64,

    #[serde(rename = "eol_date", default, skip_serializing_if = "Option::is_none")]
    pub eol_date: Option<String>,
}

pub type ListImageCatalogResponse = ImageCatalogResponse;

pub type ListImageCatalogItem = ImageCatalogCategory;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListImagesParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "os", default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(
        rename = "architecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ListImagesParametersStatus>,

    #[serde(
        rename = "all_versions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub all_versions: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListImagesParametersStatus {
    Pending,
    Importing,
    Active,
    Error,
    Deleting,
    Withdrawn,
    Unknown(String),
}
impl ListImagesParametersStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Importing => "importing",
            Self::Active => "active",
            Self::Error => "error",
            Self::Deleting => "deleting",
            Self::Withdrawn => "withdrawn",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListImagesParametersStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListImagesParametersStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "importing" => Self::Importing,
            "active" => Self::Active,
            "error" => Self::Error,
            "deleting" => Self::Deleting,
            "withdrawn" => Self::Withdrawn,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListImagesQuery = ListImagesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageListResponse {
    #[serde(rename = "images")]
    pub images: Vec<Image>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListImagesResponse = ImageListResponse;

pub type ListImagesItem = Image;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInstanceInterfacesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListInstanceInterfacesQuery = ListInstanceInterfacesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceInterfaceListResponse {
    #[serde(rename = "interfaces")]
    pub interfaces: Vec<AttachedInterface>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInstanceInterfacesResponse = InstanceInterfaceListResponse;

pub type ListInstanceInterfacesItem = AttachedInterface;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInstancePoolFloatingIpsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListInstancePoolFloatingIpsQuery = ListInstancePoolFloatingIpsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FloatingIpListResponse {
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<FloatingIp>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInstancePoolFloatingIpsResponse = FloatingIpListResponse;

pub type ListInstancePoolFloatingIpsItem = FloatingIp;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInstancePoolsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

pub type ListInstancePoolsQuery = ListInstancePoolsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolListResponse {
    #[serde(rename = "instance_pools")]
    pub instance_pools: Vec<InstancePool>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInstancePoolsResponse = InstancePoolListResponse;

pub type ListInstancePoolsItem = InstancePool;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInstanceVolumesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListInstanceVolumesQuery = ListInstanceVolumesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceVolumeListResponse {
    #[serde(rename = "volumes")]
    pub volumes: Vec<AttachedVolume>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInstanceVolumesResponse = InstanceVolumeListResponse;

pub type ListInstanceVolumesItem = AttachedVolume;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListInstancesParameters {
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

pub type ListInstancesQuery = ListInstancesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstanceListResponse {
    #[serde(rename = "instances")]
    pub instances: Vec<Instance>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListInstancesResponse = InstanceListResponse;

pub type ListInstancesItem = Instance;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListPoolInstancesParameters {
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

pub type ListPoolInstancesQuery = ListPoolInstancesParameters;

pub type ListPoolInstancesResponse = InstanceListResponse;

pub type ListPoolInstancesItem = Instance;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InstanceRebootRequestInput {
    /// Force a power cycle (destroy + start, equivalent to a reset button) instead of the default ACPI graceful reboot the guest can act on.
    #[serde(rename = "hard", default, skip_serializing_if = "Option::is_none")]
    pub hard: Option<bool>,
}

pub type RebootInstanceBody = InstanceRebootRequestInput;

pub type RefreshInstancePoolResponse = InstancePoolResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ReinstallInstanceRequest {
    /// Replacement image reference (UUID, architecture-qualified CRN, name or name:version). Omit to reinstall from the instance's current image.
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Replacement boot disk size; omitted = the image's min_disk_gb. Must be within the volume size range (1..16384) and at least the image's min_disk_gb.
    #[serde(rename = "size_gb", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i64>,
    /// Replacement boot disk tier; omitted = the region default.
    #[serde(
        rename = "volume_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_type: Option<ReinstallInstanceRequestVolumeType>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReinstallInstanceRequestVolumeType {
    Ssd,
    Nvme,
    Unknown(String),
}
impl ReinstallInstanceRequestVolumeType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ssd => "ssd",
            Self::Nvme => "nvme",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ReinstallInstanceRequestVolumeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReinstallInstanceRequestVolumeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ssd" => Self::Ssd,
            "nvme" => Self::Nvme,
            _ => Self::Unknown(value),
        })
    }
}

pub type ReinstallInstanceBody = ReinstallInstanceRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ResizeInstanceRequest {
    /// Regional flavor reference (UUID, CRN or exact name) to resize to.
    #[serde(rename = "flavor")]
    pub flavor: String,
}
impl ResizeInstanceRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(flavor: String) -> Self {
        Self { flavor }
    }
}

pub type ResizeInstanceBody = ResizeInstanceRequest;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct StartSerialConsoleParameters {
    #[serde(
        rename = "backlog_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backlog_bytes: Option<i64>,
}

pub type StartSerialConsoleQuery = StartSerialConsoleParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ImageUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Switch the resolve-by-name pointer for this image's name. true promotes this version to current (the switch / rollback action) and demotes whatever else was current for the same (name, architecture); false clears the pointer. Only active images can be made current.
    #[serde(rename = "current", default, skip_serializing_if = "Option::is_none")]
    pub current: Option<bool>,
    /// Set the release's end-of-life date. An explicit null clears it; omitting the field leaves it unchanged. Clearing matters because the catalog withdraws platform images on this date — one recorded by mistake has to be removable.
    #[serde(
        rename = "eol_date",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub eol_date: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
}

pub type UpdateImageBody = ImageUpdateRequestInput;

pub type UpdateImageResponse = ImageResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InstanceUpdateRequestInput {
    /// Attach or replace the instance workload role using its ID, name, or CRN. Omit this field to keep the current role; send an empty string to detach it. Null is not accepted. Requires compute:UpdateInstance; attach/replace also require iam:PassRole and a role trust policy allowing this instance. Only running or stopped customer-managed instances with no operation in progress support role edits. Pool members use the pool launch template. New IMDS requests observe the committed association immediately. Previously issued credentials are not revoked and remain valid until expiry (up to one hour); in-flight requests may complete with their prior association.
    #[serde(rename = "iam_role", default, skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateInstanceBody = InstanceUpdateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateInstanceResult {
    #[serde(rename = "instance")]
    pub instance: Instance,
}

pub type UpdateInstanceResponse = UpdateInstanceResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct InstancePoolUpdateRequestInput {
    /// Exact member UUIDs to retire. Requires an explicit lower desired_count and exactly current desired_count minus new desired_count members. The pool must have converged membership and no retirement or refresh in progress. Selected members must belong to this pool and be unprotected. Only sizing and autoscaling fields may accompany selection. Selection and sizing are committed atomically; stale or protected selections return 409 without substituting another member. Use Idempotency-Key when retrying.
    #[serde(
        rename = "scale_in_instances",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scale_in_instances: Option<Vec<String>>,
    /// Replace the eligible placement set. Omit to preserve it. Affects future launches; run an explicit refresh to move existing members. All selected subnets must share a VPC.
    #[serde(rename = "subnets", default, skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,

    #[serde(
        rename = "autoscaling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub autoscaling: Option<AutoscalingPolicyInput>,
    /// Customer note on the pool. Omit to preserve it; send an empty string to clear it. Changes no instances, sizing or launch configuration.
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// REPLACES the pool's labels: the map you send becomes the whole set, an empty object clears them, and omitting the field leaves them alone. Replacement rather than a merge because a merge leaves no way to say a key should be removed. These label the pool, not its instances. To change what future replicas are tagged with, send `template.tags`.
    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
    /// New target size, bounded by the resulting min_count/max_count and the hard platform cap of 100.
    #[serde(
        rename = "desired_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_count: Option<i64>,
    /// New lower bound; omitted desired_count rises to this bound if needed.
    #[serde(rename = "min_count", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i64>,
    /// New upper bound; omitted desired_count falls to this bound if needed. A value of 0 means the pool holds no members until max_count is raised.
    #[serde(rename = "max_count", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
    /// Replaces the launch config WHOLESALE — the object you send is what the pool launches next, and anything you leave out is cleared rather than kept. Replacement rather than a deep merge so a shorter `networks` or `volumes` cannot be read as a truncation and silently drop an interface or a disk.
    #[serde(rename = "template", default, skip_serializing_if = "Option::is_none")]
    pub template: Option<InstancePoolTemplateRequestInput>,
}

pub type UpdateInstancePoolBody = InstancePoolUpdateRequestInput;

pub type UpdateInstancePoolResponse = InstancePoolResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InstancePoolMemberProtectionRequestInput {
    /// Protect this member from manual and automatic scale-in. Remove protection before selecting it for scale-in. Cannot change a retiring member. Explicit refresh, failed-instance replacement, instance deletion and pool deletion are unaffected. Replacement instances start unprotected.
    #[serde(rename = "scale_in_protected")]
    pub scale_in_protected: bool,
}
impl InstancePoolMemberProtectionRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(scale_in_protected: bool) -> Self {
        Self { scale_in_protected }
    }
}

pub type UpdateInstancePoolMemberProtectionBody = InstancePoolMemberProtectionRequestInput;

pub type UpdateInstancePoolMemberProtectionResponse = InstancePoolResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateInstanceVolumeAttachmentRequest {
    #[serde(rename = "delete_on_termination")]
    pub delete_on_termination: bool,
}
impl UpdateInstanceVolumeAttachmentRequest {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(delete_on_termination: bool) -> Self {
        Self {
            delete_on_termination,
        }
    }
}

pub type UpdateInstanceVolumeAttachmentBody = UpdateInstanceVolumeAttachmentRequest;
