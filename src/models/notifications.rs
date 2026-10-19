//! Generated typed wire models. Unknown string enum values are retained.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTopicRequestInput {
    /// 1-256 characters of letters, digits, dot, dash, or underscore. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,

    #[serde(
        rename = "display_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    #[serde(
        rename = "delivery_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_policy: Option<DeliveryPolicyInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl CreateTopicRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(name: String) -> Self {
        Self {
            name,
            display_name: None,
            delivery_policy: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DeliveryPolicyInput {
    /// Total attempts including the first. 1 disables retries.
    #[serde(
        rename = "max_attempts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_attempts: Option<i64>,
    /// Backoff before the second attempt.
    #[serde(
        rename = "min_delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_delay_seconds: Option<i64>,
    /// Cap on the exponential curve.
    #[serde(
        rename = "max_delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_delay_seconds: Option<i64>,
    /// Per-request timeout. Ignored for email.
    #[serde(
        rename = "timeout_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_seconds: Option<i64>,
}

pub type TagsInput = std::collections::BTreeMap<String, String>;

pub type CreateTopicBody = CreateTopicRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TopicResponse {
    #[serde(rename = "topic")]
    pub topic: Topic,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Topic {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "crn")]
    pub crn: String,
    /// Unique within the calling account. Immutable. Resource names must not start with the literal crn: prefix or be UUIDs (canonical, compact, braced, or urn:uuid: forms, in either case).
    #[serde(rename = "name")]
    pub name: String,
    /// Human-facing label; used as the email subject when a notification carries none.
    #[serde(
        rename = "display_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    #[serde(
        rename = "delivery_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_policy: Option<DeliveryPolicy>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct DeliveryPolicy {
    /// Total attempts including the first. 1 disables retries.
    #[serde(
        rename = "max_attempts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_attempts: Option<i64>,
    /// Backoff before the second attempt.
    #[serde(
        rename = "min_delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_delay_seconds: Option<i64>,
    /// Cap on the exponential curve.
    #[serde(
        rename = "max_delay_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_delay_seconds: Option<i64>,
    /// Per-request timeout. Ignored for email.
    #[serde(
        rename = "timeout_seconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout_seconds: Option<i64>,
}

pub type Tags = std::collections::BTreeMap<String, String>;

pub type CreateTopicResponse = TopicResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageResponse {
    #[serde(rename = "message")]
    pub message: Message,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(
        rename = "message_structure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_structure: Option<MessageMessageStructure>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<MessageAttributes>,
    /// `pending` until the worker has expanded the message into deliveries.
    #[serde(rename = "fanout_state")]
    pub fanout_state: MessageFanoutState,
    /// Deliveries fan-out produced. Meaningful only once fanout_state is complete.
    #[serde(rename = "delivery_count")]
    pub delivery_count: i64,

    #[serde(rename = "published_at")]
    pub published_at: String,

    #[serde(
        rename = "fanned_out_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub fanned_out_at: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageMessageStructure {
    Value,
    Json,
    Unknown(String),
}
impl MessageMessageStructure {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value => "",
            Self::Json => "json",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for MessageMessageStructure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MessageMessageStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "" => Self::Value,
            "json" => Self::Json,
            _ => Self::Unknown(value),
        })
    }
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
    Unknown(String),
}
impl MessageAttributesValueType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String => "String",
            Self::Number => "Number",
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
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageFanoutState {
    Pending,
    Complete,
    Unknown(String),
}
impl MessageFanoutState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Complete => "complete",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for MessageFanoutState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MessageFanoutState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "complete" => Self::Complete,
            _ => Self::Unknown(value),
        })
    }
}

