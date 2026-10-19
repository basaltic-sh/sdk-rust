//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::queue as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://queue.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "changeMessageVisibility",
    method: "POST",
    path: "/v1/queues/{queue_id}/messages/visibility",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "changeMessageVisibilityBatch",
    method: "POST",
    path: "/v1/queues/{queue_id}/messages/visibility-batch",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "createQueue",
    method: "POST",
    path: "/v1/queues",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "deleteMessage",
    method: "DELETE",
    path: "/v1/queues/{queue_id}/messages",
    authenticated: true,
    required_query: &["receipt_handle"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "receipt_handle",
        style: "form",
        explode: true,
    }],
};
static OP_4: Operation = Operation {
    id: "deleteMessageBatch",
    method: "POST",
    path: "/v1/queues/{queue_id}/messages/delete-batch",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "deleteQueue",
    method: "DELETE",
    path: "/v1/queues/{queue_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "describeQueue",
    method: "GET",
    path: "/v1/queues/{queue_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "listQueues",
    method: "GET",
    path: "/v1/queues",
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
static OP_8: Operation = Operation {
    id: "peekMessages",
    method: "GET",
    path: "/v1/queues/{queue_id}/messages",
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
            name: "visibility",
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
    id: "purgeQueue",
    method: "POST",
    path: "/v1/queues/{queue_id}/purge",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "receiveMessages",
    method: "POST",
    path: "/v1/queues/{queue_id}/messages/receive",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "sendMessage",
    method: "POST",
    path: "/v1/queues/{queue_id}/messages",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "sendMessageBatch",
    method: "POST",
    path: "/v1/queues/{queue_id}/messages/batch",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "updateQueue",
    method: "PATCH",
    path: "/v1/queues/{queue_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct QueueService {
    pub(crate) client: Client,
}
impl QueueService {
    /// Extend or shorten a lease
    pub fn change_message_visibility(
        &self,
        queue_id: &str,
        body: &m::ChangeMessageVisibilityBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_0,
            &[("queue_id", queue_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Extend or shorten up to ten leases
    pub fn change_message_visibility_batch(
        &self,
        queue_id: &str,
        body: &m::ChangeMessageVisibilityBatchBody,
    ) -> Request<m::ChangeMessageVisibilityBatchResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_1,
            &[("queue_id", queue_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a queue
    pub fn create_queue(&self, body: &m::CreateQueueBody) -> Request<m::CreateQueueResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_2,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a leased message
    pub fn delete_message(&self, queue_id: &str, query: &m::DeleteMessageQuery) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_3,
            &[("queue_id", queue_id)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Delete up to ten leased messages
    pub fn delete_message_batch(
        &self,
        queue_id: &str,
        body: &m::DeleteMessageBatchBody,
    ) -> Request<m::DeleteMessageBatchResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_4,
            &[("queue_id", queue_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a queue
    pub fn delete_queue(&self, queue_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_5,
            &[("queue_id", queue_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe a queue
    pub fn describe_queue(&self, queue_id: &str) -> Request<m::DescribeQueueResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_6,
            &[("queue_id", queue_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List queues
    pub fn list_queues(
        &self,
        query: &m::ListQueuesQuery,
    ) -> PagedRequest<m::ListQueuesResponse, m::ListQueuesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "queue",
                ENDPOINT,
                &OP_7,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "queues",
        )
    }
    /// Peek at a queue's messages
    pub fn peek_messages(
        &self,
        queue_id: &str,
        query: &m::PeekMessagesQuery,
    ) -> Request<m::PeekMessagesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_8,
            &[("queue_id", queue_id)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Delete every message on a queue
    pub fn purge_queue(&self, queue_id: &str) -> Request<m::PurgeQueueResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_9,
            &[("queue_id", queue_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Lease messages
    pub fn receive_messages(
        &self,
        queue_id: &str,
        body: Option<&m::ReceiveMessagesBody>,
    ) -> Request<m::ReceiveMessagesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_10,
            &[("queue_id", queue_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Send a message
    pub fn send_message(
        &self,
        queue_id: &str,
        body: &m::SendMessageBody,
    ) -> Request<m::SendMessageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_11,
            &[("queue_id", queue_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Send up to ten messages
    pub fn send_message_batch(
        &self,
        queue_id: &str,
        body: &m::SendMessageBatchBody,
    ) -> Request<m::SendMessageBatchResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_12,
            &[("queue_id", queue_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update queue attributes
    pub fn update_queue(
        &self,
        queue_id: &str,
        body: &m::UpdateQueueBody,
    ) -> Request<m::UpdateQueueResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "queue",
            ENDPOINT,
            &OP_13,
            &[("queue_id", queue_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
