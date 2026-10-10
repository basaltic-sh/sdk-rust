//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CompleteMultipartUploadRequestInput {
    /// Every part the assembled object is made of, in ascending part_number order. Each etag must match the one that part's upload returned.
    #[serde(rename = "parts")]
    pub parts: Vec<CompleteMultipartUploadRequestInputPartsItem>,
}
impl CompleteMultipartUploadRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(parts: Vec<CompleteMultipartUploadRequestInputPartsItem>) -> Self {
        Self { parts }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CompleteMultipartUploadRequestInputPartsItem {
    #[serde(rename = "part_number")]
    pub part_number: i64,

    #[serde(rename = "etag")]
    pub etag: String,
}
impl CompleteMultipartUploadRequestInputPartsItem {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(part_number: i64, etag: String) -> Self {
        Self { part_number, etag }
    }
}

pub type CompleteMultipartUploadBody = CompleteMultipartUploadRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CompleteMultipartUploadResponse2 {
    #[serde(rename = "etag")]
    pub etag: String,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(
        rename = "version_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_id: Option<String>,

    #[serde(
        rename = "storage_class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_class: Option<String>,
}

pub type CompleteMultipartUploadResponse = CompleteMultipartUploadResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateBucketRequestInput {
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// When true, enables S3 Object Lock on the bucket at creation time and turns versioning on. Object Lock cannot be enabled later.
    #[serde(
        rename = "object_lock_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub object_lock_enabled: Option<bool>,
}
impl CreateBucketRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            object_lock_enabled: None,
        }
    }
}