pub type DescribeMessageResponse = MessageResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionResponse {
    #[serde(rename = "subscription")]
    pub subscription: Subscription,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Subscription {
    #[serde(rename = "id")]
    pub id: String,
    /// Nested under the actual topic name and owner account, including cross-account subscriptions.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "protocol")]
    pub protocol: SubscriptionProtocol,
    /// Immutable. An email address, an http/https URL, or a canonical queue CRN when protocol is queue.
    #[serde(rename = "endpoint")]
    pub endpoint: String,

    #[serde(rename = "status")]
    pub status: SubscriptionStatus,
    /// http/https only. Deliver the published payload verbatim instead of the notification envelope; the metadata moves into headers.
    #[serde(rename = "raw_message_delivery")]
    pub raw_message_delivery: bool,

    #[serde(
        rename = "filter_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_policy: Option<FilterPolicy>,

    #[serde(
        rename = "delivery_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_policy: Option<DeliveryPolicy>,

    #[serde(
        rename = "redrive_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redrive_policy: Option<RedrivePolicy>,
    /// When the pending confirmation lapses. Absent once confirmed.
    #[serde(
        rename = "confirmation_expires_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub confirmation_expires_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "confirmed_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub confirmed_at: Option<crate::Nullable<String>>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubscriptionProtocol {
    Email,
    Https,
    Http,
    Queue,
    Unknown(String),
}
impl SubscriptionProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Email => "email",
            Self::Https => "https",
            Self::Http => "http",
            Self::Queue => "queue",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SubscriptionProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "email" => Self::Email,
            "https" => Self::Https,
            "http" => Self::Http,
            "queue" => Self::Queue,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubscriptionStatus {
    PendingConfirmation,
    Confirmed,
    Unknown(String),
}
impl SubscriptionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PendingConfirmation => "pending_confirmation",
            Self::Confirmed => "confirmed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SubscriptionStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscriptionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending_confirmation" => Self::PendingConfirmation,
            "confirmed" => Self::Confirmed,
            _ => Self::Unknown(value),
        })
    }
}

pub type FilterPolicy = std::collections::BTreeMap<String, Vec<serde_json::Value>>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RedrivePolicy {
    /// The bound queue. A deleted target retains its stored ID and CRN without a name.
    #[serde(rename = "dead_letter_queue")]
    pub dead_letter_queue: ResourceRef,
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

pub type DescribeSubscriptionResponse = SubscriptionResponse;

pub type DescribeTopicResponse = TopicResponse;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SigningSecretResponse {
    #[serde(rename = "subscription")]
    pub subscription: ResourceRef,

    #[serde(rename = "signing_secret")]
    pub signing_secret: String,
}
impl std::fmt::Debug for SigningSecretResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SigningSecretResponse");
        d.field("subscription", &self.subscription);
        d.field("signing_secret", &"[REDACTED]");
        d.finish()
    }
}

pub type GetSubscriptionSigningSecretResponse = SigningSecretResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TopicPolicyResponse {
    #[serde(rename = "document")]
    pub document: TopicPolicy,
}

pub type TopicPolicy = std::collections::BTreeMap<String, serde_json::Value>;

