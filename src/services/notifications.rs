//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::notifications as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://notifications.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "createTopic",
    method: "POST",
    path: "/v1/topics",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "deleteTopic",
    method: "DELETE",
    path: "/v1/topics/{topic_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "deleteTopicPolicy",
    method: "DELETE",
    path: "/v1/topics/{topic_id}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "describeMessage",
    method: "GET",
    path: "/v1/topics/{topic_id}/messages/{message_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "describeSubscription",
    method: "GET",
    path: "/v1/topics/{topic_id}/subscriptions/{subscription_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "describeTopic",
    method: "GET",
    path: "/v1/topics/{topic_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "getSubscriptionSigningSecret",
    method: "GET",
    path: "/v1/topics/{topic_id}/subscriptions/{subscription_id}/signing-secret",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "getTopicPolicy",
    method: "GET",
    path: "/v1/topics/{topic_id}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "listDeliveries",
    method: "GET",
    path: "/v1/topics/{topic_id}/deliveries",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "message",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "subscription",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "status",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_9: Operation = Operation {
    id: "listMessages",
    method: "GET",
    path: "/v1/topics/{topic_id}/messages",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "fanout_state",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_10: Operation = Operation {
    id: "listSubscriptions",
    method: "GET",
    path: "/v1/topics/{topic_id}/subscriptions",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "status",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_11: Operation = Operation {
    id: "listTopics",
    method: "GET",
    path: "/v1/topics",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
    ],
};
static OP_12: Operation = Operation {
    id: "publish",
    method: "POST",
    path: "/v1/topics/{topic_id}/publish",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "publishBatch",
    method: "POST",
    path: "/v1/topics/{topic_id}/publish-batch",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "putTopicPolicy",
    method: "PUT",
    path: "/v1/topics/{topic_id}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "resendConfirmation",
    method: "POST",
    path: "/v1/topics/{topic_id}/subscriptions/{subscription_id}/resend-confirmation",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "rotateSubscriptionSigningSecret",
    method: "POST",
    path: "/v1/topics/{topic_id}/subscriptions/{subscription_id}/signing-secret/rotate",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "subscribe",
    method: "POST",
    path: "/v1/topics/{topic_id}/subscriptions",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "unsubscribe",
    method: "DELETE",
    path: "/v1/topics/{topic_id}/subscriptions/{subscription_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_19: Operation = Operation {
    id: "updateSubscription",
    method: "PATCH",
    path: "/v1/topics/{topic_id}/subscriptions/{subscription_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_20: Operation = Operation {
    id: "updateTopic",
    method: "PATCH",
    path: "/v1/topics/{topic_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct NotificationsService {
    pub(crate) client: Client,
}
impl NotificationsService {
    /// Create a topic
    pub fn create_topic(&self, body: &m::CreateTopicBody) -> Request<m::CreateTopicResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_0,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a topic
    pub fn delete_topic(&self, topic_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_1,
            &[("topic_id", topic_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete the topic's resource policy
    pub fn delete_topic_policy(&self, topic_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_2,
            &[("topic_id", topic_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a published notification
    pub fn describe_message(
        &self,
        topic_id: &str,
        message_id: &str,
    ) -> Request<m::DescribeMessageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_3,
            &[("topic_id", topic_id), ("message_id", message_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a subscription
    pub fn describe_subscription(
        &self,
        topic_id: &str,
        subscription_id: &str,
    ) -> Request<m::DescribeSubscriptionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_4,
            &[("topic_id", topic_id), ("subscription_id", subscription_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a topic
    pub fn describe_topic(&self, topic_id: &str) -> Request<m::DescribeTopicResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_5,
            &[("topic_id", topic_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Read the webhook signing secret
    pub fn get_subscription_signing_secret(
        &self,
        topic_id: &str,
        subscription_id: &str,
    ) -> Request<m::GetSubscriptionSigningSecretResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_6,
            &[("topic_id", topic_id), ("subscription_id", subscription_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Read the topic's resource policy
    pub fn get_topic_policy(&self, topic_id: &str) -> Request<m::GetTopicPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_7,
            &[("topic_id", topic_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List delivery records
    pub fn list_deliveries(
        &self,
        topic_id: &str,
        query: &m::ListDeliveriesQuery,
    ) -> PagedRequest<m::ListDeliveriesResponse, m::ListDeliveriesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "notifications",
                ENDPOINT,
                &OP_8,
                &[("topic_id", topic_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "deliveries",
        )
    }
    /// List messages
    pub fn list_messages(
        &self,
        topic_id: &str,
        query: &m::ListMessagesQuery,
    ) -> PagedRequest<m::ListMessagesResponse, m::ListMessagesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "notifications",
                ENDPOINT,
                &OP_9,
                &[("topic_id", topic_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "messages",
        )
    }
    /// List subscriptions
    pub fn list_subscriptions(
        &self,
        topic_id: &str,
        query: &m::ListSubscriptionsQuery,
    ) -> PagedRequest<m::ListSubscriptionsResponse, m::ListSubscriptionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "notifications",
                ENDPOINT,
                &OP_10,
                &[("topic_id", topic_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "subscriptions",
        )
    }
    /// List topics
    pub fn list_topics(
        &self,
        query: &m::ListTopicsQuery,
    ) -> PagedRequest<m::ListTopicsResponse, m::ListTopicsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "notifications",
                ENDPOINT,
                &OP_11,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "topics",
        )
    }
    /// Publish a notification
    pub fn publish(&self, topic_id: &str, body: &m::PublishBody) -> Request<m::PublishResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_12,
            &[("topic_id", topic_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Publish up to 10 notifications
    pub fn publish_batch(
        &self,
        topic_id: &str,
        body: &m::PublishBatchBody,
    ) -> Request<m::PublishBatchResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_13,
            &[("topic_id", topic_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set the topic's resource policy
    pub fn put_topic_policy(
        &self,
        topic_id: &str,
        body: &m::PutTopicPolicyBody,
    ) -> Request<m::PutTopicPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_14,
            &[("topic_id", topic_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Re-send the confirmation handshake
    pub fn resend_confirmation(
        &self,
        topic_id: &str,
        subscription_id: &str,
    ) -> Request<m::ResendConfirmationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_15,
            &[("topic_id", topic_id), ("subscription_id", subscription_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Rotate the webhook signing secret
    pub fn rotate_subscription_signing_secret(
        &self,
        topic_id: &str,
        subscription_id: &str,
    ) -> Request<m::RotateSubscriptionSigningSecretResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_16,
            &[("topic_id", topic_id), ("subscription_id", subscription_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Subscribe an endpoint to a topic
    pub fn subscribe(
        &self,
        topic_id: &str,
        body: &m::SubscribeBody,
    ) -> Request<m::SubscribeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_17,
            &[("topic_id", topic_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Unsubscribe
    pub fn unsubscribe(&self, topic_id: &str, subscription_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_18,
            &[("topic_id", topic_id), ("subscription_id", subscription_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update mutable subscription settings
    pub fn update_subscription(
        &self,
        topic_id: &str,
        subscription_id: &str,
        body: &m::UpdateSubscriptionBody,
    ) -> Request<m::UpdateSubscriptionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_19,
            &[("topic_id", topic_id), ("subscription_id", subscription_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update mutable topic metadata
    pub fn update_topic(
        &self,
        topic_id: &str,
        body: &m::UpdateTopicBody,
    ) -> Request<m::UpdateTopicResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "notifications",
            ENDPOINT,
            &OP_20,
            &[("topic_id", topic_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