pub type CreateBucketBody = CreateBucketRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BucketResponse {
    #[serde(rename = "bucket", default, skip_serializing_if = "Option::is_none")]
    pub bucket: Option<Bucket>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Bucket {
    #[serde(rename = "id")]
    pub id: String,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "acl")]
    pub acl: String,
    /// Versioning state. `suspended` means versioning was on and was turned off — existing versions are kept, new writes stop creating them — which is distinct from `disabled`, a bucket that never had it enabled. The S3-compatible endpoint spells the same states `Enabled` / `Suspended` in its XML, and reports `disabled` by omitting the element.
    #[serde(rename = "versioning")]
    pub versioning: BucketVersioning,
    /// When true, DeleteBucket schedules deletion instead of removing the bucket immediately. Default false deliberately preserves S3 immediate deletion.
    #[serde(rename = "deletion_protection")]
    pub deletion_protection: bool,
    /// Configured recovery window in whole days (default 7). New writes require 1–30; disabled historical buckets can retain legacy out-of-range values.
    #[serde(rename = "recovery_window_days")]
    pub recovery_window_days: i64,
    /// When deletion was requested; null when not pending or for historical windows.
    #[serde(rename = "deleted_at")]
    pub deleted_at: crate::Nullable<String>,
    /// Purge deadline, present while deletion is pending. Restore before this time to cancel.
    #[serde(
        rename = "scheduled_purge_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_purge_at: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BucketVersioning {
    Disabled,
    Enabled,
    Suspended,
    Unknown(String),
}
impl BucketVersioning {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Disabled => "disabled",
            Self::Enabled => "enabled",
            Self::Suspended => "suspended",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for BucketVersioning {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BucketVersioning {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "disabled" => Self::Disabled,
            "enabled" => Self::Enabled,
            "suspended" => Self::Suspended,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateBucketResponse = BucketResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SnapshotCreateRequestInput {
    /// Account-owned volume UUID, CRN, or exact name.
    #[serde(rename = "volume")]
    pub volume: String,
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
    pub tags: Option<TagsInput>,
}
impl SnapshotCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(volume: String, name: String) -> Self {
        Self {
            volume,
            name,
            description: None,
            tags: None,
        }
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateSnapshotBody = SnapshotCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SnapshotResponse {
    #[serde(rename = "snapshot", default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Snapshot {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name (name-based, region-scoped).
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// Source volume the snapshot was taken from.
    #[serde(rename = "volume_id", default, skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// The snapshot policy that took this snapshot. Absent when a person did. This is also what retention matches on, so its presence is what makes a snapshot eligible for automatic deletion — snapshots taken by hand are never reaped.
    #[serde(
        rename = "snapshot_policy_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_policy_id: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    /// Frozen size of the source volume at the time the snapshot was taken — the volume may have been extended since.
    #[serde(rename = "size_gb", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i64>,
    /// Frozen logical restore capacity in bytes (size_gb multiplied by 2^30), not measured written data.
    #[serde(
        rename = "logical_size_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logical_size_bytes: Option<i64>,

    #[serde(
        rename = "snapshot_usage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_usage: Option<SnapshotUsage>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SnapshotStatus>,
    /// Active faults, newest first. Empty when healthy. Status is error if and only if an active error-severity fault exists. Resolved history is retained internally.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,

    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

pub type Tags = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SnapshotUsage {
    /// Measured means a complete observation with a matching current catalog generation and age at most 90 minutes. Stale means its generation changed or it expired. This is a last-observed value, not continuous backend verification. Unknown and stale never imply zero usage.
    #[serde(rename = "state")]
    pub state: SnapshotUsageState,

    #[serde(rename = "scope")]
    pub scope: SnapshotUsageScope,
    /// These observations do not produce charges.
    #[serde(rename = "billable")]
    pub billable: bool,
    /// Observation timestamp for measured or stale data; null when unknown.
    #[serde(rename = "measured_at")]
    pub measured_at: crate::Nullable<String>,
    /// Exact retained lineage bytes only when measured; null when unknown or stale. An explicit measured zero is distinct from unavailable data.
    #[serde(rename = "lineage_retained_bytes")]
    pub lineage_retained_bytes: crate::Nullable<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotUsageState {
    Unknown1,
    Stale,
    Measured,
    Unknown(String),
}
impl SnapshotUsageState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Unknown1 => "unknown",
            Self::Stale => "stale",
            Self::Measured => "measured",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SnapshotUsageState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotUsageState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "unknown" => Self::Unknown1,
            "stale" => Self::Stale,
            "measured" => Self::Measured,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotUsageScope {
    VolumeLineage,
    Unknown(String),
}
impl SnapshotUsageScope {
    pub fn as_str(&self) -> &str {
        match self {
            Self::VolumeLineage => "volume_lineage",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SnapshotUsageScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotUsageScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "volume_lineage" => Self::VolumeLineage,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotStatus {
    Creating,
    Available,
    Deleting,
    Error,
    Unknown(String),
}
impl SnapshotStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Creating => "creating",
            Self::Available => "available",
            Self::Deleting => "deleting",
            Self::Error => "error",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SnapshotStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "creating" => Self::Creating,
            "available" => Self::Available,
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

pub type CreateSnapshotResponse = SnapshotResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SnapshotPolicyCreateRequestInput {
    /// Account-owned volume UUID, CRN, or exact name.
    #[serde(rename = "volume")]
    pub volume: String,
    /// Unique within the account — it names the policy in its CRN. Scheduled snapshots are named `<policy>-<UTC timestamp>`. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
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
    /// Defaults to true. Set false to attach a paused schedule. Pausing stops the whole policy — no snapshots are taken and none are deleted, because a paused schedule that kept reaping would delete history while you were looking at it.
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl SnapshotPolicyCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        volume: String,
        name: String,
        interval_minutes: SnapshotIntervalMinutesInput,
        retention_count: SnapshotRetentionCountInput,
    ) -> Self {
        Self {
            volume,
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

pub type CreateSnapshotPolicyBody = SnapshotPolicyCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SnapshotPolicyResponse {
    #[serde(
        rename = "snapshot_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_policy: Option<SnapshotPolicy>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SnapshotPolicy {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name (name-based, region-scoped).
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// The volume this schedule is attached to. A volume supports up to 16 independent policies.
    #[serde(rename = "volume_id", default, skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// Disabling pauses the whole policy — no scheduled snapshots and no retention. A paused schedule that kept reaping would delete history while you were looking at it.
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(
        rename = "interval_minutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_minutes: Option<SnapshotIntervalMinutes>,

    #[serde(
        rename = "retention_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_count: Option<SnapshotRetentionCount>,

    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_days: Option<SnapshotRetentionDays>,
    /// When the next snapshot is due. Re-stamped to `now + interval_minutes` each time the policy fires — never to `previous + interval` — so a window missed while the region was busy costs one snapshot, not one per window missed.
    #[serde(
        rename = "next_run_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_run_at: Option<String>,
    /// When the policy last fired. Absent until the first fire.
    #[serde(
        rename = "last_run_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_run_at: Option<String>,
    /// Active warning faults, newest first; empty when healthy. Policies have no error status and faults never change enabled. Execution and retention recover independently; resolved history is retained internally.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,

    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

pub type SnapshotIntervalMinutes = i64;

pub type SnapshotRetentionCount = i64;

pub type SnapshotRetentionDays = i64;

pub type CreateSnapshotPolicyResponse = SnapshotPolicyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumeCreateRequestInput {
    #[serde(
        rename = "performance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub performance: Option<VolumePerformanceRequestInput>,
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
    pub tags: Option<TagsInput>,

    #[serde(rename = "volume_type")]
    pub volume_type: CreatableVolumeTypeNameInput,

    #[serde(rename = "size_gb")]
    pub size_gb: i64,
    /// Architecture used for source_image name resolution. A full image CRN pins its own architecture.
    #[serde(
        rename = "architecture",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub architecture: Option<String>,
    /// Image UUID, name, name:version, or full CRN image/&lt;name&gt;/architecture/&lt;arch&gt;/version/&lt;version&gt;. Names resolve in the caller account first, then tagged platform catalog images, using architecture (default amd64). Mutually exclusive with source_snapshot. Image tags are not accepted.
    #[serde(
        rename = "source_image",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_image: Option<String>,
    /// Clone from an available account-owned snapshot UUID or nested CRN volume/&lt;volume-name&gt;/snapshot/&lt;snapshot-name&gt;. Bare snapshot names are rejected because this request has no fixed source volume. Mutually exclusive with source_image; size_gb must be at least the snapshot's frozen size.
    #[serde(
        rename = "source_snapshot",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_snapshot: Option<String>,

    #[serde(rename = "bootable", default, skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
}
impl VolumeCreateRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String, volume_type: CreatableVolumeTypeNameInput, size_gb: i64) -> Self {
        Self {
            performance: None,
            name,
            description: None,
            tags: None,
            volume_type,
            size_gb,
            architecture: None,
            source_image: None,
            source_snapshot: None,
            bootable: None,
        }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreatableVolumeTypeNameInput {
    Ssd,
    Nvme,
    Unknown(String),
}
impl CreatableVolumeTypeNameInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ssd => "ssd",
            Self::Nvme => "nvme",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CreatableVolumeTypeNameInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreatableVolumeTypeNameInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "ssd" => Self::Ssd,
            "nvme" => Self::Nvme,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateVolumeBody = VolumeCreateRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VolumeResponse {
    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Volume {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Cloud Resource Name (name-based, region-scoped).
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

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(
        rename = "volume_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_type: Option<VolumeTypeName>,

    #[serde(rename = "size_gb", default, skip_serializing_if = "Option::is_none")]
    pub size_gb: Option<i64>,

    #[serde(
        rename = "performance",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub performance: Option<VolumePerformance>,

    #[serde(
        rename = "included_io_limits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub included_io_limits: Option<IncludedIOLimits>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VolumeStatus>,
    /// Set at create time when the volume is provisioned from a bootable image. Immutable after creation.
    #[serde(rename = "bootable", default, skip_serializing_if = "Option::is_none")]
    pub bootable: Option<bool>,
    /// Image the volume was cloned from, when applicable.
    #[serde(
        rename = "source_image_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub source_image_id: Option<crate::Nullable<String>>,
    /// Snapshot the volume was cloned from (restore path), when applicable.
    #[serde(
        rename = "source_snapshot_id",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub source_snapshot_id: Option<crate::Nullable<String>>,
    /// Active faults, newest first. Empty when healthy. Status is error if and only if an active error-severity fault exists. Resolved history is retained internally.
    #[serde(rename = "faults")]
    pub faults: Vec<Fault>,

    #[serde(
        rename = "created_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<String>,

    #[serde(
        rename = "updated_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<String>,
}

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

    #[serde(rename = "state")]
    pub state: VolumePerformanceState,

    #[serde(
        rename = "operation_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operation_id: Option<String>,

    #[serde(
        rename = "applied_at",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub applied_at: Option<String>,
    /// Latest retryable failure; the accepted request remains durable.
    #[serde(
        rename = "last_error",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_error: Option<String>,
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
pub enum VolumePerformanceState {
    Creating,
    Pending,
    Applied,
    Unknown(String),
}
impl VolumePerformanceState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Creating => "creating",
            Self::Pending => "pending",
            Self::Applied => "applied",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for VolumePerformanceState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VolumePerformanceState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "creating" => Self::Creating,
            "pending" => Self::Pending,
            "applied" => Self::Applied,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VolumeStatus {
    Creating,
    Available,
    InUse,
    Extending,
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
            "deleting" => Self::Deleting,
            "error" => Self::Error,
            _ => Self::Unknown(value),
        })
    }
}

pub type CreateVolumeResponse = VolumeResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteBucketResultVariant1 {
    #[serde(rename = "scheduled_purge_at")]
    pub scheduled_purge_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum DeleteBucketResult {
    Variant1(Box<DeleteBucketResultVariant1>),
    Variant2(Box<std::collections::BTreeMap<String, serde_json::Value>>),
}

pub type DeleteBucketResponse = DeleteBucketResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumeExtendRequestInput {
    /// New size in GB. Must be strictly greater than the current size.
    #[serde(rename = "new_size_gb")]
    pub new_size_gb: i64,
}
impl VolumeExtendRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(new_size_gb: i64) -> Self {
        Self { new_size_gb }
    }
}

pub type ExtendVolumeBody = VolumeExtendRequestInput;

pub type ExtendVolumeResponse = VolumeResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BucketCORSResponse {
    #[serde(rename = "cors")]
    pub cors: CORSConfig,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CORSConfig {
    #[serde(rename = "rules")]
    pub rules: Vec<CORSRule>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CORSRule {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "allowed_origins")]
    pub allowed_origins: Vec<String>,

    #[serde(rename = "allowed_methods")]
    pub allowed_methods: Vec<CORSRuleAllowedMethodsItem>,

    #[serde(
        rename = "allowed_headers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_headers: Option<Vec<String>>,

    #[serde(
        rename = "expose_headers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expose_headers: Option<Vec<String>>,

    #[serde(
        rename = "max_age_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_age_seconds: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CORSRuleAllowedMethodsItem {
    GET,
    PUT,
    POST,
    DELETE,
    HEAD,
    Unknown(String),
}
impl CORSRuleAllowedMethodsItem {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GET => "GET",
            Self::PUT => "PUT",
            Self::POST => "POST",
            Self::DELETE => "DELETE",
            Self::HEAD => "HEAD",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CORSRuleAllowedMethodsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CORSRuleAllowedMethodsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "GET" => Self::GET,
            "PUT" => Self::PUT,
            "POST" => Self::POST,
            "DELETE" => Self::DELETE,
            "HEAD" => Self::HEAD,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetBucketCORSResponse = BucketCORSResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BucketEncryptionResponse {
    #[serde(rename = "encryption")]
    pub encryption: EncryptionConfig,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfig {
    #[serde(rename = "rules")]
    pub rules: Vec<EncryptionRule>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct EncryptionRule {
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default: Option<EncryptionRuleDefault>,

    #[serde(
        rename = "bucket_key_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bucket_key_enabled: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionRuleDefault {
    #[serde(rename = "sse_algorithm")]
    pub sse_algorithm: String,
    /// Accepted but unused S3 placeholder. Only AES256 is supported; this is not a KMS resource reference.
    #[serde(
        rename = "kms_master_key_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_master_key_id: Option<String>,
}

pub type GetBucketEncryptionResponse = BucketEncryptionResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BucketLifecycleResponse {
    /// Opaque configuration revision. Supply this value in double quotes as If-Match on PUT or DELETE. The empty configuration has revision none.
    #[serde(rename = "revision")]
    pub revision: String,

    #[serde(rename = "lifecycle")]
    pub lifecycle: LifecycleConfig,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleConfig {
    #[serde(rename = "rules")]
    pub rules: Vec<LifecycleRule>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleRule {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `disabled` keeps the rule in the configuration but skips it during evaluation. The S3-compatible endpoint spells the same states `Enabled` / `Disabled` in its XML.
    #[serde(rename = "status")]
    pub status: LifecycleRuleStatus,

    #[serde(rename = "filter", default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<LifecycleRuleFilter>,
    /// Move matching objects to another storage class once they are old enough. The object keeps its identity — same key, same version id, same last-modified — and only its bytes move between pools, so pairing a transition with an expiration works: the expiry clock is not restarted by the move. Exactly one of `days` or `date`. When the rule also has an `expiration`, the transition must come strictly first, otherwise the object would be deleted before it ever moved and the rule is rejected. Only one transition per rule: the platform serves two classes, so a second has nowhere to go. The S3-compatible endpoint accepts a single-element `<Transition>` list and rejects longer ones rather than silently applying the first.
    #[serde(
        rename = "transition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transition: Option<LifecycleRuleTransition>,

    #[serde(
        rename = "expiration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration: Option<LifecycleRuleExpiration>,

    #[serde(
        rename = "noncurrent_version_expiration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub noncurrent_version_expiration: Option<LifecycleRuleNoncurrentVersionExpiration>,

    #[serde(
        rename = "abort_incomplete_multipart_upload",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub abort_incomplete_multipart_upload: Option<LifecycleRuleAbortIncompleteMultipartUpload>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleRuleStatus {
    Enabled,
    Disabled,
    Unknown(String),
}
impl LifecycleRuleStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleRuleStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRuleStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleFilter {
    #[serde(rename = "prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleRuleTransition {
    /// Days after the object's last-modified time.
    #[serde(rename = "days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Absolute cut-off; fires on the next sweep after this instant.
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Destination class. Keeps the S3 standard's casing rather than the platform's lowercase convention, because the class vocabulary is S3's. Unsupported values are rejected — a transition to a class that does not exist would silently never run.
    #[serde(rename = "storage_class")]
    pub storage_class: LifecycleRuleTransitionStorageClass,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleRuleTransitionStorageClass {
    STANDARD,
    COLD,
    Unknown(String),
}
impl LifecycleRuleTransitionStorageClass {
    pub fn as_str(&self) -> &str {
        match self {
            Self::STANDARD => "STANDARD",
            Self::COLD => "COLD",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleRuleTransitionStorageClass {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRuleTransitionStorageClass {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "STANDARD" => Self::STANDARD,
            "COLD" => Self::COLD,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleExpiration {
    #[serde(rename = "days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,

    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleNoncurrentVersionExpiration {
    #[serde(
        rename = "noncurrent_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub noncurrent_days: Option<i64>,

    #[serde(
        rename = "newer_noncurrent_versions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub newer_noncurrent_versions: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleAbortIncompleteMultipartUpload {
    #[serde(
        rename = "days_after_initiation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_after_initiation: Option<i64>,
}

pub type GetBucketLifecycleResponse = BucketLifecycleResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BucketObjectLockResponse {
    #[serde(rename = "object_lock")]
    pub object_lock: ObjectLockConfig,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ObjectLockConfig {
    #[serde(
        rename = "object_lock_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub object_lock_enabled: Option<String>,

    #[serde(rename = "rule", default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<ObjectLockConfigRule>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ObjectLockConfigRule {
    #[serde(
        rename = "default_retention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_retention: Option<ObjectLockConfigRuleDefaultRetention>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ObjectLockConfigRuleDefaultRetention {
    #[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectLockConfigRuleDefaultRetentionMode>,

    #[serde(rename = "days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,

    #[serde(rename = "years", default, skip_serializing_if = "Option::is_none")]
    pub years: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectLockConfigRuleDefaultRetentionMode {
    GOVERNANCE,
    COMPLIANCE,
    Unknown(String),
}
impl ObjectLockConfigRuleDefaultRetentionMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GOVERNANCE => "GOVERNANCE",
            Self::COMPLIANCE => "COMPLIANCE",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ObjectLockConfigRuleDefaultRetentionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ObjectLockConfigRuleDefaultRetentionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "GOVERNANCE" => Self::GOVERNANCE,
            "COMPLIANCE" => Self::COMPLIANCE,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetBucketObjectLockResponse = BucketObjectLockResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BucketPolicyResponse {
    #[serde(rename = "document", default, skip_serializing_if = "Option::is_none")]
    pub document: Option<BucketPolicy>,
}

pub type BucketPolicy = std::collections::BTreeMap<String, serde_json::Value>;

pub type GetBucketPolicyResponse = BucketPolicyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BucketTaggingResponse {
    #[serde(rename = "tags")]
    pub tags: TagSet,
}

pub type TagSet = std::collections::BTreeMap<String, String>;

pub type GetBucketTaggingResponse = BucketTaggingResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BucketVersioningResponse {
    #[serde(rename = "status")]
    pub status: BucketVersioningResponseStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BucketVersioningResponseStatus {
    Disabled,
    Enabled,
    Suspended,
    Unknown(String),
}
impl BucketVersioningResponseStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Disabled => "disabled",
            Self::Enabled => "enabled",
            Self::Suspended => "suspended",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for BucketVersioningResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BucketVersioningResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "disabled" => Self::Disabled,
            "enabled" => Self::Enabled,
            "suspended" => Self::Suspended,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetBucketVersioningResponse = BucketVersioningResponse;

pub type GetSnapshotResponse = SnapshotResponse;

pub type GetSnapshotResource = Snapshot;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetSnapshotScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SnapshotStatusInput>,

    #[serde(
        rename = "snapshot_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_policy: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotStatusInput {
    Creating,
    Available,
    Deleting,
    Error,
    Unknown(String),
}
impl SnapshotStatusInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Creating => "creating",
            Self::Available => "available",
            Self::Deleting => "deleting",
            Self::Error => "error",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SnapshotStatusInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotStatusInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "creating" => Self::Creating,
            "available" => Self::Available,
            "deleting" => Self::Deleting,
            "error" => Self::Error,
            _ => Self::Unknown(value),
        })
    }
}

pub type GetSnapshotPolicyResponse = SnapshotPolicyResponse;

pub type GetSnapshotPolicyResource = SnapshotPolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetSnapshotPolicyScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

pub type GetVolumeResponse = VolumeResponse;

pub type GetVolumeResource = Volume;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct GetVolumeScope {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VolumeStatusInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VolumeStatusInput {
    Creating,
    Available,
    InUse,
    Extending,
    Deleting,
    Error,
    Unknown(String),
}
impl VolumeStatusInput {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Creating => "creating",
            Self::Available => "available",
            Self::InUse => "in_use",
            Self::Extending => "extending",
            Self::Deleting => "deleting",
            Self::Error => "error",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for VolumeStatusInput {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VolumeStatusInput {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "creating" => Self::Creating,
            "available" => Self::Available,
            "in_use" => Self::InUse,
            "extending" => Self::Extending,
            "deleting" => Self::Deleting,
            "error" => Self::Error,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InitiateMultipartUploadRequestInput {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(
        rename = "content_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,

    #[serde(
        rename = "storage_class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_class: Option<String>,

    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
}
impl InitiateMultipartUploadRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(key: String) -> Self {
        Self {
            key,
            content_type: None,
            storage_class: None,
            metadata: None,
        }
    }
}

pub type InitiateMultipartUploadBody = InitiateMultipartUploadRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MultipartUploadResponse {
    #[serde(rename = "upload")]
    pub upload: MultipartUpload,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MultipartUpload {
    #[serde(rename = "upload_id")]
    pub upload_id: String,

    #[serde(rename = "bucket")]
    pub bucket: String,

    #[serde(rename = "key")]
    pub key: String,

    #[serde(
        rename = "content_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,

    #[serde(
        rename = "storage_class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_class: Option<String>,

    #[serde(rename = "metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

pub type InitiateMultipartUploadResponse = MultipartUploadResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListBucketsParameters {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListBucketsQuery = ListBucketsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct BucketListResponse {
    #[serde(rename = "buckets", default, skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<Bucket>>,

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

pub type ListBucketsResponse = BucketListResponse;

pub type ListBucketsItem = Bucket;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListMultipartUploadsParameters {
    #[serde(rename = "prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(
        rename = "max_uploads",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_uploads: Option<i64>,
}

pub type ListMultipartUploadsQuery = ListMultipartUploadsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListMultipartUploadsResponse2 {
    #[serde(rename = "uploads")]
    pub uploads: Vec<MultipartUpload>,
}

pub type ListMultipartUploadsResponse = ListMultipartUploadsResponse2;

pub type ListMultipartUploadsItem = MultipartUpload;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListObjectVersionsParameters {
    #[serde(rename = "prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(
        rename = "key_marker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_marker: Option<String>,

    #[serde(
        rename = "version_id_marker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_id_marker: Option<String>,

    #[serde(rename = "max_keys", default, skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
}

pub type ListObjectVersionsQuery = ListObjectVersionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListObjectVersionsResponse2 {
    #[serde(rename = "versions")]
    pub versions: Vec<ObjectVersion>,

    #[serde(rename = "is_truncated")]
    pub is_truncated: bool,

    #[serde(
        rename = "next_key_marker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_key_marker: Option<String>,

    #[serde(
        rename = "next_version_id_marker",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub next_version_id_marker: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ObjectVersion {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "version_id")]
    pub version_id: String,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "etag")]
    pub etag: String,

    #[serde(
        rename = "content_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,

    #[serde(
        rename = "storage_class",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_class: Option<String>,

    #[serde(rename = "last_modified")]
    pub last_modified: String,

    #[serde(rename = "is_latest")]
    pub is_latest: bool,

    #[serde(rename = "is_delete_marker")]
    pub is_delete_marker: bool,
}

pub type ListObjectVersionsResponse = ListObjectVersionsResponse2;

pub type ListObjectVersionsItem = ObjectVersion;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListObjectsParameters {
    #[serde(rename = "prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(rename = "delimiter", default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "max_keys", default, skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i64>,
}

pub type ListObjectsQuery = ListObjectsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ObjectListResponse {
    #[serde(rename = "objects", default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<Vec<ObjectEntry>>,

    #[serde(
        rename = "common_prefixes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub common_prefixes: Option<Vec<String>>,

    #[serde(
        rename = "is_truncated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_truncated: Option<bool>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ObjectEntry {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "etag")]
    pub etag: String,

    #[serde(
        rename = "content_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,

    #[serde(rename = "last_modified")]
    pub last_modified: String,
}

pub type ListObjectsResponse = ObjectListResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ListPartsResponse2 {
    #[serde(rename = "upload")]
    pub upload: MultipartUpload,

    #[serde(rename = "parts")]
    pub parts: Vec<MultipartPart>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MultipartPart {
    #[serde(rename = "part_number")]
    pub part_number: i64,

    #[serde(rename = "size")]
    pub size: i64,

    #[serde(rename = "etag")]
    pub etag: String,
}

pub type ListPartsResponse = ListPartsResponse2;

pub type ListPartsItem = MultipartPart;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSnapshotPoliciesParameters {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,

    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListSnapshotPoliciesQuery = ListSnapshotPoliciesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SnapshotPolicyListResponse {
    #[serde(
        rename = "snapshot_policies",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_policies: Option<Vec<SnapshotPolicy>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListSnapshotPoliciesResponse = SnapshotPolicyListResponse;

pub type ListSnapshotPoliciesItem = SnapshotPolicy;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSnapshotsParameters {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "volume", default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SnapshotStatusInput>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(
        rename = "snapshot_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub snapshot_policy: Option<String>,
}

pub type ListSnapshotsQuery = ListSnapshotsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SnapshotListResponse {
    #[serde(rename = "snapshots", default, skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<Snapshot>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListSnapshotsResponse = SnapshotListResponse;

pub type ListSnapshotsItem = Snapshot;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListVolumeTypesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListVolumeTypesQuery = ListVolumeTypesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VolumeTypeListResponse {
    #[serde(
        rename = "volume_types",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub volume_types: Option<Vec<VolumeType>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VolumeType {
    #[serde(
        rename = "included_iops",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub included_iops: Option<i64>,

    #[serde(
        rename = "included_throughput_mib_s",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub included_throughput_mib_s: Option<i64>,

    #[serde(rename = "max_iops", default, skip_serializing_if = "Option::is_none")]
    pub max_iops: Option<i64>,

    #[serde(
        rename = "max_throughput_mib_s",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_throughput_mib_s: Option<i64>,
    /// Regional platform catalog identity, using the immutable type token.
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
    /// The type token (matches `volume_type` on a Volume).
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

pub type ListVolumeTypesResponse = VolumeTypeListResponse;

pub type ListVolumeTypesItem = VolumeType;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListVolumesParameters {
    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<VolumeStatusInput>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,
}

pub type ListVolumesQuery = ListVolumesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VolumeListResponse {
    #[serde(rename = "volumes", default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListVolumesResponse = VolumeListResponse;

pub type ListVolumesItem = Volume;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketCORSRequestInput {
    #[serde(rename = "cors")]
    pub cors: CORSConfigInput,
}
impl PutBucketCORSRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(cors: CORSConfigInput) -> Self {
        Self { cors }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CORSConfigInput {
    #[serde(rename = "rules")]
    pub rules: Vec<CORSRuleInput>,
}
impl CORSConfigInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(rules: Vec<CORSRuleInput>) -> Self {
        Self { rules }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CORSRuleInput {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "allowed_origins")]
    pub allowed_origins: Vec<String>,

    #[serde(rename = "allowed_methods")]
    pub allowed_methods: Vec<CORSRuleInputAllowedMethodsItem>,

    #[serde(
        rename = "allowed_headers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_headers: Option<Vec<String>>,

    #[serde(
        rename = "expose_headers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expose_headers: Option<Vec<String>>,

    #[serde(
        rename = "max_age_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_age_seconds: Option<i64>,
}
impl CORSRuleInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        allowed_origins: Vec<String>,
        allowed_methods: Vec<CORSRuleInputAllowedMethodsItem>,
    ) -> Self {
        Self {
            id: None,
            allowed_origins,
            allowed_methods,
            allowed_headers: None,
            expose_headers: None,
            max_age_seconds: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CORSRuleInputAllowedMethodsItem {
    GET,
    PUT,
    POST,
    DELETE,
    HEAD,
    Unknown(String),
}
impl CORSRuleInputAllowedMethodsItem {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GET => "GET",
            Self::PUT => "PUT",
            Self::POST => "POST",
            Self::DELETE => "DELETE",
            Self::HEAD => "HEAD",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for CORSRuleInputAllowedMethodsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CORSRuleInputAllowedMethodsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "GET" => Self::GET,
            "PUT" => Self::PUT,
            "POST" => Self::POST,
            "DELETE" => Self::DELETE,
            "HEAD" => Self::HEAD,
            _ => Self::Unknown(value),
        })
    }
}

pub type PutBucketCORSBody = PutBucketCORSRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketDeletionProtectionRequestInput {
    /// When true, DeleteBucket schedules deletion instead of removing immediately.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Whole days; omitted defaults to 7. Explicit values outside 1–30 return 400 even when disabling protection.
    #[serde(
        rename = "recovery_window_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_window_days: Option<i64>,
}
impl PutBucketDeletionProtectionRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            recovery_window_days: None,
        }
    }
}

pub type PutBucketDeletionProtectionBody = PutBucketDeletionProtectionRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketEncryptionRequestInput {
    #[serde(rename = "encryption")]
    pub encryption: EncryptionConfigInput,
}
impl PutBucketEncryptionRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(encryption: EncryptionConfigInput) -> Self {
        Self { encryption }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfigInput {
    #[serde(rename = "rules")]
    pub rules: Vec<EncryptionRuleInput>,
}
impl EncryptionConfigInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(rules: Vec<EncryptionRuleInput>) -> Self {
        Self { rules }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct EncryptionRuleInput {
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default: Option<EncryptionRuleInputDefault>,

    #[serde(
        rename = "bucket_key_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bucket_key_enabled: Option<bool>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EncryptionRuleInputDefault {
    #[serde(rename = "sse_algorithm")]
    pub sse_algorithm: String,
    /// Accepted but unused S3 placeholder. Only AES256 is supported; this is not a KMS resource reference.
    #[serde(
        rename = "kms_master_key_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_master_key_id: Option<String>,
}
impl EncryptionRuleInputDefault {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(sse_algorithm: String) -> Self {
        Self {
            sse_algorithm,
            kms_master_key_id: None,
        }
    }
}

pub type PutBucketEncryptionBody = PutBucketEncryptionRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketLifecycleRequestInput {
    #[serde(rename = "lifecycle")]
    pub lifecycle: LifecycleConfigInput,
}
impl PutBucketLifecycleRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(lifecycle: LifecycleConfigInput) -> Self {
        Self { lifecycle }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleConfigInput {
    #[serde(rename = "rules")]
    pub rules: Vec<LifecycleRuleInput>,
}
impl LifecycleConfigInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(rules: Vec<LifecycleRuleInput>) -> Self {
        Self { rules }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleRuleInput {
    #[serde(rename = "id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// `disabled` keeps the rule in the configuration but skips it during evaluation. The S3-compatible endpoint spells the same states `Enabled` / `Disabled` in its XML.
    #[serde(rename = "status")]
    pub status: LifecycleRuleInputStatus,

    #[serde(rename = "filter", default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<LifecycleRuleInputFilter>,
    /// Move matching objects to another storage class once they are old enough. The object keeps its identity — same key, same version id, same last-modified — and only its bytes move between pools, so pairing a transition with an expiration works: the expiry clock is not restarted by the move. Exactly one of `days` or `date`. When the rule also has an `expiration`, the transition must come strictly first, otherwise the object would be deleted before it ever moved and the rule is rejected. Only one transition per rule: the platform serves two classes, so a second has nowhere to go. The S3-compatible endpoint accepts a single-element `<Transition>` list and rejects longer ones rather than silently applying the first.
    #[serde(
        rename = "transition",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transition: Option<LifecycleRuleInputTransition>,

    #[serde(
        rename = "expiration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expiration: Option<LifecycleRuleInputExpiration>,

    #[serde(
        rename = "noncurrent_version_expiration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub noncurrent_version_expiration: Option<LifecycleRuleInputNoncurrentVersionExpiration>,

    #[serde(
        rename = "abort_incomplete_multipart_upload",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub abort_incomplete_multipart_upload: Option<LifecycleRuleInputAbortIncompleteMultipartUpload>,
}
impl LifecycleRuleInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(status: LifecycleRuleInputStatus) -> Self {
        Self {
            id: None,
            status,
            filter: None,
            transition: None,
            expiration: None,
            noncurrent_version_expiration: None,
            abort_incomplete_multipart_upload: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleRuleInputStatus {
    Enabled,
    Disabled,
    Unknown(String),
}
impl LifecycleRuleInputStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleRuleInputStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRuleInputStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "enabled" => Self::Enabled,
            "disabled" => Self::Disabled,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleInputFilter {
    #[serde(rename = "prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct LifecycleRuleInputTransition {
    /// Days after the object's last-modified time.
    #[serde(rename = "days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
    /// Absolute cut-off; fires on the next sweep after this instant.
    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// Destination class. Keeps the S3 standard's casing rather than the platform's lowercase convention, because the class vocabulary is S3's. Unsupported values are rejected — a transition to a class that does not exist would silently never run.
    #[serde(rename = "storage_class")]
    pub storage_class: LifecycleRuleInputTransitionStorageClass,
}
impl LifecycleRuleInputTransition {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(storage_class: LifecycleRuleInputTransitionStorageClass) -> Self {
        Self {
            days: None,
            date: None,
            storage_class,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LifecycleRuleInputTransitionStorageClass {
    STANDARD,
    COLD,
    Unknown(String),
}
impl LifecycleRuleInputTransitionStorageClass {
    pub fn as_str(&self) -> &str {
        match self {
            Self::STANDARD => "STANDARD",
            Self::COLD => "COLD",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LifecycleRuleInputTransitionStorageClass {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRuleInputTransitionStorageClass {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "STANDARD" => Self::STANDARD,
            "COLD" => Self::COLD,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleInputExpiration {
    #[serde(rename = "days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,

    #[serde(rename = "date", default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleInputNoncurrentVersionExpiration {
    #[serde(
        rename = "noncurrent_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub noncurrent_days: Option<i64>,

    #[serde(
        rename = "newer_noncurrent_versions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub newer_noncurrent_versions: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct LifecycleRuleInputAbortIncompleteMultipartUpload {
    #[serde(
        rename = "days_after_initiation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub days_after_initiation: Option<i64>,
}

pub type PutBucketLifecycleBody = PutBucketLifecycleRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketObjectLockRequestInput {
    #[serde(rename = "object_lock")]
    pub object_lock: ObjectLockConfigInput,
}
impl PutBucketObjectLockRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(object_lock: ObjectLockConfigInput) -> Self {
        Self { object_lock }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ObjectLockConfigInput {
    #[serde(
        rename = "object_lock_enabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub object_lock_enabled: Option<String>,

    #[serde(rename = "rule", default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<ObjectLockConfigInputRule>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ObjectLockConfigInputRule {
    #[serde(
        rename = "default_retention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_retention: Option<ObjectLockConfigInputRuleDefaultRetention>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ObjectLockConfigInputRuleDefaultRetention {
    #[serde(rename = "mode", default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<ObjectLockConfigInputRuleDefaultRetentionMode>,

    #[serde(rename = "days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,

    #[serde(rename = "years", default, skip_serializing_if = "Option::is_none")]
    pub years: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ObjectLockConfigInputRuleDefaultRetentionMode {
    GOVERNANCE,
    COMPLIANCE,
    Unknown(String),
}
impl ObjectLockConfigInputRuleDefaultRetentionMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GOVERNANCE => "GOVERNANCE",
            Self::COMPLIANCE => "COMPLIANCE",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ObjectLockConfigInputRuleDefaultRetentionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ObjectLockConfigInputRuleDefaultRetentionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "GOVERNANCE" => Self::GOVERNANCE,
            "COMPLIANCE" => Self::COMPLIANCE,
            _ => Self::Unknown(value),
        })
    }
}

pub type PutBucketObjectLockBody = PutBucketObjectLockRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketPolicyRequestInput {
    #[serde(rename = "document")]
    pub document: BucketPolicyInput,
}
impl PutBucketPolicyRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(document: BucketPolicyInput) -> Self {
        Self { document }
    }
}

pub type BucketPolicyInput = std::collections::BTreeMap<String, serde_json::Value>;

pub type PutBucketPolicyBody = PutBucketPolicyRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketTaggingRequestInput {
    #[serde(rename = "tags")]
    pub tags: TagSetInput,
}
impl PutBucketTaggingRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(tags: TagSetInput) -> Self {
        Self { tags }
    }
}

pub type TagSetInput = std::collections::BTreeMap<String, String>;

pub type PutBucketTaggingBody = PutBucketTaggingRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutBucketVersioningRequestInput {
    #[serde(rename = "status")]
    pub status: PutBucketVersioningRequestInputStatus,
}
impl PutBucketVersioningRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(status: PutBucketVersioningRequestInputStatus) -> Self {
        Self { status }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PutBucketVersioningRequestInputStatus {
    Enabled,
    Suspended,
    Unknown(String),
}
impl PutBucketVersioningRequestInputStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Enabled => "enabled",
            Self::Suspended => "suspended",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PutBucketVersioningRequestInputStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PutBucketVersioningRequestInputStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "enabled" => Self::Enabled,
            "suspended" => Self::Suspended,
            _ => Self::Unknown(value),
        })
    }
}

pub type PutBucketVersioningBody = PutBucketVersioningRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutObjectResponse2 {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "etag")]
    pub etag: String,

    #[serde(rename = "size")]
    pub size: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PutObjectResult {
    Variant1(Box<PutObjectResponse2>),
    Variant2(Box<std::collections::BTreeMap<String, serde_json::Value>>),
}

pub type PutObjectResponse = PutObjectResult;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SnapshotUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateSnapshotBody = SnapshotUpdateRequestInput;

pub type UpdateSnapshotResponse = SnapshotResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct SnapshotPolicyUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    /// false pauses the policy, true resumes it. Pausing stops the whole policy — no snapshots are taken and none are deleted, so a paused schedule cannot lose you history. Resuming applies the retention window again on the next run, so anything sitting outside it by then — because you lowered `retention_count` while paused, say — is reaped on that run.
    #[serde(rename = "enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(
        rename = "interval_minutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub interval_minutes: Option<SnapshotIntervalMinutesInput>,

    #[serde(
        rename = "retention_count",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_count: Option<SnapshotRetentionCountInput>,

    #[serde(
        rename = "retention_days",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retention_days: Option<SnapshotRetentionDaysInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateSnapshotPolicyBody = SnapshotPolicyUpdateRequestInput;

pub type UpdateSnapshotPolicyResponse = SnapshotPolicyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct VolumeUpdateRequestInput {
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateVolumeBody = VolumeUpdateRequestInput;

pub type UpdateVolumeResponse = VolumeResponse;

pub type UpdateVolumePerformanceBody = VolumePerformanceRequestInput;

pub type UpdateVolumePerformanceResponse = VolumeResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UploadPartResponse2 {
    #[serde(rename = "part_number")]
    pub part_number: i64,

    #[serde(rename = "etag")]
    pub etag: String,

    #[serde(rename = "size")]
    pub size: i64,
}

pub type UploadPartResponse = UploadPartResponse2;
