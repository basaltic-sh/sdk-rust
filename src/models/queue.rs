//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ChangeVisibilityRequestInput {
    #[serde(rename = "receipt_handle")]
    pub receipt_handle: String,
    /// Measured from now, not from the original lease. 0 releases the message immediately.
    #[serde(rename = "visibility_timeout_seconds")]
    pub visibility_timeout_seconds: i64,
}
impl ChangeVisibilityRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(receipt_handle: String, visibility_timeout_seconds: i64) -> Self {
        Self {
            receipt_handle,
            visibility_timeout_seconds,
        }
    }
}

pub type ChangeMessageVisibilityBody = ChangeVisibilityRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ChangeVisibilityBatchRequestInput {
    #[serde(rename = "entries")]
    pub entries: Vec<ChangeVisibilityBatchRequestInputEntriesItem>,
}
impl ChangeVisibilityBatchRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(entries: Vec<ChangeVisibilityBatchRequestInputEntriesItem>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ChangeVisibilityBatchRequestInputEntriesItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "receipt_handle")]
    pub receipt_handle: String,

    #[serde(rename = "visibility_timeout_seconds")]
    pub visibility_timeout_seconds: i64,
}
impl ChangeVisibilityBatchRequestInputEntriesItem {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(id: String, receipt_handle: String, visibility_timeout_seconds: i64) -> Self {
        Self {
            id,
            receipt_handle,
            visibility_timeout_seconds,
        }
    }
}

