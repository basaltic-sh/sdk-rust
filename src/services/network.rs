//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::network as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://network.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "attachFloatingIp",
    method: "POST",
    path: "/v1/floating-ips/{floating_ip_id}/attach",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "attachInternetGateway",
    method: "POST",
    path: "/v1/internet-gateways/{internet_gateway_id}/attach",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "createEgressOnlyGateway",
    method: "POST",
    path: "/v1/egress-only-gateways",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "createFloatingIp",
    method: "POST",
    path: "/v1/floating-ips",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "createInterface",
    method: "POST",
    path: "/v1/interfaces",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "createInterfaceAddress",
    method: "POST",
    path: "/v1/interfaces/{interface_id}/addresses",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "createInterfacePrefix",
    method: "POST",
    path: "/v1/interfaces/{interface_id}/prefixes",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "createInternetGateway",
    method: "POST",
    path: "/v1/internet-gateways",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "createNATGateway",
    method: "POST",
    path: "/v1/nat-gateways",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "createPrefixPool",
    method: "POST",
    path: "/v1/vpcs/{vpc_id}/prefix-pools",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "createRoute",
    method: "POST",
    path: "/v1/route-tables/{route_table_id}/routes",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_11: Operation = Operation {
    id: "createRouteTable",
    method: "POST",
    path: "/v1/route-tables",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_12: Operation = Operation {
    id: "createSecurityGroup",
    method: "POST",
    path: "/v1/security-groups",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "createSecurityGroupRule",
    method: "POST",
    path: "/v1/security-groups/{security_group_id}/rules",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_14: Operation = Operation {
    id: "createSubnet",
    method: "POST",
    path: "/v1/subnets",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "createVpc",
    method: "POST",
    path: "/v1/vpcs",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "deleteEgressOnlyGateway",
    method: "DELETE",
    path: "/v1/egress-only-gateways/{egress_only_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_17: Operation = Operation {
    id: "deleteFloatingIp",
    method: "DELETE",
    path: "/v1/floating-ips/{floating_ip_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "deleteInterface",
    method: "DELETE",
    path: "/v1/interfaces/{interface_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_19: Operation = Operation {
    id: "deleteInterfaceAddress",
    method: "DELETE",
    path: "/v1/interfaces/{interface_id}/addresses/{address_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_20: Operation = Operation {
    id: "deleteInterfacePrefix",
    method: "DELETE",
    path: "/v1/interfaces/{interface_id}/prefixes/{prefix_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_21: Operation = Operation {
    id: "deleteInternetGateway",
    method: "DELETE",
    path: "/v1/internet-gateways/{internet_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_22: Operation = Operation {
    id: "deleteNATGateway",
    method: "DELETE",
    path: "/v1/nat-gateways/{nat_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_23: Operation = Operation {
    id: "deletePrefixPool",
    method: "DELETE",
    path: "/v1/vpcs/{vpc_id}/prefix-pools/{pool_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_24: Operation = Operation {
    id: "deleteRoute",
    method: "DELETE",
    path: "/v1/route-tables/{route_table_id}/routes/{route_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_25: Operation = Operation {
    id: "deleteRouteTable",
    method: "DELETE",
    path: "/v1/route-tables/{route_table_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_26: Operation = Operation {
    id: "deleteSecurityGroup",
    method: "DELETE",
    path: "/v1/security-groups/{security_group_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_27: Operation = Operation {
    id: "deleteSecurityGroupRule",
    method: "DELETE",
    path: "/v1/security-groups/{security_group_id}/rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_28: Operation = Operation {
    id: "deleteSubnet",
    method: "DELETE",
    path: "/v1/subnets/{subnet_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_29: Operation = Operation {
    id: "deleteVpc",
    method: "DELETE",
    path: "/v1/vpcs/{vpc_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_30: Operation = Operation {
    id: "detachFloatingIp",
    method: "POST",
    path: "/v1/floating-ips/{floating_ip_id}/detach",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_31: Operation = Operation {
    id: "detachInternetGateway",
    method: "POST",
    path: "/v1/internet-gateways/{internet_gateway_id}/detach",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_32: Operation = Operation {
    id: "getEgressOnlyGateway",
    method: "GET",
    path: "/v1/egress-only-gateways/{egress_only_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_33: Operation = Operation {
    id: "getFloatingIp",
    method: "GET",
    path: "/v1/floating-ips/{floating_ip_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_34: Operation = Operation {
    id: "getInterface",
    method: "GET",
    path: "/v1/interfaces/{interface_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_35: Operation = Operation {
    id: "getInterfaceAddress",
    method: "GET",
    path: "/v1/interfaces/{interface_id}/addresses/{address_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_36: Operation = Operation {
    id: "getInternetGateway",
    method: "GET",
    path: "/v1/internet-gateways/{internet_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_37: Operation = Operation {
    id: "getNATGateway",
    method: "GET",
    path: "/v1/nat-gateways/{nat_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_38: Operation = Operation {
    id: "getRoute",
    method: "GET",
    path: "/v1/route-tables/{route_table_id}/routes/{route_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_39: Operation = Operation {
    id: "getRouteTable",
    method: "GET",
    path: "/v1/route-tables/{route_table_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_40: Operation = Operation {
    id: "getSecurityGroup",
    method: "GET",
    path: "/v1/security-groups/{security_group_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_41: Operation = Operation {
    id: "getSecurityGroupRule",
    method: "GET",
    path: "/v1/security-groups/{security_group_id}/rules/{rule_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_42: Operation = Operation {
    id: "getSubnet",
    method: "GET",
    path: "/v1/subnets/{subnet_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_43: Operation = Operation {
    id: "getVpc",
    method: "GET",
    path: "/v1/vpcs/{vpc_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_44: Operation = Operation {
    id: "listEgressOnlyGatewayRoutes",
    method: "GET",
    path: "/v1/egress-only-gateways/{egress_only_gateway_id}/routes",
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
static OP_45: Operation = Operation {
    id: "listEgressOnlyGateways",
    method: "GET",
    path: "/v1/egress-only-gateways",
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
static OP_46: Operation = Operation {
    id: "listFloatingIps",
    method: "GET",
    path: "/v1/floating-ips",
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
            name: "attached_to",
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
static OP_47: Operation = Operation {
    id: "listInterfaceAddresses",
    method: "GET",
    path: "/v1/interfaces/{interface_id}/addresses",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_48: Operation = Operation {
    id: "listInterfacePrefixes",
    method: "GET",
    path: "/v1/interfaces/{interface_id}/prefixes",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_49: Operation = Operation {
    id: "listInterfaceSecurityGroups",
    method: "GET",
    path: "/v1/interfaces/{interface_id}/security-groups",
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
static OP_50: Operation = Operation {
    id: "listInterfaces",
    method: "GET",
    path: "/v1/interfaces",
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
            name: "subnet",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "vpc",
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
static OP_51: Operation = Operation {
    id: "listInternetGatewayRoutes",
    method: "GET",
    path: "/v1/internet-gateways/{internet_gateway_id}/routes",
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
static OP_52: Operation = Operation {
    id: "listInternetGateways",
    method: "GET",
    path: "/v1/internet-gateways",
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
static OP_53: Operation = Operation {
    id: "listNATGatewayRoutes",
    method: "GET",
    path: "/v1/nat-gateways/{nat_gateway_id}/routes",
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
static OP_54: Operation = Operation {
    id: "listNATGateways",
    method: "GET",
    path: "/v1/nat-gateways",
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
            name: "subnet",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "vpc",
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
static OP_55: Operation = Operation {
    id: "listPrefixPools",
    method: "GET",
    path: "/v1/vpcs/{vpc_id}/prefix-pools",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_56: Operation = Operation {
    id: "listRouteTables",
    method: "GET",
    path: "/v1/route-tables",
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
            name: "vpc",
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
static OP_57: Operation = Operation {
    id: "listRoutes",
    method: "GET",
    path: "/v1/route-tables/{route_table_id}/routes",
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
static OP_58: Operation = Operation {
    id: "listSecurityGroupRules",
    method: "GET",
    path: "/v1/security-groups/{security_group_id}/rules",
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
static OP_59: Operation = Operation {
    id: "listSecurityGroups",
    method: "GET",
    path: "/v1/security-groups",
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
static OP_60: Operation = Operation {
    id: "listSubnets",
    method: "GET",
    path: "/v1/subnets",
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
            name: "vpc",
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
static OP_61: Operation = Operation {
    id: "listVpcs",
    method: "GET",
    path: "/v1/vpcs",
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
static OP_62: Operation = Operation {
    id: "setInterfaceSecurityGroups",
    method: "PUT",
    path: "/v1/interfaces/{interface_id}/security-groups",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_63: Operation = Operation {
    id: "updateEgressOnlyGateway",
    method: "PATCH",
    path: "/v1/egress-only-gateways/{egress_only_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_64: Operation = Operation {
    id: "updateFloatingIp",
    method: "PATCH",
    path: "/v1/floating-ips/{floating_ip_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_65: Operation = Operation {
    id: "updateInterface",
    method: "PATCH",
    path: "/v1/interfaces/{interface_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_66: Operation = Operation {
    id: "updateInternetGateway",
    method: "PATCH",
    path: "/v1/internet-gateways/{internet_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_67: Operation = Operation {
    id: "updateNATGateway",
    method: "PATCH",
    path: "/v1/nat-gateways/{nat_gateway_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_68: Operation = Operation {
    id: "updateRoute",
    method: "PATCH",
    path: "/v1/route-tables/{route_table_id}/routes/{route_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_69: Operation = Operation {
    id: "updateRouteTable",
    method: "PATCH",
    path: "/v1/route-tables/{route_table_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_70: Operation = Operation {
    id: "updateSecurityGroup",
    method: "PATCH",
    path: "/v1/security-groups/{security_group_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_71: Operation = Operation {
    id: "updateSubnet",
    method: "PATCH",
    path: "/v1/subnets/{subnet_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_72: Operation = Operation {
    id: "updateVpc",
    method: "PATCH",
    path: "/v1/vpcs/{vpc_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct NetworkService {
    pub(crate) client: Client,
}
impl NetworkService {
    /// Attach a floating IP to an interface
    pub fn attach_floating_ip(
        &self,
        floating_ip_id: &str,
        body: &m::AttachFloatingIpBody,
    ) -> Request<m::AttachFloatingIpResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_0,
            &[("floating_ip_id", floating_ip_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Attach internet gateway to a VPC
    pub fn attach_internet_gateway(
        &self,
        internet_gateway_id: &str,
        body: &m::AttachInternetGatewayBody,
    ) -> Request<m::AttachInternetGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_1,
            &[("internet_gateway_id", internet_gateway_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create egress-only gateway
    pub fn create_egress_only_gateway(
        &self,
        body: &m::CreateEgressOnlyGatewayBody,
    ) -> Request<m::CreateEgressOnlyGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_2,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Allocate floating IP
    pub fn create_floating_ip(
        &self,
        body: Option<&m::CreateFloatingIpBody>,
    ) -> Request<m::CreateFloatingIpResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_3,
            &[],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create interface
    pub fn create_interface(
        &self,
        body: &m::CreateInterfaceBody,
    ) -> Request<m::CreateInterfaceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_4,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create interface address
    pub fn create_interface_address(
        &self,
        interface_id: &str,
        body: &m::CreateInterfaceAddressBody,
    ) -> Request<m::CreateInterfaceAddressResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_5,
            &[("interface_id", interface_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create interface prefix
    pub fn create_interface_prefix(
        &self,
        interface_id: &str,
        body: &m::CreateInterfacePrefixBody,
    ) -> Request<m::CreateInterfacePrefixResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_6,
            &[("interface_id", interface_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create internet gateway
    pub fn create_internet_gateway(
        &self,
        body: &m::CreateInternetGatewayBody,
    ) -> Request<m::CreateInternetGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_7,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create NAT gateway
    pub fn create_nat_gateway(
        &self,
        body: &m::CreateNATGatewayBody,
    ) -> Request<m::CreateNATGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_8,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create prefix pool
    pub fn create_prefix_pool(
        &self,
        vpc_id: &str,
        body: &m::CreatePrefixPoolBody,
    ) -> Request<m::CreatePrefixPoolResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_9,
            &[("vpc_id", vpc_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create route
    pub fn create_route(
        &self,
        route_table_id: &str,
        body: &m::CreateRouteBody,
    ) -> Request<m::CreateRouteResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_10,
            &[("route_table_id", route_table_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create route table
    pub fn create_route_table(
        &self,
        body: &m::CreateRouteTableBody,
    ) -> Request<m::CreateRouteTableResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_11,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create security group
    pub fn create_security_group(
        &self,
        body: &m::CreateSecurityGroupBody,
    ) -> Request<m::CreateSecurityGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_12,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create security group rule
    pub fn create_security_group_rule(
        &self,
        security_group_id: &str,
        body: &m::CreateSecurityGroupRuleBody,
    ) -> Request<m::CreateSecurityGroupRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_13,
            &[("security_group_id", security_group_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create subnet
    pub fn create_subnet(&self, body: &m::CreateSubnetBody) -> Request<m::CreateSubnetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_14,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Create VPC
    pub fn create_vpc(&self, body: &m::CreateVpcBody) -> Request<m::CreateVpcResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_15,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete egress-only gateway
    pub fn delete_egress_only_gateway(&self, egress_only_gateway_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_16,
            &[("egress_only_gateway_id", egress_only_gateway_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Release floating IP
    pub fn delete_floating_ip(&self, floating_ip_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_17,
            &[("floating_ip_id", floating_ip_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete interface
    pub fn delete_interface(&self, interface_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_18,
            &[("interface_id", interface_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete interface address
    pub fn delete_interface_address(&self, interface_id: &str, address_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_19,
            &[("interface_id", interface_id), ("address_id", address_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete interface prefix
    pub fn delete_interface_prefix(&self, interface_id: &str, prefix_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_20,
            &[("interface_id", interface_id), ("prefix_id", prefix_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete internet gateway
    pub fn delete_internet_gateway(&self, internet_gateway_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_21,
            &[("internet_gateway_id", internet_gateway_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete NAT gateway
    pub fn delete_nat_gateway(&self, nat_gateway_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_22,
            &[("nat_gateway_id", nat_gateway_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete prefix pool
    pub fn delete_prefix_pool(&self, vpc_id: &str, pool_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_23,
            &[("vpc_id", vpc_id), ("pool_id", pool_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete route
    pub fn delete_route(&self, route_table_id: &str, route_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_24,
            &[("route_table_id", route_table_id), ("route_id", route_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete route table
    pub fn delete_route_table(&self, route_table_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_25,
            &[("route_table_id", route_table_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete security group
    pub fn delete_security_group(&self, security_group_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_26,
            &[("security_group_id", security_group_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete security group rule
    pub fn delete_security_group_rule(
        &self,
        security_group_id: &str,
        rule_id: &str,
    ) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_27,
            &[
                ("security_group_id", security_group_id),
                ("rule_id", rule_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete subnet
    pub fn delete_subnet(&self, subnet_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_28,
            &[("subnet_id", subnet_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete VPC
    pub fn delete_vpc(&self, vpc_id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_29,
            &[("vpc_id", vpc_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach a floating IP
    pub fn detach_floating_ip(
        &self,
        floating_ip_id: &str,
        body: Option<&m::DetachFloatingIpBody>,
    ) -> Request<m::DetachFloatingIpResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_30,
            &[("floating_ip_id", floating_ip_id)],
            body.map(Payload::json).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Detach internet gateway from its VPC
    pub fn detach_internet_gateway(
        &self,
        internet_gateway_id: &str,
    ) -> Request<m::DetachInternetGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_31,
            &[("internet_gateway_id", internet_gateway_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get egress-only gateway
    pub fn get_egress_only_gateway(
        &self,
        egress_only_gateway_id: &str,
    ) -> Request<m::GetEgressOnlyGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_32,
            &[("egress_only_gateway_id", egress_only_gateway_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_egress_only_gateway_by_reference(
        &self,
        reference: &str,
        scope: &m::GetEgressOnlyGatewayScope,
    ) -> Request<m::GetEgressOnlyGatewayResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_32,
                &[("egress_only_gateway_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_45,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("egress_only_gateway"),
            "egress_only_gateways",
        );
        Request::reference(core, extract)
    }
    /// Get floating IP
    pub fn get_floating_ip(&self, floating_ip_id: &str) -> Request<m::GetFloatingIpResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_33,
            &[("floating_ip_id", floating_ip_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_floating_ip_by_reference(
        &self,
        reference: &str,
        scope: &m::GetFloatingIpScope,
    ) -> Request<m::GetFloatingIpResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_33,
                &[("floating_ip_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_46,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("floating_ip"),
            "floating_ips",
        );
        Request::reference(core, extract)
    }
    /// Get interface
    pub fn get_interface(&self, interface_id: &str) -> Request<m::GetInterfaceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_34,
            &[("interface_id", interface_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_interface_by_reference(
        &self,
        reference: &str,
        scope: &m::GetInterfaceScope,
    ) -> Request<m::GetInterfaceResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_34,
                &[("interface_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_50,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("interface"),
            "interfaces",
        );
        Request::reference(core, extract)
    }
    /// Get interface address
    pub fn get_interface_address(
        &self,
        interface_id: &str,
        address_id: &str,
    ) -> Request<m::GetInterfaceAddressResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_35,
            &[("interface_id", interface_id), ("address_id", address_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get internet gateway
    pub fn get_internet_gateway(
        &self,
        internet_gateway_id: &str,
    ) -> Request<m::GetInternetGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_36,
            &[("internet_gateway_id", internet_gateway_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_internet_gateway_by_reference(
        &self,
        reference: &str,
        scope: &m::GetInternetGatewayScope,
    ) -> Request<m::GetInternetGatewayResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_36,
                &[("internet_gateway_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_52,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("internet_gateway"),
            "internet_gateways",
        );
        Request::reference(core, extract)
    }
    /// Get NAT gateway
    pub fn get_nat_gateway(&self, nat_gateway_id: &str) -> Request<m::GetNATGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_37,
            &[("nat_gateway_id", nat_gateway_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_nat_gateway_by_reference(
        &self,
        reference: &str,
        scope: &m::GetNATGatewayScope,
    ) -> Request<m::GetNATGatewayResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_37,
                &[("nat_gateway_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_54,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("nat_gateway"),
            "nat_gateways",
        );
        Request::reference(core, extract)
    }
    /// Get route
    pub fn get_route(&self, route_table_id: &str, route_id: &str) -> Request<m::GetRouteResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_38,
            &[("route_table_id", route_table_id), ("route_id", route_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_route_by_reference(
        &self,
        route_table_id: &str,
        reference: &str,
        scope: &m::GetRouteScope,
    ) -> Request<m::GetRouteResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_38,
                &[("route_table_id", route_table_id), ("route_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_57,
                &[("route_table_id", route_table_id)],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("route"),
            "routes",
        );
        Request::reference(core, extract)
    }
    /// Get route table
    pub fn get_route_table(&self, route_table_id: &str) -> Request<m::GetRouteTableResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_39,
            &[("route_table_id", route_table_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_route_table_by_reference(
        &self,
        reference: &str,
        scope: &m::GetRouteTableScope,
    ) -> Request<m::GetRouteTableResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_39,
                &[("route_table_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_56,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("route_table"),
            "route_tables",
        );
        Request::reference(core, extract)
    }
    /// Get security group
    pub fn get_security_group(
        &self,
        security_group_id: &str,
    ) -> Request<m::GetSecurityGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_40,
            &[("security_group_id", security_group_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_security_group_by_reference(
        &self,
        reference: &str,
        scope: &m::GetSecurityGroupScope,
    ) -> Request<m::GetSecurityGroupResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_40,
                &[("security_group_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_59,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("security_group"),
            "security_groups",
        );
        Request::reference(core, extract)
    }
    /// Get security group rule
    pub fn get_security_group_rule(
        &self,
        security_group_id: &str,
        rule_id: &str,
    ) -> Request<m::GetSecurityGroupRuleResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_41,
            &[
                ("security_group_id", security_group_id),
                ("rule_id", rule_id),
            ],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_security_group_rule_by_reference(
        &self,
        security_group_id: &str,
        reference: &str,
        scope: &m::GetSecurityGroupRuleScope,
    ) -> Request<m::GetSecurityGroupRuleResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_41,
                &[
                    ("security_group_id", security_group_id),
                    ("rule_id", reference),
                ],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_58,
                &[("security_group_id", security_group_id)],
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
    /// Get subnet
    pub fn get_subnet(&self, subnet_id: &str) -> Request<m::GetSubnetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_42,
            &[("subnet_id", subnet_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_subnet_by_reference(
        &self,
        reference: &str,
        scope: &m::GetSubnetScope,
    ) -> Request<m::GetSubnetResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_42,
                &[("subnet_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_60,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("subnet"),
            "subnets",
        );
        Request::reference(core, extract)
    }
    /// Get VPC
    pub fn get_vpc(&self, vpc_id: &str) -> Request<m::GetVpcResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_43,
            &[("vpc_id", vpc_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_vpc_by_reference(
        &self,
        reference: &str,
        scope: &m::GetVpcScope,
    ) -> Request<m::GetVpcResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_43,
                &[("vpc_id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_61,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("vpc"),
            "vpcs",
        );
        Request::reference(core, extract)
    }
    /// List egress-only gateway routes
    pub fn list_egress_only_gateway_routes(
        &self,
        egress_only_gateway_id: &str,
        query: &m::ListEgressOnlyGatewayRoutesQuery,
    ) -> PagedRequest<m::ListEgressOnlyGatewayRoutesResponse, m::ListEgressOnlyGatewayRoutesItem>
    {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_44,
                &[("egress_only_gateway_id", egress_only_gateway_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "routes",
        )
    }
    /// List egress-only gateways
    pub fn list_egress_only_gateways(
        &self,
        query: &m::ListEgressOnlyGatewaysQuery,
    ) -> PagedRequest<m::ListEgressOnlyGatewaysResponse, m::ListEgressOnlyGatewaysItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_45,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "egress_only_gateways",
        )
    }
    /// List floating IPs
    pub fn list_floating_ips(
        &self,
        query: &m::ListFloatingIpsQuery,
    ) -> PagedRequest<m::ListFloatingIpsResponse, m::ListFloatingIpsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_46,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "floating_ips",
        )
    }
    /// List interface addresses
    pub fn list_interface_addresses(
        &self,
        interface_id: &str,
    ) -> PagedRequest<m::ListInterfaceAddressesResponse, m::ListInterfaceAddressesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_47,
                &[("interface_id", interface_id)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "addresses",
        )
    }
    /// List interface prefixes
    pub fn list_interface_prefixes(
        &self,
        interface_id: &str,
    ) -> PagedRequest<m::ListInterfacePrefixesResponse, m::ListInterfacePrefixesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_48,
                &[("interface_id", interface_id)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "routed_prefixes",
        )
    }
    /// List interface security-group membership
    pub fn list_interface_security_groups(
        &self,
        interface_id: &str,
        query: &m::ListInterfaceSecurityGroupsQuery,
    ) -> PagedRequest<m::ListInterfaceSecurityGroupsResponse, m::ListInterfaceSecurityGroupsItem>
    {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_49,
                &[("interface_id", interface_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "security_group_ids",
        )
    }
    /// List interfaces
    pub fn list_interfaces(
        &self,
        query: &m::ListInterfacesQuery,
    ) -> PagedRequest<m::ListInterfacesResponse, m::ListInterfacesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_50,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "interfaces",
        )
    }
    /// List internet gateway routes
    pub fn list_internet_gateway_routes(
        &self,
        internet_gateway_id: &str,
        query: &m::ListInternetGatewayRoutesQuery,
    ) -> PagedRequest<m::ListInternetGatewayRoutesResponse, m::ListInternetGatewayRoutesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_51,
                &[("internet_gateway_id", internet_gateway_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "routes",
        )
    }
    /// List internet gateways
    pub fn list_internet_gateways(
        &self,
        query: &m::ListInternetGatewaysQuery,
    ) -> PagedRequest<m::ListInternetGatewaysResponse, m::ListInternetGatewaysItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_52,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "internet_gateways",
        )
    }
    /// List NAT gateway routes
    pub fn list_nat_gateway_routes(
        &self,
        nat_gateway_id: &str,
        query: &m::ListNATGatewayRoutesQuery,
    ) -> PagedRequest<m::ListNATGatewayRoutesResponse, m::ListNATGatewayRoutesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_53,
                &[("nat_gateway_id", nat_gateway_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "routes",
        )
    }
    /// List NAT gateways
    pub fn list_nat_gateways(
        &self,
        query: &m::ListNATGatewaysQuery,
    ) -> PagedRequest<m::ListNATGatewaysResponse, m::ListNATGatewaysItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_54,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "nat_gateways",
        )
    }
    /// List prefix pools
    pub fn list_prefix_pools(
        &self,
        vpc_id: &str,
    ) -> PagedRequest<m::ListPrefixPoolsResponse, m::ListPrefixPoolsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_55,
                &[("vpc_id", vpc_id)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "prefix_pools",
        )
    }
    /// List route tables
    pub fn list_route_tables(
        &self,
        query: &m::ListRouteTablesQuery,
    ) -> PagedRequest<m::ListRouteTablesResponse, m::ListRouteTablesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_56,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "route_tables",
        )
    }
    /// List routes
    pub fn list_routes(
        &self,
        route_table_id: &str,
        query: &m::ListRoutesQuery,
    ) -> PagedRequest<m::ListRoutesResponse, m::ListRoutesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_57,
                &[("route_table_id", route_table_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "routes",
        )
    }
    /// List security group rules
    pub fn list_security_group_rules(
        &self,
        security_group_id: &str,
        query: &m::ListSecurityGroupRulesQuery,
    ) -> PagedRequest<m::ListSecurityGroupRulesResponse, m::ListSecurityGroupRulesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_58,
                &[("security_group_id", security_group_id)],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "rules",
        )
    }
    /// List security groups
    pub fn list_security_groups(
        &self,
        query: &m::ListSecurityGroupsQuery,
    ) -> PagedRequest<m::ListSecurityGroupsResponse, m::ListSecurityGroupsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_59,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "security_groups",
        )
    }
    /// List subnets
    pub fn list_subnets(
        &self,
        query: &m::ListSubnetsQuery,
    ) -> PagedRequest<m::ListSubnetsResponse, m::ListSubnetsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_60,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "subnets",
        )
    }
    /// List VPCs
    pub fn list_vpcs(
        &self,
        query: &m::ListVpcsQuery,
    ) -> PagedRequest<m::ListVpcsResponse, m::ListVpcsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "network",
                ENDPOINT,
                &OP_61,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "vpcs",
        )
    }
    /// Set interface security-group membership
    pub fn set_interface_security_groups(
        &self,
        interface_id: &str,
        body: &m::SetInterfaceSecurityGroupsBody,
    ) -> Request<m::SetInterfaceSecurityGroupsResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_62,
            &[("interface_id", interface_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update egress-only gateway
    pub fn update_egress_only_gateway(
        &self,
        egress_only_gateway_id: &str,
        body: &m::UpdateEgressOnlyGatewayBody,
    ) -> Request<m::UpdateEgressOnlyGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_63,
            &[("egress_only_gateway_id", egress_only_gateway_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update floating IP
    pub fn update_floating_ip(
        &self,
        floating_ip_id: &str,
        body: &m::UpdateFloatingIpBody,
    ) -> Request<m::UpdateFloatingIpResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_64,
            &[("floating_ip_id", floating_ip_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update interface
    pub fn update_interface(
        &self,
        interface_id: &str,
        body: &m::UpdateInterfaceBody,
    ) -> Request<m::UpdateInterfaceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_65,
            &[("interface_id", interface_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update internet gateway
    pub fn update_internet_gateway(
        &self,
        internet_gateway_id: &str,
        body: &m::UpdateInternetGatewayBody,
    ) -> Request<m::UpdateInternetGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_66,
            &[("internet_gateway_id", internet_gateway_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update NAT gateway
    pub fn update_nat_gateway(
        &self,
        nat_gateway_id: &str,
        body: &m::UpdateNATGatewayBody,
    ) -> Request<m::UpdateNATGatewayResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_67,
            &[("nat_gateway_id", nat_gateway_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update route
    pub fn update_route(
        &self,
        route_table_id: &str,
        route_id: &str,
        body: &m::UpdateRouteBody,
    ) -> Request<m::UpdateRouteResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_68,
            &[("route_table_id", route_table_id), ("route_id", route_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update route table
    pub fn update_route_table(
        &self,
        route_table_id: &str,
        body: &m::UpdateRouteTableBody,
    ) -> Request<m::UpdateRouteTableResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_69,
            &[("route_table_id", route_table_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update security group
    pub fn update_security_group(
        &self,
        security_group_id: &str,
        body: &m::UpdateSecurityGroupBody,
    ) -> Request<m::UpdateSecurityGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_70,
            &[("security_group_id", security_group_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update subnet
    pub fn update_subnet(
        &self,
        subnet_id: &str,
        body: &m::UpdateSubnetBody,
    ) -> Request<m::UpdateSubnetResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_71,
            &[("subnet_id", subnet_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update VPC
    pub fn update_vpc(
        &self,
        vpc_id: &str,
        body: &m::UpdateVpcBody,
    ) -> Request<m::UpdateVpcResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "network",
            ENDPOINT,
            &OP_72,
            &[("vpc_id", vpc_id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
}
