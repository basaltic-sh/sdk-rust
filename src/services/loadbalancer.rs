//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::loadbalancer as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://loadbalancer.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "attachListenerCertificate",
    method: "POST",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}/certificates",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "attachTarget",
    method: "POST",
    path: "/v1/target-groups/{id}/targets",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "createListener",
    method: "POST",
    path: "/v1/load-balancers/{id}/listeners",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "createLoadBalancer",
    method: "POST",
    path: "/v1/load-balancers",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "createRule",
    method: "POST",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}/rules",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "createTargetGroup",
    method: "POST",
    path: "/v1/target-groups",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "deleteListener",
    method: "DELETE",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "deleteLoadBalancer",
    method: "DELETE",
    path: "/v1/load-balancers/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "deleteRuleInListener",
    method: "DELETE",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}/rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "deleteTargetGroup",
    method: "DELETE",
    path: "/v1/target-groups/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "detachListenerCertificate",
    method: "DELETE",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}/certificates/{certificate_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "detachTarget",
    method: "DELETE",
    path: "/v1/target-groups/{id}/targets/{target_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "getListener",
    method: "GET",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "getLoadBalancer",
    method: "GET",
    path: "/v1/load-balancers/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "getRule",
    method: "GET",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}/rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "getTarget",
    method: "GET",
    path: "/v1/target-groups/{id}/targets/{target_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "getTargetGroup",
    method: "GET",
    path: "/v1/target-groups/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "listListeners",
    method: "GET",
    path: "/v1/load-balancers/{id}/listeners",
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
static OP_18: Operation = Operation {
    id: "listLoadBalancerReplicas",
    method: "GET",
    path: "/v1/load-balancers/{id}/replicas",
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
static OP_19: Operation = Operation {
    id: "listLoadBalancers",
    method: "GET",
    path: "/v1/load-balancers",
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
            name: "status",
            style: "form",
            explode: true,
        },
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
    ],
};
static OP_20: Operation = Operation {
    id: "listRules",
    method: "GET",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}/rules",
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
static OP_21: Operation = Operation {
    id: "listTargetGroups",
    method: "GET",
    path: "/v1/target-groups",
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
            name: "protocol",
            style: "form",
            explode: true,
        },
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
    ],
};
static OP_22: Operation = Operation {
    id: "listTargets",
    method: "GET",
    path: "/v1/target-groups/{id}/targets",
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
static OP_23: Operation = Operation {
    id: "updateListener",
    method: "PATCH",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_24: Operation = Operation {
    id: "updateLoadBalancer",
    method: "PATCH",
    path: "/v1/load-balancers/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_25: Operation = Operation {
    id: "updateRule",
    method: "PATCH",
    path: "/v1/load-balancers/{id}/listeners/{listener_id}/rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_26: Operation = Operation {
    id: "updateTargetGroup",
    method: "PATCH",
    path: "/v1/target-groups/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct LoadbalancerService {
    pub(crate) client: Client,
}
impl LoadbalancerService {
    /// Attach an additional certificate to an HTTPS listener
    pub fn attach_listener_certificate(
        &self,
        id: &str,
        listener_id: &str,
        body: &m::AttachListenerCertificateBody,
    ) -> Request<m::AttachListenerCertificateResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_0,
            &[("id", id), ("listener_id", listener_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Attach a target to this group
    pub fn attach_target(
        &self,
        id: &str,
        body: &m::AttachTargetBody,
    ) -> Request<m::AttachTargetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_1,
            &[("id", id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a listener on this load balancer
    pub fn create_listener(
        &self,
        id: &str,
        body: &m::CreateListenerBody,
    ) -> Request<m::CreateListenerResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_2,
            &[("id", id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a load balancer
    pub fn create_load_balancer(
        &self,
        body: &m::CreateLoadBalancerBody,
    ) -> Request<m::CreateLoadBalancerResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_3,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a routing rule on this listener (HTTP/HTTPS only)
    pub fn create_rule(
        &self,
        id: &str,
        listener_id: &str,
        body: &m::CreateRuleBody,
    ) -> Request<m::CreateRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_4,
            &[("id", id), ("listener_id", listener_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create a target group
    pub fn create_target_group(
        &self,
        body: &m::CreateTargetGroupBody,
    ) -> Request<m::CreateTargetGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_5,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a listener
    pub fn delete_listener(&self, id: &str, listener_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_6,
            &[("id", id), ("listener_id", listener_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a load balancer
    pub fn delete_load_balancer(&self, id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_7,
            &[("id", id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a routing rule
    pub fn delete_rule_in_listener(
        &self,
        id: &str,
        listener_id: &str,
        rule_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_8,
            &[
                ("id", id),
                ("listener_id", listener_id),
                ("rule_id", rule_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a target group
    pub fn delete_target_group(&self, id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_9,
            &[("id", id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach a certificate from an HTTPS listener
    pub fn detach_listener_certificate(
        &self,
        id: &str,
        listener_id: &str,
        certificate_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_10,
            &[
                ("id", id),
                ("listener_id", listener_id),
                ("certificate_id", certificate_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach a target
    pub fn detach_target(&self, id: &str, target_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_11,
            &[("id", id), ("target_id", target_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get a listener
    pub fn get_listener(&self, id: &str, listener_id: &str) -> Request<m::GetListenerResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_12,
            &[("id", id), ("listener_id", listener_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_listener_by_reference(
        &self,
        id: &str,
        reference: &str,
        scope: &m::GetListenerScope,
    ) -> Request<m::GetListenerResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_12,
                &[("id", id), ("listener_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_17,
                &[("id", id)],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("listener"),
            "listeners",
        );
        Request::reference(core, extract)
    }
    /// Get a load balancer
    pub fn get_load_balancer(&self, id: &str) -> Request<m::GetLoadBalancerResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_13,
            &[("id", id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_load_balancer_by_reference(
        &self,
        reference: &str,
        scope: &m::GetLoadBalancerScope,
    ) -> Request<m::GetLoadBalancerResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_13,
                &[("id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_19,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("load_balancer"),
            "load_balancers",
        );
        Request::reference(core, extract)
    }
    /// Get a routing rule
    pub fn get_rule(
        &self,
        id: &str,
        listener_id: &str,
        rule_id: &str,
    ) -> Request<m::GetRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_14,
            &[
                ("id", id),
                ("listener_id", listener_id),
                ("rule_id", rule_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_rule_by_reference(
        &self,
        id: &str,
        listener_id: &str,
        reference: &str,
        scope: &m::GetRuleScope,
    ) -> Request<m::GetRuleResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_14,
                &[
                    ("id", id),
                    ("listener_id", listener_id),
                    ("rule_id", reference),
                ],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_20,
                &[("id", id), ("listener_id", listener_id)],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("rule"),
            "rules",
        );
        Request::reference(core, extract)
    }
    /// Get a target
    pub fn get_target(&self, id: &str, target_id: &str) -> Request<m::GetTargetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_15,
            &[("id", id), ("target_id", target_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_target_by_reference(
        &self,
        id: &str,
        reference: &str,
        scope: &m::GetTargetScope,
    ) -> Request<m::GetTargetResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_15,
                &[("id", id), ("target_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_22,
                &[("id", id)],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("target"),
            "targets",
        );
        Request::reference(core, extract)
    }
    /// Get a target group
    pub fn get_target_group(&self, id: &str) -> Request<m::GetTargetGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_16,
            &[("id", id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_target_group_by_reference(
        &self,
        reference: &str,
        scope: &m::GetTargetGroupScope,
    ) -> Request<m::GetTargetGroupResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_16,
                &[("id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_21,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("target_group"),
            "target_groups",
        );
        Request::reference(core, extract)
    }
    /// List this load balancer's listeners
    pub fn list_listeners(
        &self,
        id: &str,
        query: &m::ListListenersQuery,
    ) -> PagedRequest<m::ListListenersResponse, m::ListListenersItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_17,
                &[("id", id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "listeners",
        )
    }
    /// List the LB's instance replicas with live health
    pub fn list_load_balancer_replicas(
        &self,
        id: &str,
        query: &m::ListLoadBalancerReplicasQuery,
    ) -> PagedRequest<m::ListLoadBalancerReplicasResponse, m::ListLoadBalancerReplicasItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_18,
                &[("id", id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "replicas",
        )
    }
    /// List load balancers
    pub fn list_load_balancers(
        &self,
        query: &m::ListLoadBalancersQuery,
    ) -> PagedRequest<m::ListLoadBalancersResponse, m::ListLoadBalancersItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_19,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "load_balancers",
        )
    }
    /// List this listener's rules
    pub fn list_rules(
        &self,
        id: &str,
        listener_id: &str,
        query: &m::ListRulesQuery,
    ) -> PagedRequest<m::ListRulesResponse, m::ListRulesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_20,
                &[("id", id), ("listener_id", listener_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "rules",
        )
    }
    /// List target groups
    pub fn list_target_groups(
        &self,
        query: &m::ListTargetGroupsQuery,
    ) -> PagedRequest<m::ListTargetGroupsResponse, m::ListTargetGroupsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_21,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "target_groups",
        )
    }
    /// List targets in this group
    pub fn list_targets(
        &self,
        id: &str,
        query: &m::ListTargetsQuery,
    ) -> PagedRequest<m::ListTargetsResponse, m::ListTargetsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "loadbalancer",
                ENDPOINT,
                &OP_22,
                &[("id", id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "targets",
        )
    }
    /// Patch a listener (rotate cert, change default target group)
    pub fn update_listener(
        &self,
        id: &str,
        listener_id: &str,
        body: &m::UpdateListenerBody,
    ) -> Request<m::UpdateListenerResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_23,
            &[("id", id), ("listener_id", listener_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Scale or resize a load balancer
    pub fn update_load_balancer(
        &self,
        id: &str,
        body: &m::UpdateLoadBalancerBody,
    ) -> Request<m::UpdateLoadBalancerResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_24,
            &[("id", id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update a routing rule (full replace)
    pub fn update_rule(
        &self,
        id: &str,
        listener_id: &str,
        rule_id: &str,
        body: &m::UpdateRuleBody,
    ) -> Request<m::UpdateRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_25,
            &[
                ("id", id),
                ("listener_id", listener_id),
                ("rule_id", rule_id),
            ],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update target group health checks, framing, or stickiness
    pub fn update_target_group(
        &self,
        id: &str,
        body: &m::UpdateTargetGroupBody,
    ) -> Request<m::UpdateTargetGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "loadbalancer",
            ENDPOINT,
            &OP_26,
            &[("id", id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