pub type ChangeMessageVisibilityBatchBody = ChangeVisibilityBatchRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchResponse {
    #[serde(rename = "results")]
    pub results: Vec<BatchResult>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BatchResult {
    /// The entry id from the request.
    #[serde(rename = "id")]
    pub id: String,

    #[serde(
        rename = "message_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_id: Option<String>,

    #[serde(rename = "duplicate", default, skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<bool>,
    /// Machine-readable code when this entry failed.
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Human-readable reason accompanying error.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub type ChangeMessageVisibilityBatchResponse = BatchResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateQueueRequestInput {
    /// Letters, digits, dot, dash, or underscore, starting with a letter or digit. A name ending in `.fifo` creates a FIFO queue. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "visibility_timeout_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility_timeout_seconds: Option<i64>,

    #[serde(
        rename = "message_retention_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_retention_seconds: Option<i64>,

    #[serde(
        rename = "max_message_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_message_bytes: Option<i64>,

    #[serde(
        rename = "delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delay_seconds: Option<i64>,

    #[serde(
        rename = "receive_wait_time_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_wait_time_seconds: Option<i64>,

    #[serde(
        rename = "kms_data_key_reuse_period_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_data_key_reuse_period_seconds: Option<i64>,
    /// FIFO queues only.
    #[serde(
        rename = "content_based_deduplication",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_based_deduplication: Option<bool>,
    /// KMS key UUID, CRN or account-scoped name. Omit to use the platform key.
    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,

    #[serde(
        rename = "redrive_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redrive_policy: Option<RedrivePolicyInputInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl CreateQueueRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            visibility_timeout_seconds: None,
            message_retention_seconds: None,
            max_message_bytes: None,
            delay_seconds: None,
            receive_wait_time_seconds: None,
            kms_data_key_reuse_period_seconds: None,
            content_based_deduplication: None,
            kms_key: None,
            redrive_policy: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RedrivePolicyInputInput {
    #[serde(rename = "dead_letter_queue")]
    pub dead_letter_queue: String,

    #[serde(rename = "max_receive_count")]
    pub max_receive_count: i64,
}
impl RedrivePolicyInputInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(dead_letter_queue: String, max_receive_count: i64) -> Self {
        Self {
            dead_letter_queue,
            max_receive_count,
        }
    }
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateQueueBody = CreateQueueRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueueResponse {
    #[serde(rename = "queue")]
    pub queue: Queue,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Queue {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Unique within the calling account. Immutable. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// Derived from the name and immutable.
    #[serde(rename = "kind")]
    pub kind: QueueKind,

    #[serde(rename = "attributes")]
    pub attributes: QueueAttributes,

    #[serde(
        rename = "redrive_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redrive_policy: Option<RedrivePolicy>,
    /// FIFO only. Derives a deduplication id from a hash of the body when the sender supplies none.
    #[serde(
        rename = "content_based_deduplication",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_based_deduplication: Option<bool>,
    /// Customer-managed key wrapping the queue data key. Omitted for platform encryption. If key metadata is unavailable, the reference retains its stored ID and CRN without a name.
    #[serde(rename = "kms_key", default, skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<ResourceRef>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(rename = "depth", default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<QueueDepth>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueueKind {
    Standard,
    Fifo,
    Unknown(String),
}
impl QueueKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Standard => "standard",
            Self::Fifo => "fifo",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for QueueKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QueueKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "standard" => Self::Standard,
            "fifo" => Self::Fifo,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct QueueAttributes {
    /// How long a received message stays invisible before redelivery. 0 redelivers immediately.
    #[serde(
        rename = "visibility_timeout_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility_timeout_seconds: Option<i64>,
    /// How long an undeleted message survives before it is discarded.
    #[serde(
        rename = "message_retention_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_retention_seconds: Option<i64>,
    /// Largest accepted message. Attributes count against this alongside the body.
    #[serde(
        rename = "max_message_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_message_bytes: Option<i64>,
    /// Default delay applied to every send that does not override it. Not supported on FIFO queues.
    #[serde(
        rename = "delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delay_seconds: Option<i64>,
    /// Default long-poll window for a receive that does not override it.
    #[serde(
        rename = "receive_wait_time_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_wait_time_seconds: Option<i64>,
    /// How long an unwrapped data key is cached. Sends and receives keep working for up to this long after the wrapping key is disabled; lower it to tighten that window at the cost of more KMS round-trips.
    #[serde(
        rename = "kms_data_key_reuse_period_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_data_key_reuse_period_seconds: Option<i64>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RedrivePolicy {
    #[serde(rename = "dead_letter_queue")]
    pub dead_letter_queue: ResourceRef,

    #[serde(rename = "max_receive_count")]
    pub max_receive_count: i64,
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

pub type Tags = std::collections::BTreeMap<String, String>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct QueueDepth {
    #[serde(
        rename = "approximate_available",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approximate_available: Option<i64>,

    #[serde(
        rename = "approximate_in_flight",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approximate_in_flight: Option<i64>,

    #[serde(
        rename = "approximate_delayed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub approximate_delayed: Option<i64>,

    #[serde(
        rename = "counts_capped",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub counts_capped: Option<bool>,
}

pub type CreateQueueResponse = QueueResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteMessageParameters {
    #[serde(rename = "receipt_handle")]
    pub receipt_handle: String,
}
impl DeleteMessageParameters {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(receipt_handle: String) -> Self {
        Self { receipt_handle }
    }
}

pub type DeleteMessageQuery = DeleteMessageParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteMessageBatchRequestInput {
    #[serde(rename = "entries")]
    pub entries: Vec<DeleteMessageBatchRequestInputEntriesItem>,
}
impl DeleteMessageBatchRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(entries: Vec<DeleteMessageBatchRequestInputEntriesItem>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteMessageBatchRequestInputEntriesItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "receipt_handle")]
    pub receipt_handle: String,
}
impl DeleteMessageBatchRequestInputEntriesItem {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(id: String, receipt_handle: String) -> Self {
        Self { id, receipt_handle }
    }
}

pub type DeleteMessageBatchBody = DeleteMessageBatchRequestInput;

pub type DeleteMessageBatchResponse = BatchResponse;

pub type DescribeQueueResponse = QueueResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListQueuesParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListQueuesQuery = ListQueuesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct QueueListResponse {
    #[serde(rename = "queues")]
    pub queues: Vec<Queue>,

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

pub type ListQueuesResponse = QueueListResponse;

pub type ListQueuesItem = Queue;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PeekMessagesParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(
        rename = "visibility",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility: Option<PeekMessagesParametersVisibility>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PeekMessagesParametersVisibility {
    Available,
    InFlight,
    Delayed,
    Unknown(String),
}
impl PeekMessagesParametersVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Available => "available",
            Self::InFlight => "in_flight",
            Self::Delayed => "delayed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PeekMessagesParametersVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PeekMessagesParametersVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "available" => Self::Available,
            "in_flight" => Self::InFlight,
            "delayed" => Self::Delayed,
            _ => Self::Unknown(value),
        })
    }
}

pub type PeekMessagesQuery = PeekMessagesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PeekResponse {
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "queue_id")]
    pub queue_id: String,
    /// Absent when the body could not be decrypted — the rest of the row is still reported.
    #[serde(rename = "body", default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<MessageAttributes>,

    #[serde(rename = "visibility")]
    pub visibility: MessageVisibility,

    #[serde(rename = "receive_count")]
    pub receive_count: i64,
    /// Plaintext size, counted against the queue's max_message_bytes.
    #[serde(
        rename = "body_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub body_bytes: Option<i64>,

    #[serde(
        rename = "message_group_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_group_id: Option<String>,

    #[serde(rename = "enqueued_at")]
    pub enqueued_at: String,

    #[serde(rename = "visible_at")]
    pub visible_at: String,
    /// When retention discards this message.
    #[serde(rename = "expires_at")]
    pub expires_at: String,

    #[serde(
        rename = "first_received_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub first_received_at: Option<crate::Nullable<String>>,
}