pub type GetTopicPolicyResponse = TopicPolicyResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListDeliveriesParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(
        rename = "subscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subscription: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ListDeliveriesParametersStatus>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListDeliveriesParametersStatus {
    Pending,
    Delivered,
    Failed,
    Unknown(String),
}
impl ListDeliveriesParametersStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListDeliveriesParametersStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListDeliveriesParametersStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "delivered" => Self::Delivered,
            "failed" => Self::Failed,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListDeliveriesQuery = ListDeliveriesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeliveryListResponse {
    #[serde(rename = "deliveries")]
    pub deliveries: Vec<Delivery>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Delivery {
    /// Canonical message/UUID/delivery/UUID CRN.
    #[serde(rename = "crn")]
    pub crn: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "message")]
    pub message: ResourceRef,

    #[serde(rename = "subscription")]
    pub subscription: ResourceRef,

    #[serde(rename = "protocol")]
    pub protocol: DeliveryProtocol,
    /// Endpoint recorded at fan-out.
    #[serde(rename = "endpoint")]
    pub endpoint: String,

    #[serde(rename = "status")]
    pub status: DeliveryStatus,

    #[serde(rename = "attempts")]
    pub attempts: i64,
    /// HTTP status of the last attempt. Absent for a transport-level failure or an email delivery.
    #[serde(
        rename = "last_status_code",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_status_code: Option<crate::Nullable<i64>>,
    /// Truncated to 1024 characters.
    #[serde(
        rename = "last_error",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_error: Option<String>,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(
        rename = "last_attempt_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub last_attempt_at: Option<crate::Nullable<String>>,

    #[serde(
        rename = "completed_at",
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::response::present_nullable"
    )]
    pub completed_at: Option<crate::Nullable<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeliveryProtocol {
    Email,
    Https,
    Http,
    Queue,
    Unknown(String),
}
impl DeliveryProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Email => "email",
            Self::Https => "https",
            Self::Http => "http",
            Self::Queue => "queue",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for DeliveryProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeliveryProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "email" => Self::Email,
            "https" => Self::Https,
            "http" => Self::Http,
            "queue" => Self::Queue,
            _ => Self::Unknown(value),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
    Unknown(String),
}
impl DeliveryStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for DeliveryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeliveryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "delivered" => Self::Delivered,
            "failed" => Self::Failed,
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

pub type ListDeliveriesResponse = DeliveryListResponse;

pub type ListDeliveriesItem = Delivery;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListMessagesParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(
        rename = "fanout_state",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fanout_state: Option<ListMessagesParametersFanoutState>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListMessagesParametersFanoutState {
    Pending,
    Complete,
    Unknown(String),
}
impl ListMessagesParametersFanoutState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Complete => "complete",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListMessagesParametersFanoutState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListMessagesParametersFanoutState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "complete" => Self::Complete,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListMessagesQuery = ListMessagesParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MessageListResponse {
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,

    #[serde(rename = "meta")]
    pub meta: PaginationMeta,
}

pub type ListMessagesResponse = MessageListResponse;

pub type ListMessagesItem = Message;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListSubscriptionsParameters {
    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ListSubscriptionsParametersStatus>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ListSubscriptionsParametersStatus {
    PendingConfirmation,
    Confirmed,
    Unknown(String),
}
impl ListSubscriptionsParametersStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PendingConfirmation => "pending_confirmation",
            Self::Confirmed => "confirmed",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListSubscriptionsParametersStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListSubscriptionsParametersStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "pending_confirmation" => Self::PendingConfirmation,
            "confirmed" => Self::Confirmed,
            _ => Self::Unknown(value),
        })
    }
}

pub type ListSubscriptionsQuery = ListSubscriptionsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionListResponse {
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<Subscription>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListSubscriptionsResponse = SubscriptionListResponse;

pub type ListSubscriptionsItem = Subscription;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct ListTopicsParameters {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "crn", default, skip_serializing_if = "Option::is_none")]
    pub crn: Option<String>,

    #[serde(rename = "marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,

    #[serde(rename = "limit", default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

pub type ListTopicsQuery = ListTopicsParameters;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TopicListResponse {
    #[serde(rename = "topics")]
    pub topics: Vec<Topic>,

    #[serde(rename = "meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

pub type ListTopicsResponse = TopicListResponse;

pub type ListTopicsItem = Topic;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PublishRequestInput {
    /// Used as the email subject. No line breaks.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The payload. Up to 256 KiB.
    #[serde(rename = "message")]
    pub message: String,
    /// Omit to deliver `message` verbatim to every protocol. Set to `json` to have `message` read as an object of protocol keys (`default`, `email`, `https`, `http`), letting one publish carry a different payload per transport. A `default` key is required — it is what a protocol with no explicit entry gets.
    #[serde(
        rename = "message_structure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_structure: Option<PublishRequestInputMessageStructure>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<MessageAttributesInput>,
}
impl PublishRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(message: String) -> Self {
        Self {
            subject: None,
            message,
            message_structure: None,
            attributes: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublishRequestInputMessageStructure {
    Value,
    Json,
    Unknown(String),
}
impl PublishRequestInputMessageStructure {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value => "",
            Self::Json => "json",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PublishRequestInputMessageStructure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PublishRequestInputMessageStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "" => Self::Value,
            "json" => Self::Json,
            _ => Self::Unknown(value),
        })
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
    Unknown(String),
}
impl MessageAttributesInputValueType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String => "String",
            Self::Number => "Number",
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
            _ => Self::Unknown(value),
        })
    }
}

