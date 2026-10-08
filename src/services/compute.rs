//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::compute as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://compute.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "attachInstanceNIC",
    method: "POST",
    path: "/v1/instances/{instance_id}/nics",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "attachInstancePoolFloatingIp",
    method: "POST",
    path: "/v1/instance-pools/{pool_id}/floating-ips",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "attachInstanceVolume",
    method: "POST",
    path: "/v1/instances/{instance_id}/volumes",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "createImage",
    method: "POST",
    path: "/v1/images",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "createInstance",
    method: "POST",
    path: "/v1/instances",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "createInstancePool",
    method: "POST",
    path: "/v1/instance-pools",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "createSerialConsoleTicket",
    method: "POST",
    path: "/v1/instances/{instance_id}/console/ticket",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "deleteImage",
    method: "DELETE",
    path: "/v1/images/{image_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "deleteInstance",
    method: "DELETE",
    path: "/v1/instances/{instance_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "deleteInstancePool",
    method: "DELETE",
    path: "/v1/instance-pools/{pool_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "detachInstanceNIC",
    method: "DELETE",
    path: "/v1/instances/{instance_id}/nics/{interface_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "detachInstancePoolFloatingIp",
    method: "DELETE",
    path: "/v1/instance-pools/{pool_id}/floating-ips/{floating_ip_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "detachInstanceVolume",
    method: "DELETE",
    path: "/v1/instances/{instance_id}/volumes/{volume_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "getConsoleOutput",
    method: "GET",
    path: "/v1/instances/{instance_id}/console/output",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "max_bytes",
        style: "form",
        explode: true,
    }],
};
static OP_14: Operation = Operation {
    id: "getConsoleScreenshot",
    method: "GET",
    path: "/v1/instances/{instance_id}/console/screenshot",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "*/*",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "getFlavor",
    method: "GET",
    path: "/v1/flavors/{flavor_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "getImage",
    method: "GET",
    path: "/v1/images/{image_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "getInstance",
    method: "GET",
    path: "/v1/instances/{instance_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "getInstancePool",
    method: "GET",
    path: "/v1/instance-pools/{pool_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_19: Operation = Operation {
    id: "listFlavors",
    method: "GET",
    path: "/v1/flavors",
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
            name: "family",
            style: "form",
            explode: true,
        },
    ],
};
static OP_20: Operation = Operation {
    id: "listImageCatalog",
    method: "GET",
    path: "/v1/image-catalog",
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
            name: "os",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "architecture",
            style: "form",
            explode: true,
        },
    ],
};
static OP_21: Operation = Operation {
    id: "listImages",
    method: "GET",
    path: "/v1/images",
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
            name: "os",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "architecture",
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
            name: "all_versions",
            style: "form",
            explode: true,
        },
    ],
};
static OP_22: Operation = Operation {
    id: "listInstanceNICs",
    method: "GET",
    path: "/v1/instances/{instance_id}/nics",
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
    id: "listInstancePoolFloatingIps",
    method: "GET",
    path: "/v1/instance-pools/{pool_id}/floating-ips",
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
static OP_24: Operation = Operation {
    id: "listInstancePools",
    method: "GET",
    path: "/v1/instance-pools",
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
static OP_25: Operation = Operation {
    id: "listInstanceVolumes",
    method: "GET",
    path: "/v1/instances/{instance_id}/volumes",
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
static OP_26: Operation = Operation {
    id: "listInstances",
    method: "GET",
    path: "/v1/instances",
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
            name: "current_state",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "flavor",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "image",
            style: "form",
            explode: true,
        },
    ],
};
static OP_27: Operation = Operation {
    id: "listPoolInstances",
    method: "GET",
    path: "/v1/instance-pools/{pool_id}/instances",
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
            name: "current_state",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "flavor",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "image",
            style: "form",
            explode: true,
        },
    ],
};
static OP_28: Operation = Operation {
    id: "rebootInstance",
    method: "POST",
    path: "/v1/instances/{instance_id}/reboot",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_29: Operation = Operation {
    id: "refreshInstancePool",
    method: "POST",
    path: "/v1/instance-pools/{pool_id}/refresh",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_30: Operation = Operation {
    id: "reinstallInstance",
    method: "POST",
    path: "/v1/instances/{instance_id}/reinstall",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_31: Operation = Operation {
    id: "resizeInstance",
    method: "POST",
    path: "/v1/instances/{instance_id}/resize",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_32: Operation = Operation {
    id: "startInstance",
    method: "POST",
    path: "/v1/instances/{instance_id}/start",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_33: Operation = Operation {
    id: "startSerialConsole",
    method: "GET",
    path: "/v1/instances/{instance_id}/console/serial",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "backlog_bytes",
        style: "form",
        explode: true,
    }],
};
static OP_34: Operation = Operation {
    id: "stopInstance",
    method: "POST",
    path: "/v1/instances/{instance_id}/stop",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_35: Operation = Operation {
    id: "updateImage",
    method: "PATCH",
    path: "/v1/images/{image_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_36: Operation = Operation {
    id: "updateInstance",
    method: "PATCH",
    path: "/v1/instances/{instance_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_37: Operation = Operation {
    id: "updateInstancePool",
    method: "PATCH",
    path: "/v1/instance-pools/{pool_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_38: Operation = Operation {
    id: "updateInstanceVolumeAttachment",
    method: "PATCH",
    path: "/v1/instances/{instance_id}/volumes/{volume_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct ComputeService {
    pub(crate) client: Client,
}
impl ComputeService {
    /// Attach an existing NIC to an instance
    pub fn attach_instance_nic(
        &self,
        instance_id: &str,
        body: &m::AttachInstanceNICBody,
    ) -> Request<m::AttachInstanceNICResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_0,
            &[("instance_id", instance_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Give the pool a shared public address
    pub fn attach_instance_pool_floating_ip(
        &self,
        pool_id: &str,
        body: &m::AttachInstancePoolFloatingIpBody,
    ) -> Request<m::AttachInstancePoolFloatingIpResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_1,
            &[("pool_id", pool_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Attach a data volume to an instance
    pub fn attach_instance_volume(
        &self,
        instance_id: &str,
        body: &m::AttachInstanceVolumeBody,
    ) -> Request<m::AttachInstanceVolumeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_2,
            &[("instance_id", instance_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Import an image from an object URL
    pub fn create_image(&self, body: &m::CreateImageBody) -> Request<m::CreateImageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_3,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create instance
    pub fn create_instance(
        &self,
        body: &m::CreateInstanceBody,
    ) -> Request<m::CreateInstanceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_4,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create an instance pool
    pub fn create_instance_pool(
        &self,
        body: &m::CreateInstancePoolBody,
    ) -> Request<m::CreateInstancePoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_5,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Mint a ticket for the serial console
    pub fn create_serial_console_ticket(
        &self,
        instance_id: &str,
    ) -> Request<m::CreateSerialConsoleTicketResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_6,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete an unused image
    pub fn delete_image(&self, image_id: &str) -> Request<m::DeleteImageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_7,
            &[("image_id", image_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete instance
    pub fn delete_instance(&self, instance_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_8,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete an instance pool
    pub fn delete_instance_pool(&self, pool_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_9,
            &[("pool_id", pool_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach a NIC from a running instance
    pub fn detach_instance_nic(&self, instance_id: &str, interface_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_10,
            &[("instance_id", instance_id), ("interface_id", interface_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Take a shared address off the pool
    pub fn detach_instance_pool_floating_ip(
        &self,
        pool_id: &str,
        floating_ip_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_11,
            &[("pool_id", pool_id), ("floating_ip_id", floating_ip_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach a data volume from an instance
    pub fn detach_instance_volume(&self, instance_id: &str, volume_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_12,
            &[("instance_id", instance_id), ("volume_id", volume_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get the instance's serial console output
    pub fn get_console_output(
        &self,
        instance_id: &str,
        query: &m::GetConsoleOutputQuery,
    ) -> Request<m::GetConsoleOutputResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_13,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Capture the instance's display
    pub fn get_console_screenshot(&self, instance_id: &str) -> BinaryRequest {
        BinaryRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_14,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get flavor
    pub fn get_flavor(&self, flavor_id: &str) -> Request<m::GetFlavorResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_15,
            &[("flavor_id", flavor_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_flavor_by_reference(
        &self,
        reference: &str,
        scope: &m::GetFlavorScope,
    ) -> Request<m::GetFlavorResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_15,
                &[("flavor_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_19,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("flavor"),
            "flavors",
        );
        Request::reference(core, extract)
    }
    /// Get an image
    pub fn get_image(&self, image_id: &str) -> Request<m::GetImageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_16,
            &[("image_id", image_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_image_by_reference(
        &self,
        reference: &str,
        scope: &m::GetImageScope,
    ) -> Request<m::GetImageResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_16,
                &[("image_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_21,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("image"),
            "images",
        );
        Request::reference(core, extract)
    }
    /// Get instance
    pub fn get_instance(&self, instance_id: &str) -> Request<m::GetInstanceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_17,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_instance_by_reference(
        &self,
        reference: &str,
        scope: &m::GetInstanceScope,
    ) -> Request<m::GetInstanceResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_17,
                &[("instance_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_26,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("instance"),
            "instances",
        );
        Request::reference(core, extract)
    }
    /// Get an instance pool
    pub fn get_instance_pool(&self, pool_id: &str) -> Request<m::GetInstancePoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_18,
            &[("pool_id", pool_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_instance_pool_by_reference(
        &self,
        reference: &str,
        scope: &m::GetInstancePoolScope,
    ) -> Request<m::GetInstancePoolResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_18,
                &[("pool_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_24,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("instance_pool"),
            "instance_pools",
        );
        Request::reference(core, extract)
    }
    /// List flavors
    pub fn list_flavors(
        &self,
        query: &m::ListFlavorsQuery,
    ) -> PagedRequest<m::ListFlavorsResponse, m::ListFlavorsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_19,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "flavors",
        )
    }
    /// List the launch image catalog
    pub fn list_image_catalog(
        &self,
        query: &m::ListImageCatalogQuery,
    ) -> PagedRequest<m::ListImageCatalogResponse, m::ListImageCatalogItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_20,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "categories",
        )
    }
    /// List images
    pub fn list_images(
        &self,
        query: &m::ListImagesQuery,
    ) -> PagedRequest<m::ListImagesResponse, m::ListImagesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_21,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "images",
        )
    }
    /// List the instance's network interfaces
    pub fn list_instance_ni_cs(
        &self,
        instance_id: &str,
        query: &m::ListInstanceNICsQuery,
    ) -> PagedRequest<m::ListInstanceNICsResponse, m::ListInstanceNICsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_22,
                &[("instance_id", instance_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "nics",
        )
    }
    /// List the pool's shared public addresses
    pub fn list_instance_pool_floating_ips(
        &self,
        pool_id: &str,
        query: &m::ListInstancePoolFloatingIpsQuery,
    ) -> PagedRequest<m::ListInstancePoolFloatingIpsResponse, m::ListInstancePoolFloatingIpsItem>
    {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_23,
                &[("pool_id", pool_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "floating_ips",
        )
    }
    /// List instance pools
    pub fn list_instance_pools(
        &self,
        query: &m::ListInstancePoolsQuery,
    ) -> PagedRequest<m::ListInstancePoolsResponse, m::ListInstancePoolsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_24,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "instance_pools",
        )
    }
    /// List the instance's attached volumes
    pub fn list_instance_volumes(
        &self,
        instance_id: &str,
        query: &m::ListInstanceVolumesQuery,
    ) -> PagedRequest<m::ListInstanceVolumesResponse, m::ListInstanceVolumesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_25,
                &[("instance_id", instance_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "attachments",
        )
    }
    /// List instances
    pub fn list_instances(
        &self,
        query: &m::ListInstancesQuery,
    ) -> PagedRequest<m::ListInstancesResponse, m::ListInstancesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_26,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "instances",
        )
    }
    /// List a pool's instances
    pub fn list_pool_instances(
        &self,
        pool_id: &str,
        query: &m::ListPoolInstancesQuery,
    ) -> PagedRequest<m::ListPoolInstancesResponse, m::ListPoolInstancesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "compute",
                ENDPOINT,
                &OP_27,
                &[("pool_id", pool_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "instances",
        )
    }
    /// Reboot instance
    pub fn reboot_instance(
        &self,
        instance_id: &str,
        body: Option<&m::RebootInstanceBody>,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_28,
            &[("instance_id", instance_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Roll every member onto the pool's current launch template
    pub fn refresh_instance_pool(&self, pool_id: &str) -> Request<m::RefreshInstancePoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_29,
            &[("pool_id", pool_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Reinstall instance
    pub fn reinstall_instance(
        &self,
        instance_id: &str,
        body: Option<&m::ReinstallInstanceBody>,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_30,
            &[("instance_id", instance_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resize instance
    pub fn resize_instance(&self, instance_id: &str, body: &m::ResizeInstanceBody) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_31,
            &[("instance_id", instance_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Start instance
    pub fn start_instance(&self, instance_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_32,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Open an interactive serial console
    pub fn start_serial_console(
        &self,
        instance_id: &str,
        query: &m::StartSerialConsoleQuery,
    ) -> WebSocketRequest {
        WebSocketRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_33,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Stop instance
    pub fn stop_instance(&self, instance_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_34,
            &[("instance_id", instance_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update an image's metadata
    pub fn update_image(
        &self,
        image_id: &str,
        body: &m::UpdateImageBody,
    ) -> Request<m::UpdateImageResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_35,
            &[("image_id", image_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update instance
    pub fn update_instance(
        &self,
        instance_id: &str,
        body: &m::UpdateInstanceBody,
    ) -> Request<m::UpdateInstanceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_36,
            &[("instance_id", instance_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update an instance pool's description, size, tags or launch template
    pub fn update_instance_pool(
        &self,
        pool_id: &str,
        body: &m::UpdateInstancePoolBody,
    ) -> Request<m::UpdateInstancePoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_37,
            &[("pool_id", pool_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update a volume attachment's settings
    pub fn update_instance_volume_attachment(
        &self,
        instance_id: &str,
        volume_id: &str,
        body: &m::UpdateInstanceVolumeAttachmentBody,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "compute",
            ENDPOINT,
            &OP_38,
            &[("instance_id", instance_id), ("volume_id", volume_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