pub type MessageAttributes = std::collections::BTreeMap<String, MessageAttributesValue>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageAttributesValue {
    #[serde(rename = "type")]
    pub type_: MessageAttributesValueType,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageAttributesValueType {
    String,
    Number,
    Binary,
    Unknown(String),
}
impl MessageAttributesValueType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String => "String",
            Self::Number => "Number",
            Self::Binary => "Binary",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for MessageAttributesValueType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MessageAttributesValueType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "String" => Self::String,
            "Number" => Self::Number,
            "Binary" => Self::Binary,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageVisibility {
    Available,
    InFlight,
    Delayed,
    Unknown(String),
}
impl MessageVisibility {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Available => "available",
            Self::InFlight => "in_flight",
            Self::Delayed => "delayed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for MessageVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MessageVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "available" => Self::Available,
            "in_flight" => Self::InFlight,
            "delayed" => Self::Delayed,
            _ => Self::Unknown(value),
        })
    }
}

pub type PeekMessagesResponse = PeekResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PurgeQueueResponse2 {
    #[serde(rename = "messages_deleted")]
    pub messages_deleted: i64,
}

pub type PurgeQueueResponse = PurgeQueueResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ReceiveRequestInput {
    /// How many messages to lease. Fewer may be returned even when more are available.
    #[serde(
        rename = "max_messages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_messages: Option<i64>,
    /// Overrides the queue's default for this lease only.
    #[serde(
        rename = "visibility_timeout_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility_timeout_seconds: Option<i64>,
    /// Block up to this long for a message rather than returning empty. Defaults to the queue's receive_wait_time_seconds.
    #[serde(
        rename = "wait_time_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub wait_time_seconds: Option<i64>,
}

pub type ReceiveMessagesBody = ReceiveRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceiveResponse {
    #[serde(rename = "messages")]
    pub messages: Vec<ReceivedMessage>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ReceivedMessage {
    #[serde(rename = "id")]
    pub id: String,
    /// The credential for this lease. Required to delete the message or extend its lease, and valid only until the lease lapses.
    #[serde(rename = "receipt_handle")]
    pub receipt_handle: String,

    #[serde(rename = "body")]
    pub body: String,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<MessageAttributes>,
    /// How many times this message has been leased, including now. Compare against the redrive policy to spot a poison message.
    #[serde(rename = "receive_count")]
    pub receive_count: i64,

    #[serde(
        rename = "message_group_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_group_id: Option<String>,

    #[serde(rename = "enqueued_at")]
    pub enqueued_at: String,
    /// When this lease lapses and the message is redelivered.
    #[serde(rename = "visible_at")]
    pub visible_at: String,
}

pub type ReceiveMessagesResponse = ReceiveResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendMessageRequestInput {
    /// The payload. Counted against the queue's max_message_bytes together with the attributes.
    #[serde(rename = "body")]
    pub body: String,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<MessageAttributesInput>,
    /// Overrides the queue's default for this message. Rejected on FIFO queues, where it would let a later message become visible before an earlier one in the same group.
    #[serde(
        rename = "delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delay_seconds: Option<i64>,
    /// Required on FIFO queues, rejected on standard ones. Messages sharing a group are delivered in order, and a group's next message is withheld while an earlier one is in flight.
    #[serde(
        rename = "message_group_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_group_id: Option<String>,
    /// FIFO only. A repeat within five minutes is suppressed and the original message's id is returned. Omit on a queue with `content_based_deduplication` to derive it from the body.
    #[serde(
        rename = "message_deduplication_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_deduplication_id: Option<String>,
}
impl SendMessageRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(body: String) -> Self {
        Self {
            body,
            attributes: None,
            delay_seconds: None,
            message_group_id: None,
            message_deduplication_id: None,
        }
    }
}

