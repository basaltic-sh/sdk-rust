//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::storage as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://storage.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "abortMultipartUpload",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/multipart-uploads/{upload_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "completeMultipartUpload",
    method: "POST",
    path: "/v1/buckets/{bucket}/multipart-uploads/{upload_id}/complete",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "createBucket",
    method: "POST",
    path: "/v1/buckets",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "createSnapshot",
    method: "POST",
    path: "/v1/snapshots",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "createSnapshotPolicy",
    method: "POST",
    path: "/v1/snapshot-policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "createVolume",
    method: "POST",
    path: "/v1/volumes",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "deleteBucket",
    method: "DELETE",
    path: "/v1/buckets/{bucket}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "deleteBucketCORS",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/cors",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "deleteBucketEncryption",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/encryption",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "deleteBucketLifecycle",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/lifecycle",
    authenticated: true,
    required_query: &[],
    required_headers: &["If-Match"],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "deleteBucketObjectLock",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/object-lock",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "deleteBucketPolicy",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "deleteBucketTagging",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/tagging",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "deleteObject",
    method: "DELETE",
    path: "/v1/buckets/{bucket}/objects/{key}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "deleteSnapshot",
    method: "DELETE",
    path: "/v1/snapshots/{snapshot_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "deleteSnapshotPolicy",
    method: "DELETE",
    path: "/v1/snapshot-policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "deleteVolume",
    method: "DELETE",
    path: "/v1/volumes/{volume_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "extendVolume",
    method: "POST",
    path: "/v1/volumes/{volume_id}/extend",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "getBucketCORS",
    method: "GET",
    path: "/v1/buckets/{bucket}/cors",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_19: Operation = Operation {
    id: "getBucketEncryption",
    method: "GET",
    path: "/v1/buckets/{bucket}/encryption",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_20: Operation = Operation {
    id: "getBucketLifecycle",
    method: "GET",
    path: "/v1/buckets/{bucket}/lifecycle",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_21: Operation = Operation {
    id: "getBucketObjectLock",
    method: "GET",
    path: "/v1/buckets/{bucket}/object-lock",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_22: Operation = Operation {
    id: "getBucketPolicy",
    method: "GET",
    path: "/v1/buckets/{bucket}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_23: Operation = Operation {
    id: "getBucketTagging",
    method: "GET",
    path: "/v1/buckets/{bucket}/tagging",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_24: Operation = Operation {
    id: "getBucketVersioning",
    method: "GET",
    path: "/v1/buckets/{bucket}/versioning",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_25: Operation = Operation {
    id: "getObject",
    method: "GET",
    path: "/v1/buckets/{bucket}/objects/{key}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_26: Operation = Operation {
    id: "getSnapshot",
    method: "GET",
    path: "/v1/snapshots/{snapshot_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_27: Operation = Operation {
    id: "getSnapshotPolicy",
    method: "GET",
    path: "/v1/snapshot-policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_28: Operation = Operation {
    id: "getVolume",
    method: "GET",
    path: "/v1/volumes/{volume_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_29: Operation = Operation {
    id: "headBucket",
    method: "HEAD",
    path: "/v1/buckets/{bucket}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_30: Operation = Operation {
    id: "headObject",
    method: "HEAD",
    path: "/v1/buckets/{bucket}/objects/{key}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_31: Operation = Operation {
    id: "initiateMultipartUpload",
    method: "POST",
    path: "/v1/buckets/{bucket}/multipart-uploads",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_32: Operation = Operation {
    id: "listBuckets",
    method: "GET",
    path: "/v1/buckets",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
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
    ],
};
static OP_33: Operation = Operation {
    id: "listMultipartUploads",
    method: "GET",
    path: "/v1/buckets/{bucket}/multipart-uploads",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "prefix",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "max_uploads",
            style: "form",
            explode: true,
        },
    ],
};
static OP_34: Operation = Operation {
    id: "listObjectVersions",
    method: "GET",
    path: "/v1/buckets/{bucket}/object-versions",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "prefix",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "key_marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "version_id_marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "max_keys",
            style: "form",
            explode: true,
        },
    ],
};
static OP_35: Operation = Operation {
    id: "listObjects",
    method: "GET",
    path: "/v1/buckets/{bucket}/objects",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "prefix",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "delimiter",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "max_keys",
            style: "form",
            explode: true,
        },
    ],
};
static OP_36: Operation = Operation {
    id: "listParts",
    method: "GET",
    path: "/v1/buckets/{bucket}/multipart-uploads/{upload_id}/parts",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_37: Operation = Operation {
    id: "listSnapshotPolicies",
    method: "GET",
    path: "/v1/snapshot-policies",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "volume",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "enabled",
            style: "form",
            explode: true,
        },
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
    ],
};
static OP_38: Operation = Operation {
    id: "listSnapshots",
    method: "GET",
    path: "/v1/snapshots",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "volume",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "status",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "snapshot_policy",
            style: "form",
            explode: true,
        },
    ],
};
static OP_39: Operation = Operation {
    id: "listVolumeTypes",
    method: "GET",
    path: "/v1/volume-types",
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
    ],
};
static OP_40: Operation = Operation {
    id: "listVolumes",
    method: "GET",
    path: "/v1/volumes",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "limit",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "marker",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "name",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "status",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "crn",
            style: "form",
            explode: true,
        },
    ],
};
static OP_41: Operation = Operation {
    id: "putBucketCORS",
    method: "PUT",
    path: "/v1/buckets/{bucket}/cors",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_42: Operation = Operation {
    id: "putBucketDeletionProtection",
    method: "PUT",
    path: "/v1/buckets/{bucket}/deletion-protection",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_43: Operation = Operation {
    id: "putBucketEncryption",
    method: "PUT",
    path: "/v1/buckets/{bucket}/encryption",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_44: Operation = Operation {
    id: "putBucketLifecycle",
    method: "PUT",
    path: "/v1/buckets/{bucket}/lifecycle",
    authenticated: true,
    required_query: &[],
    required_headers: &["If-Match"],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_45: Operation = Operation {
    id: "putBucketObjectLock",
    method: "PUT",
    path: "/v1/buckets/{bucket}/object-lock",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_46: Operation = Operation {
    id: "putBucketPolicy",
    method: "PUT",
    path: "/v1/buckets/{bucket}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_47: Operation = Operation {
    id: "putBucketTagging",
    method: "PUT",
    path: "/v1/buckets/{bucket}/tagging",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_48: Operation = Operation {
    id: "putBucketVersioning",
    method: "PUT",
    path: "/v1/buckets/{bucket}/versioning",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_49: Operation = Operation {
    id: "putObject",
    method: "PUT",
    path: "/v1/buckets/{bucket}/objects/{key}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/octet-stream",
    accept: "application/json",
    query_encoding: &[],
};
static OP_50: Operation = Operation {
    id: "restoreBucket",
    method: "POST",
    path: "/v1/buckets/{bucket}/restore",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_51: Operation = Operation {
    id: "updateSnapshot",
    method: "PATCH",
    path: "/v1/snapshots/{snapshot_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_52: Operation = Operation {
    id: "updateSnapshotPolicy",
    method: "PATCH",
    path: "/v1/snapshot-policies/{policy_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_53: Operation = Operation {
    id: "updateVolume",
    method: "PATCH",
    path: "/v1/volumes/{volume_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_54: Operation = Operation {
    id: "updateVolumePerformance",
    method: "POST",
    path: "/v1/volumes/{volume_id}/performance",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_55: Operation = Operation {
    id: "uploadPart",
    method: "PUT",
    path: "/v1/buckets/{bucket}/multipart-uploads/{upload_id}/parts/{part_number}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/octet-stream",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct StorageService {
    pub(crate) client: Client,
}
impl StorageService {
    /// Abort a multipart upload
    pub fn abort_multipart_upload(&self, bucket: &str, upload_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_0,
            &[("bucket", bucket), ("upload_id", upload_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Complete a multipart upload
    pub fn complete_multipart_upload(
        &self,
        bucket: &str,
        upload_id: &str,
        body: &m::CompleteMultipartUploadBody,
    ) -> Request<m::CompleteMultipartUploadResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_1,
            &[("bucket", bucket), ("upload_id", upload_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create bucket
    pub fn create_bucket(&self, body: &m::CreateBucketBody) -> Request<m::CreateBucketResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_2,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create snapshot
    pub fn create_snapshot(
        &self,
        body: &m::CreateSnapshotBody,
    ) -> Request<m::CreateSnapshotResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_3,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create snapshot policy
    pub fn create_snapshot_policy(
        &self,
        body: &m::CreateSnapshotPolicyBody,
    ) -> Request<m::CreateSnapshotPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_4,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create volume
    pub fn create_volume(&self, body: &m::CreateVolumeBody) -> Request<m::CreateVolumeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_5,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete bucket
    pub fn delete_bucket(&self, bucket: &str) -> Request<m::DeleteBucketResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_6,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete bucket CORS configuration
    pub fn delete_bucket_cors(&self, bucket: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_7,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete bucket encryption configuration
    pub fn delete_bucket_encryption(&self, bucket: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_8,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete bucket lifecycle configuration
    pub fn delete_bucket_lifecycle(&self, bucket: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_9,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete bucket object-lock configuration
    pub fn delete_bucket_object_lock(&self, bucket: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_10,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete bucket policy
    pub fn delete_bucket_policy(&self, bucket: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_11,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete bucket tag set
    pub fn delete_bucket_tagging(&self, bucket: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_12,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete object
    pub fn delete_object(&self, bucket: &str, key: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_13,
            &[("bucket", bucket), ("key", key)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete snapshot
    pub fn delete_snapshot(&self, snapshot_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_14,
            &[("snapshot_id", snapshot_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete snapshot policy
    pub fn delete_snapshot_policy(&self, policy_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_15,
            &[("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete volume
    pub fn delete_volume(&self, volume_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_16,
            &[("volume_id", volume_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Extend volume
    pub fn extend_volume(
        &self,
        volume_id: &str,
        body: &m::ExtendVolumeBody,
    ) -> Request<m::ExtendVolumeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_17,
            &[("volume_id", volume_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get bucket CORS configuration
    pub fn get_bucket_cors(&self, bucket: &str) -> Request<m::GetBucketCORSResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_18,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get bucket encryption configuration
    pub fn get_bucket_encryption(&self, bucket: &str) -> Request<m::GetBucketEncryptionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_19,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get bucket lifecycle configuration
    pub fn get_bucket_lifecycle(&self, bucket: &str) -> Request<m::GetBucketLifecycleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_20,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get bucket object-lock configuration
    pub fn get_bucket_object_lock(&self, bucket: &str) -> Request<m::GetBucketObjectLockResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_21,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get bucket policy
    pub fn get_bucket_policy(&self, bucket: &str) -> Request<m::GetBucketPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_22,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get bucket tag set
    pub fn get_bucket_tagging(&self, bucket: &str) -> Request<m::GetBucketTaggingResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_23,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get bucket versioning state
    pub fn get_bucket_versioning(&self, bucket: &str) -> Request<m::GetBucketVersioningResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_24,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Download object
    pub fn get_object(&self, bucket: &str, key: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_25,
            &[("bucket", bucket), ("key", key)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get snapshot
    pub fn get_snapshot(&self, snapshot_id: &str) -> Request<m::GetSnapshotResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_26,
            &[("snapshot_id", snapshot_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_snapshot_by_reference(
        &self,
        reference: &str,
        scope: &m::GetSnapshotScope,
    ) -> Request<m::GetSnapshotResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_26,
                &[("snapshot_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_38,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("snapshot"),
            "snapshots",
        );
        Request::reference(core, extract)
    }
    /// Get snapshot policy
    pub fn get_snapshot_policy(&self, policy_id: &str) -> Request<m::GetSnapshotPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_27,
            &[("policy_id", policy_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_snapshot_policy_by_reference(
        &self,
        reference: &str,
        scope: &m::GetSnapshotPolicyScope,
    ) -> Request<m::GetSnapshotPolicyResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_27,
                &[("policy_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_37,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("snapshot_policy"),
            "snapshot_policies",
        );
        Request::reference(core, extract)
    }
    /// Get volume
    pub fn get_volume(&self, volume_id: &str) -> Request<m::GetVolumeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_28,
            &[("volume_id", volume_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_volume_by_reference(
        &self,
        reference: &str,
        scope: &m::GetVolumeScope,
    ) -> Request<m::GetVolumeResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_28,
                &[("volume_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_40,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("volume"),
            "volumes",
        );
        Request::reference(core, extract)
    }
    /// Head bucket
    pub fn head_bucket(&self, bucket: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_29,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Head object
    pub fn head_object(&self, bucket: &str, key: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_30,
            &[("bucket", bucket), ("key", key)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Initiate a multipart upload
    pub fn initiate_multipart_upload(
        &self,
        bucket: &str,
        body: &m::InitiateMultipartUploadBody,
    ) -> Request<m::InitiateMultipartUploadResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_31,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// List buckets
    pub fn list_buckets(
        &self,
        query: &m::ListBucketsQuery,
    ) -> PagedRequest<m::ListBucketsResponse, m::ListBucketsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_32,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "buckets",
        )
    }
    /// List in-flight multipart uploads
    pub fn list_multipart_uploads(
        &self,
        bucket: &str,
        query: &m::ListMultipartUploadsQuery,
    ) -> PagedRequest<m::ListMultipartUploadsResponse, m::ListMultipartUploadsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_33,
                &[("bucket", bucket)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "uploads",
        )
    }
    /// List object versions
    pub fn list_object_versions(
        &self,
        bucket: &str,
        query: &m::ListObjectVersionsQuery,
    ) -> PagedRequest<m::ListObjectVersionsResponse, m::ListObjectVersionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_34,
                &[("bucket", bucket)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "versions",
        )
    }
    /// List objects
    pub fn list_objects(
        &self,
        bucket: &str,
        query: &m::ListObjectsQuery,
    ) -> Request<m::ListObjectsResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_35,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// List uploaded parts
    pub fn list_parts(
        &self,
        bucket: &str,
        upload_id: &str,
    ) -> PagedRequest<m::ListPartsResponse, m::ListPartsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_36,
                &[("bucket", bucket), ("upload_id", upload_id)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "parts",
        )
    }
    /// List snapshot policies
    pub fn list_snapshot_policies(
        &self,
        query: &m::ListSnapshotPoliciesQuery,
    ) -> PagedRequest<m::ListSnapshotPoliciesResponse, m::ListSnapshotPoliciesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_37,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "snapshot_policies",
        )
    }
    /// List snapshots
    pub fn list_snapshots(
        &self,
        query: &m::ListSnapshotsQuery,
    ) -> PagedRequest<m::ListSnapshotsResponse, m::ListSnapshotsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_38,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "snapshots",
        )
    }
    /// List volume types
    pub fn list_volume_types(
        &self,
        query: &m::ListVolumeTypesQuery,
    ) -> PagedRequest<m::ListVolumeTypesResponse, m::ListVolumeTypesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_39,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "volume_types",
        )
    }
    /// List volumes
    pub fn list_volumes(
        &self,
        query: &m::ListVolumesQuery,
    ) -> PagedRequest<m::ListVolumesResponse, m::ListVolumesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "storage",
                ENDPOINT,
                &OP_40,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "volumes",
        )
    }
    /// Put bucket CORS configuration
    pub fn put_bucket_cors(&self, bucket: &str, body: &m::PutBucketCORSBody) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_41,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set bucket deletion protection
    pub fn put_bucket_deletion_protection(
        &self,
        bucket: &str,
        body: &m::PutBucketDeletionProtectionBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_42,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Put bucket encryption configuration
    pub fn put_bucket_encryption(
        &self,
        bucket: &str,
        body: &m::PutBucketEncryptionBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_43,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Put bucket lifecycle configuration
    pub fn put_bucket_lifecycle(
        &self,
        bucket: &str,
        body: &m::PutBucketLifecycleBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_44,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Put bucket object-lock configuration
    pub fn put_bucket_object_lock(
        &self,
        bucket: &str,
        body: &m::PutBucketObjectLockBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_45,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Put bucket policy
    pub fn put_bucket_policy(&self, bucket: &str, body: &m::PutBucketPolicyBody) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_46,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Put bucket tag set
    pub fn put_bucket_tagging(&self, bucket: &str, body: &m::PutBucketTaggingBody) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_47,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set bucket versioning state
    pub fn put_bucket_versioning(
        &self,
        bucket: &str,
        body: &m::PutBucketVersioningBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_48,
            &[("bucket", bucket)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Upload object
    pub fn put_object(
        &self,
        bucket: &str,
        key: &str,
        body: reqwest::Body,
    ) -> Request<m::PutObjectResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_49,
            &[("bucket", bucket), ("key", key)],
            Ok(Payload::binary(body)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Restore a bucket pending deletion
    pub fn restore_bucket(&self, bucket: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_50,
            &[("bucket", bucket)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update snapshot metadata
    pub fn update_snapshot(
        &self,
        snapshot_id: &str,
        body: &m::UpdateSnapshotBody,
    ) -> Request<m::UpdateSnapshotResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_51,
            &[("snapshot_id", snapshot_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update snapshot policy
    pub fn update_snapshot_policy(
        &self,
        policy_id: &str,
        body: &m::UpdateSnapshotPolicyBody,
    ) -> Request<m::UpdateSnapshotPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_52,
            &[("policy_id", policy_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update volume metadata
    pub fn update_volume(
        &self,
        volume_id: &str,
        body: &m::UpdateVolumeBody,
    ) -> Request<m::UpdateVolumeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_53,
            &[("volume_id", volume_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update provisioned performance
    pub fn update_volume_performance(
        &self,
        volume_id: &str,
        body: &m::UpdateVolumePerformanceBody,
    ) -> Request<m::UpdateVolumePerformanceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_54,
            &[("volume_id", volume_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Upload a part
    pub fn upload_part(
        &self,
        bucket: &str,
        upload_id: &str,
        part_number: &str,
        body: reqwest::Body,
    ) -> Request<m::UploadPartResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "storage",
            ENDPOINT,
            &OP_55,
            &[
                ("bucket", bucket),
                ("upload_id", upload_id),
                ("part_number", part_number),
            ],
            Ok(Payload::binary(body)),
            Ok(serde_json::json!({})),
        ))
    }
}
