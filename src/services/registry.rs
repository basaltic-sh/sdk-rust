//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::registry as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://registry.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "batchDeleteImage",
    method: "POST",
    path: "/v1/repositories/{repository_id}/images/batch-delete",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "createPullThroughCacheRule",
    method: "POST",
    path: "/v1/pull-through-cache-rules",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "createRepository",
    method: "POST",
    path: "/v1/repositories",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "deleteEncryptionConfiguration",
    method: "DELETE",
    path: "/v1/encryption-configuration",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "deleteLifecyclePolicy",
    method: "DELETE",
    path: "/v1/repositories/{repository_id}/lifecycle-policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "deletePullThroughCacheRule",
    method: "DELETE",
    path: "/v1/pull-through-cache-rules/{prefix}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "deleteRepository",
    method: "DELETE",
    path: "/v1/repositories/{repository_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "force",
        style: "form",
        explode: true,
    }],
};
static OP_7: Operation = Operation {
    id: "deleteRepositoryPolicy",
    method: "DELETE",
    path: "/v1/repositories/{repository_id}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "describeImage",
    method: "GET",
    path: "/v1/repositories/{repository_id}/images/{image_reference}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "describeImages",
    method: "GET",
    path: "/v1/repositories/{repository_id}/images",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "tag_status",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "tag_prefix",
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
    id: "describePullThroughCacheRule",
    method: "GET",
    path: "/v1/pull-through-cache-rules/{prefix}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "describePullThroughCacheRules",
    method: "GET",
    path: "/v1/pull-through-cache-rules",
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
static OP_12: Operation = Operation {
    id: "describeRepositories",
    method: "GET",
    path: "/v1/repositories",
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
static OP_13: Operation = Operation {
    id: "describeRepository",
    method: "GET",
    path: "/v1/repositories/{repository_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "getAuthorizationToken",
    method: "POST",
    path: "/v1/authorization-token",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "getEncryptionConfiguration",
    method: "GET",
    path: "/v1/encryption-configuration",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "getLifecyclePolicy",
    method: "GET",
    path: "/v1/repositories/{repository_id}/lifecycle-policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "getRepositoryPolicy",
    method: "GET",
    path: "/v1/repositories/{repository_id}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "getRetainedRegistryPresence",
    method: "GET",
    path: "/v1/encryption-configuration/retained-data",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_19: Operation = Operation {
    id: "listTags",
    method: "GET",
    path: "/v1/repositories/{repository_id}/tags",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
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
static OP_20: Operation = Operation {
    id: "putEncryptionConfiguration",
    method: "PUT",
    path: "/v1/encryption-configuration",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_21: Operation = Operation {
    id: "putLifecyclePolicy",
    method: "PUT",
    path: "/v1/repositories/{repository_id}/lifecycle-policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_22: Operation = Operation {
    id: "setRepositoryPolicy",
    method: "PUT",
    path: "/v1/repositories/{repository_id}/policy",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_23: Operation = Operation {
    id: "startLifecyclePolicyPreview",
    method: "POST",
    path: "/v1/repositories/{repository_id}/lifecycle-policy/preview",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_24: Operation = Operation {
    id: "updatePullThroughCacheRule",
    method: "PATCH",
    path: "/v1/pull-through-cache-rules/{prefix}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_25: Operation = Operation {
    id: "updateRepository",
    method: "PATCH",
    path: "/v1/repositories/{repository_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct RegistryService {
    pub(crate) client: Client,
}
impl RegistryService {
    /// Delete images
    pub fn batch_delete_image(
        &self,
        repository_id: &str,
        body: &m::BatchDeleteImageBody,
    ) -> Request<m::BatchDeleteImageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_0,
            &[("repository_id", repository_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a pull-through cache rule
    pub fn create_pull_through_cache_rule(
        &self,
        body: &m::CreatePullThroughCacheRuleBody,
    ) -> Request<m::CreatePullThroughCacheRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_1,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a repository
    pub fn create_repository(
        &self,
        body: &m::CreateRepositoryBody,
    ) -> Request<m::CreateRepositoryResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_2,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete encryption configuration
    pub fn delete_encryption_configuration(&self) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_3,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete the lifecycle policy
    pub fn delete_lifecycle_policy(&self, repository_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_4,
            &[("repository_id", repository_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a pull-through cache rule
    pub fn delete_pull_through_cache_rule(&self, prefix: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_5,
            &[("prefix", prefix)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a repository
    pub fn delete_repository(
        &self,
        repository_id: &str,
        query: &m::DeleteRepositoryQuery,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_6,
            &[("repository_id", repository_id)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Delete the repository policy
    pub fn delete_repository_policy(&self, repository_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_7,
            &[("repository_id", repository_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Describe an image
    pub fn describe_image(
        &self,
        repository_id: &str,
        image_reference: &str,
    ) -> Request<m::DescribeImageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_8,
            &[
                ("repository_id", repository_id),
                ("image_reference", image_reference),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List images
    pub fn describe_images(
        &self,
        repository_id: &str,
        query: &m::DescribeImagesQuery,
    ) -> Request<m::DescribeImagesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_9,
            &[("repository_id", repository_id)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Get a pull-through cache rule
    pub fn describe_pull_through_cache_rule(
        &self,
        prefix: &str,
    ) -> Request<m::DescribePullThroughCacheRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_10,
            &[("prefix", prefix)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List pull-through cache rules
    pub fn describe_pull_through_cache_rules(
        &self,
        query: &m::DescribePullThroughCacheRulesQuery,
    ) -> Request<m::DescribePullThroughCacheRulesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_11,
            &[],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// List repositories
    pub fn describe_repositories(
        &self,
        query: &m::DescribeRepositoriesQuery,
    ) -> Request<m::DescribeRepositoriesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_12,
            &[],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Describe a repository
    pub fn describe_repository(
        &self,
        repository_id: &str,
    ) -> Request<m::DescribeRepositoryResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_13,
            &[("repository_id", repository_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get a docker login credential
    pub fn get_authorization_token(&self) -> Request<m::GetAuthorizationTokenResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_14,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get the encryption configuration
    pub fn get_encryption_configuration(&self) -> Request<m::GetEncryptionConfigurationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_15,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get the lifecycle policy
    pub fn get_lifecycle_policy(
        &self,
        repository_id: &str,
    ) -> Request<m::GetLifecyclePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_16,
            &[("repository_id", repository_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get the repository policy
    pub fn get_repository_policy(
        &self,
        repository_id: &str,
    ) -> Request<m::GetRepositoryPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_17,
            &[("repository_id", repository_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Check retained registry data presence
    pub fn get_retained_registry_presence(
        &self,
    ) -> Request<m::GetRetainedRegistryPresenceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_18,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// List tags
    pub fn list_tags(
        &self,
        repository_id: &str,
        query: &m::ListTagsQuery,
    ) -> PagedRequest<m::ListTagsResponse, m::ListTagsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "registry",
                ENDPOINT,
                &OP_19,
                &[("repository_id", repository_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "tags",
        )
    }
    /// Set the encryption configuration
    pub fn put_encryption_configuration(
        &self,
        body: &m::PutEncryptionConfigurationBody,
    ) -> Request<m::PutEncryptionConfigurationResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_20,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set the lifecycle policy
    pub fn put_lifecycle_policy(
        &self,
        repository_id: &str,
        body: &m::PutLifecyclePolicyBody,
    ) -> Request<m::PutLifecyclePolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_21,
            &[("repository_id", repository_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Set the repository policy
    pub fn set_repository_policy(
        &self,
        repository_id: &str,
        body: &m::SetRepositoryPolicyBody,
    ) -> Request<m::SetRepositoryPolicyResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_22,
            &[("repository_id", repository_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Preview a lifecycle policy
    pub fn start_lifecycle_policy_preview(
        &self,
        repository_id: &str,
        body: Option<&m::StartLifecyclePolicyPreviewBody>,
    ) -> Request<m::StartLifecyclePolicyPreviewResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_23,
            &[("repository_id", repository_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update a pull-through cache rule
    pub fn update_pull_through_cache_rule(
        &self,
        prefix: &str,
        body: &m::UpdatePullThroughCacheRuleBody,
    ) -> Request<m::UpdatePullThroughCacheRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_24,
            &[("prefix", prefix)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update repository settings
    pub fn update_repository(
        &self,
        repository_id: &str,
        body: &m::UpdateRepositoryBody,
    ) -> Request<m::UpdateRepositoryResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "registry",
            ENDPOINT,
            &OP_25,
            &[("repository_id", repository_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