pub type PublishBody = PublishRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PublishResponse2 {
    #[serde(rename = "message_id")]
    pub message_id: String,
}

pub type PublishResponse = PublishResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PublishBatchRequestInput {
    #[serde(rename = "entries")]
    pub entries: Vec<PublishBatchRequestInputEntriesItem>,
}
impl PublishBatchRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(entries: Vec<PublishBatchRequestInputEntriesItem>) -> Self {
        Self { entries }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PublishBatchRequestInputEntriesItem {
    /// Caller-supplied correlation id, unique within the batch. Echoed back in the result.
    #[serde(rename = "id")]
    pub id: String,
    /// Used as the email subject. No line breaks.
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The payload. Up to 256 KiB.
    #[serde(rename = "message")]
    pub message: String,
    /// Omit to deliver `message` verbatim to every protocol. Set to `json` to have `message` read as an object of protocol keys (`default`, `email`, `https`, `http`), letting one publish carry a different payload per transport. A `default` key is required — it is what a protocol with no explicit entry gets.
    #[serde(
        rename = "message_structure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_structure: Option<PublishBatchRequestInputEntriesItemMessageStructure>,

    #[serde(
        rename = "attributes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub attributes: Option<MessageAttributesInput>,
}
impl PublishBatchRequestInputEntriesItem {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(id: String, message: String) -> Self {
        Self {
            id,
            subject: None,
            message,
            message_structure: None,
            attributes: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublishBatchRequestInputEntriesItemMessageStructure {
    Value,
    Json,
    Unknown(String),
}
impl PublishBatchRequestInputEntriesItemMessageStructure {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Value => "",
            Self::Json => "json",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PublishBatchRequestInputEntriesItemMessageStructure {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PublishBatchRequestInputEntriesItemMessageStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "" => Self::Value,
            "json" => Self::Json,
            _ => Self::Unknown(value),
        })
    }
}

pub type PublishBatchBody = PublishBatchRequestInput;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct PublishBatchResponse2 {
    #[serde(
        rename = "successful",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub successful: Option<Vec<PublishBatchResponse2SuccessfulItem>>,

    #[serde(rename = "failed", default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<PublishBatchResponse2FailedItem>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PublishBatchResponse2SuccessfulItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "message_id")]
    pub message_id: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PublishBatchResponse2FailedItem {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "message")]
    pub message: String,
}

pub type PublishBatchResponse = PublishBatchResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PutTopicPolicyRequestInput {
    #[serde(rename = "document")]
    pub document: TopicPolicyInput,
}
impl PutTopicPolicyRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(document: TopicPolicyInput) -> Self {
        Self { document }
    }
}

pub type TopicPolicyInput = std::collections::BTreeMap<String, serde_json::Value>;

pub type PutTopicPolicyBody = PutTopicPolicyRequestInput;

pub type PutTopicPolicyResponse = TopicPolicyResponse;

pub type ResendConfirmationResponse = SubscriptionResponse;