pub type MessageAttributesInput = std::collections::BTreeMap<String, MessageAttributesInputValue>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageAttributesInputValue {
    #[serde(rename = "type")]
    pub type_: MessageAttributesInputValueType,

    #[serde(rename = "value")]
    pub value: String,
}
impl MessageAttributesInputValue {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(type_: MessageAttributesInputValueType, value: String) -> Self {
        Self { type_, value }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageAttributesInputValueType {
    String,
    Number,
    Binary,
    Unknown(String),
}
impl MessageAttributesInputValueType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String => "String",
            Self::Number => "Number",
            Self::Binary => "Binary",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for MessageAttributesInputValueType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MessageAttributesInputValueType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "String" => Self::String,
            "Number" => Self::Number,
            "Binary" => Self::Binary,
            _ => Self::Unknown(value),
        })
    }
}

pub type SendMessageBody = SendMessageRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendMessageResponse2 {
    #[serde(rename = "result")]
    pub result: SendResult,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendResult {
    #[serde(rename = "message_id")]
    pub message_id: String,
    /// A live deduplication entry suppressed this send; message_id names the message the original produced.
    #[serde(rename = "duplicate", default, skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<bool>,
}

pub type SendMessageResponse = SendMessageResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendMessageBatchRequestInput {
    #[serde(rename = "entries")]
    pub entries: Vec<SendMessageBatchRequestInputEntriesItem>,
}
impl SendMessageBatchRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(entries: Vec<SendMessageBatchRequestInputEntriesItem>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SendMessageBatchRequestInputEntriesItem {
    /// Your own identifier for this entry, echoed back in the result. Must be unique within the request.
    #[serde(rename = "id")]
    pub id: String,
    /// The payload. Counted against the queue's max_message_bytes together with the attributes.
    #[serde(rename = "body")]
    pub body: String,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<MessageAttributesInput>,
    /// Overrides the queue's default for this message. Rejected on FIFO queues, where it would let a later message become visible before an earlier one in the same group.
    #[serde(
        rename = "delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delay_seconds: Option<i64>,
    /// Required on FIFO queues, rejected on standard ones. Messages sharing a group are delivered in order, and a group's next message is withheld while an earlier one is in flight.
    #[serde(
        rename = "message_group_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_group_id: Option<String>,
    /// FIFO only. A repeat within five minutes is suppressed and the original message's id is returned. Omit on a queue with `content_based_deduplication` to derive it from the body.
    #[serde(
        rename = "message_deduplication_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_deduplication_id: Option<String>,
}
impl SendMessageBatchRequestInputEntriesItem {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(id: String, body: String) -> Self {
        Self {
            id,
            body,
            attributes: None,
            delay_seconds: None,
            message_group_id: None,
            message_deduplication_id: None,
        }
    }
}

pub type SendMessageBatchBody = SendMessageBatchRequestInput;

pub type SendMessageBatchResponse = BatchResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateQueueRequestInput {
    #[serde(
        rename = "visibility_timeout_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub visibility_timeout_seconds: Option<i64>,

    #[serde(
        rename = "message_retention_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_retention_seconds: Option<i64>,

    #[serde(
        rename = "max_message_bytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_message_bytes: Option<i64>,

    #[serde(
        rename = "delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delay_seconds: Option<i64>,

    #[serde(
        rename = "receive_wait_time_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_wait_time_seconds: Option<i64>,

    #[serde(
        rename = "kms_data_key_reuse_period_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kms_data_key_reuse_period_seconds: Option<i64>,
    /// Omit to keep the current policy; null removes it. An object replaces the policy.
    #[serde(
        rename = "redrive_policy",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub redrive_policy: Option<crate::Nullable<UpdateQueueRequestInputRedrivePolicyValue>>,
    /// KMS key UUID, CRN or account-scoped name. Omit to keep the current key; null restores the platform key. Existing messages keep the key that sealed them.
    #[serde(
        rename = "kms_key",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub kms_key: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UpdateQueueRequestInputRedrivePolicyValue {
    /// Queue UUID, CRN or account-scoped name.
    #[serde(rename = "dead_letter_queue")]
    pub dead_letter_queue: String,

    #[serde(rename = "max_receive_count")]
    pub max_receive_count: i64,
}
impl UpdateQueueRequestInputRedrivePolicyValue {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(dead_letter_queue: String, max_receive_count: i64) -> Self {
        Self {
            dead_letter_queue,
            max_receive_count,
        }
    }
}

pub type UpdateQueueBody = UpdateQueueRequestInput;

pub type UpdateQueueResponse = QueueResponse;