pub type RotateSubscriptionSigningSecretResponse = SigningSecretResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SubscribeRequestInput {
    #[serde(rename = "protocol")]
    pub protocol: SubscribeRequestInputProtocol,
    /// An email address; an http/https URL whose scheme matches the protocol and resolves to a publicly-routable address; or, for `queue`, a queue UUID, CRN or exact name in the subscriber account and serving region. Queue targets are stored and returned as canonical CRNs. A `queue` subscription skips the confirmation handshake and is created already confirmed. The handshake exists to prove that whoever controls the endpoint consents to receive, which is necessary for an address or a URL you may not own. A queue is your own resource, so consent is established instead by requiring `queue:SendMessage` on it at subscribe time — an authorization decision rather than a demonstration of reachability.
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    /// http/https only.
    #[serde(
        rename = "raw_message_delivery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub raw_message_delivery: Option<bool>,

    #[serde(
        rename = "filter_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_policy: Option<FilterPolicyInput>,

    #[serde(
        rename = "delivery_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_policy: Option<DeliveryPolicyInput>,

    #[serde(
        rename = "redrive_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redrive_policy: Option<RedrivePolicyInputInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}
impl SubscribeRequestInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(protocol: SubscribeRequestInputProtocol, endpoint: String) -> Self {
        Self {
            protocol,
            endpoint,
            raw_message_delivery: None,
            filter_policy: None,
            delivery_policy: None,
            redrive_policy: None,
            tags: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubscribeRequestInputProtocol {
    Email,
    Https,
    Http,
    Queue,
    Unknown(String),
}
impl SubscribeRequestInputProtocol {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Email => "email",
            Self::Https => "https",
            Self::Http => "http",
            Self::Queue => "queue",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SubscribeRequestInputProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubscribeRequestInputProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "email" => Self::Email,
            "https" => Self::Https,
            "http" => Self::Http,
            "queue" => Self::Queue,
            _ => Self::Unknown(value),
        })
    }
}

pub type FilterPolicyInput = std::collections::BTreeMap<String, Vec<serde_json::Value>>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RedrivePolicyInputInput {
    /// Queue UUID, CRN or exact name on input, scoped to the subscriber account and serving region. Empty on PATCH clears the target.
    #[serde(rename = "dead_letter_queue")]
    pub dead_letter_queue: String,
}
impl RedrivePolicyInputInput {
    /// Set the required fields; optional fields start omitted.
    #[allow(clippy::too_many_arguments)]
    pub fn new(dead_letter_queue: String) -> Self {
        Self { dead_letter_queue }
    }
}

pub type SubscribeBody = SubscribeRequestInput;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SubscribeResponse2 {
    #[serde(rename = "subscription")]
    pub subscription: Subscription,
    /// The HMAC key for verifying webhook deliveries. Returned only on the first create of an http/https subscription — absent for email, and absent when an existing subscription is returned.
    #[serde(
        rename = "signing_secret",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_secret: Option<String>,
}
impl std::fmt::Debug for SubscribeResponse2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("SubscribeResponse2");
        d.field("subscription", &self.subscription);
        d.field("signing_secret", &"[REDACTED]");
        d.finish()
    }
}

pub type SubscribeResponse = SubscribeResponse2;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateSubscriptionRequestInput {
    #[serde(
        rename = "raw_message_delivery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub raw_message_delivery: Option<bool>,

    #[serde(
        rename = "filter_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub filter_policy: Option<FilterPolicyInput>,

    #[serde(
        rename = "delivery_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_policy: Option<DeliveryPolicyInput>,
    /// Omitted leaves the dead-letter queue unchanged. Present with an empty `dead_letter_queue` clears it. Setting a new target re-checks `queue:SendMessage` on it.
    #[serde(
        rename = "redrive_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub redrive_policy: Option<RedrivePolicyInputInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateSubscriptionBody = UpdateSubscriptionRequestInput;

pub type UpdateSubscriptionResponse = SubscriptionResponse;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateTopicRequestInput {
    #[serde(
        rename = "display_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,

    #[serde(
        rename = "delivery_policy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_policy: Option<DeliveryPolicyInput>,

    #[serde(rename = "tags", default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsInput>,
}

pub type UpdateTopicBody = UpdateTopicRequestInput;

pub type UpdateTopicResponse = TopicResponse;
