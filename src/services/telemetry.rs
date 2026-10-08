//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::telemetry as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://telemetry.{region}.basaltic.sh";
static OP_0: Operation = Operation {
    id: "createLogGroup",
    method: "POST",
    path: "/v1/log-groups",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "deleteLogGroup",
    method: "DELETE",
    path: "/v1/log-groups/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "deleteTraceSettings",
    method: "DELETE",
    path: "/v1/trace-settings",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_3: Operation = Operation {
    id: "getLog",
    method: "GET",
    path: "/v1/logs/{log_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "getLogGroup",
    method: "GET",
    path: "/v1/log-groups/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_5: Operation = Operation {
    id: "getRetainedTelemetryPresence",
    method: "GET",
    path: "/v1/trace-settings/retained-data",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_6: Operation = Operation {
    id: "getTrace",
    method: "GET",
    path: "/v1/traces/{trace_id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_7: Operation = Operation {
    id: "getTraceSettings",
    method: "GET",
    path: "/v1/trace-settings",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_8: Operation = Operation {
    id: "ingestLogs",
    method: "POST",
    path: "/v1/logs",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_9: Operation = Operation {
    id: "ingestSpans",
    method: "POST",
    path: "/v1/spans",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_10: Operation = Operation {
    id: "listLogGroups",
    method: "GET",
    path: "/v1/log-groups",
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
static OP_11: Operation = Operation {
    id: "listMetricNames",
    method: "GET",
    path: "/v1/metrics/names",
    authenticated: true,
    required_query: &["start", "end"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "start",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "end",
            style: "form",
            explode: true,
        },
    ],
};
static OP_12: Operation = Operation {
    id: "listMetricNamesPost",
    method: "POST",
    path: "/v1/metrics/names",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/x-www-form-urlencoded",
    accept: "application/json",
    query_encoding: &[],
};
static OP_13: Operation = Operation {
    id: "listMetricSeries",
    method: "GET",
    path: "/v1/metrics/series",
    authenticated: true,
    required_query: &["metric", "start", "end"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "metric",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "start",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "end",
            style: "form",
            explode: true,
        },
    ],
};
static OP_14: Operation = Operation {
    id: "listMetricSeriesPost",
    method: "POST",
    path: "/v1/metrics/series",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/x-www-form-urlencoded",
    accept: "application/json",
    query_encoding: &[],
};
static OP_15: Operation = Operation {
    id: "putTraceSettings",
    method: "PUT",
    path: "/v1/trace-settings",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_16: Operation = Operation {
    id: "queryMetricsInstant",
    method: "GET",
    path: "/v1/metrics/query",
    authenticated: true,
    required_query: &["metric", "agg"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "metric",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "agg",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "match[]",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "by[]",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "step",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "time",
            style: "form",
            explode: true,
        },
    ],
};
static OP_17: Operation = Operation {
    id: "queryMetricsInstantPost",
    method: "POST",
    path: "/v1/metrics/query",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/x-www-form-urlencoded",
    accept: "application/json",
    query_encoding: &[],
};
static OP_18: Operation = Operation {
    id: "queryMetricsRange",
    method: "GET",
    path: "/v1/metrics/query_range",
    authenticated: true,
    required_query: &["metric", "agg", "start", "end"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "metric",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "agg",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "match[]",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "by[]",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "start",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "end",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "step",
            style: "form",
            explode: true,
        },
    ],
};
static OP_19: Operation = Operation {
    id: "queryMetricsRangePost",
    method: "POST",
    path: "/v1/metrics/query_range",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/x-www-form-urlencoded",
    accept: "application/json",
    query_encoding: &[],
};
static OP_20: Operation = Operation {
    id: "searchLogs",
    method: "GET",
    path: "/v1/logs",
    authenticated: true,
    required_query: &["from", "to"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "from",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "to",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "log_group",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "log_stream",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "min_severity",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "region",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "q",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "trace_id",
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
static OP_21: Operation = Operation {
    id: "searchTraces",
    method: "GET",
    path: "/v1/traces",
    authenticated: true,
    required_query: &["from", "to"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "from",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "to",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "service",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "operation",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "status_code",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "min_duration_ms",
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
    id: "updateLogGroup",
    method: "PATCH",
    path: "/v1/log-groups/{id}",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/json",
    accept: "application/json",
    query_encoding: &[],
};
static OP_23: Operation = Operation {
    id: "writeMetrics",
    method: "POST",
    path: "/v1/metrics/write",
    authenticated: true,
    required_query: &[],
    required_headers: &[],
    content_type: "application/x-protobuf",
    accept: "application/json",
    query_encoding: &[],
};
#[derive(Clone, Debug)]
pub struct TelemetryService {
    pub(crate) client: Client,
}
impl TelemetryService {
    /// Create a log group
    pub fn create_log_group(
        &self,
        body: &m::CreateLogGroupBody,
    ) -> Request<m::CreateLogGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_0,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete a log group
    pub fn delete_log_group(&self, id: &str) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_1,
            &[("id", id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Delete trace settings
    pub fn delete_trace_settings(&self) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_2,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get a single log record by id
    pub fn get_log(&self, log_id: &str) -> Request<m::GetLogResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_3,
            &[("log_id", log_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get a log group by id
    pub fn get_log_group(&self, id: &str) -> Request<m::GetLogGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_4,
            &[("id", id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_log_group_by_reference(
        &self,
        reference: &str,
        scope: &m::GetLogGroupScope,
    ) -> Request<m::GetLogGroupResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "telemetry",
                ENDPOINT,
                &OP_4,
                &[("id", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "telemetry",
                ENDPOINT,
                &OP_10,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            Some("log_group"),
            "log_groups",
        );
        Request::reference(core, extract)
    }
    /// Check retained telemetry presence
    pub fn get_retained_telemetry_presence(
        &self,
    ) -> Request<m::GetRetainedTelemetryPresenceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_5,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get all spans for a trace
    pub fn get_trace(&self, trace_id: &str) -> Request<m::GetTraceResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_6,
            &[("trace_id", trace_id)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get the caller account's trace settings
    pub fn get_trace_settings(&self) -> Request<m::GetTraceSettingsResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_7,
            &[],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Ingest a batch of log records
    pub fn ingest_logs(&self, body: &m::IngestLogsBody) -> Request<m::IngestLogsResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_8,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Ingest a batch of trace spans
    pub fn ingest_spans(&self, body: &m::IngestSpansBody) -> Request<m::IngestSpansResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_9,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// List log groups (or look up one by name)
    pub fn list_log_groups(
        &self,
        query: &m::ListLogGroupsQuery,
    ) -> PagedRequest<m::ListLogGroupsResponse, m::ListLogGroupsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "telemetry",
                ENDPOINT,
                &OP_10,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "log_groups",
        )
    }
    /// List the distinct metric names emitted in a time window
    pub fn list_metric_names(
        &self,
        query: &m::ListMetricNamesQuery,
    ) -> PagedRequest<m::ListMetricNamesResponse, m::ListMetricNamesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "telemetry",
                ENDPOINT,
                &OP_11,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "data",
        )
    }
    /// List the distinct metric names emitted in a time window (form body)
    pub fn list_metric_names_post(
        &self,
        body: Option<&m::ListMetricNamesPostBody>,
    ) -> Request<m::ListMetricNamesPostResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_12,
            &[],
            body.map(Payload::form).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// List distinct label sets for a metric
    pub fn list_metric_series(
        &self,
        query: &m::ListMetricSeriesQuery,
    ) -> Request<m::ListMetricSeriesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_13,
            &[],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// List distinct label sets for a metric (form body)
    pub fn list_metric_series_post(
        &self,
        body: Option<&m::ListMetricSeriesPostBody>,
    ) -> Request<m::ListMetricSeriesPostResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_14,
            &[],
            body.map(Payload::form).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Update the caller account's trace settings
    pub fn put_trace_settings(
        &self,
        body: &m::PutTraceSettingsBody,
    ) -> Request<m::PutTraceSettingsResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_15,
            &[],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Instant structured metric query
    pub fn query_metrics_instant(
        &self,
        query: &m::QueryMetricsInstantQuery,
    ) -> Request<m::QueryMetricsInstantResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_16,
            &[],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Instant structured metric query (form body)
    pub fn query_metrics_instant_post(
        &self,
        body: Option<&m::QueryMetricsInstantPostBody>,
    ) -> Request<m::QueryMetricsInstantPostResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_17,
            &[],
            body.map(Payload::form).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Range structured metric query
    pub fn query_metrics_range(
        &self,
        query: &m::QueryMetricsRangeQuery,
    ) -> Request<m::QueryMetricsRangeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_18,
            &[],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Range structured metric query (form body)
    pub fn query_metrics_range_post(
        &self,
        body: Option<&m::QueryMetricsRangePostBody>,
    ) -> Request<m::QueryMetricsRangePostResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_19,
            &[],
            body.map(Payload::form).unwrap_or(Ok(Payload::Empty)),
            Ok(serde_json::json!({})),
        ))
    }
    /// Search log records
    pub fn search_logs(&self, query: &m::SearchLogsQuery) -> Request<m::SearchLogsResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_20,
            &[],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// List traces
    pub fn search_traces(&self, query: &m::SearchTracesQuery) -> Request<m::SearchTracesResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_21,
            &[],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
    /// Update a log group
    pub fn update_log_group(
        &self,
        id: &str,
        body: &m::UpdateLogGroupBody,
    ) -> Request<m::UpdateLogGroupResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_22,
            &[("id", id)],
            Payload::json(body),
            Ok(serde_json::json!({})),
        ))
    }
    /// Prometheus remote_write ingest
    pub fn write_metrics(&self, body: reqwest::Body) -> EmptyRequest {
        EmptyRequest::new(Core::new(
            self.client.clone(),
            "telemetry",
            ENDPOINT,
            &OP_23,
            &[],
            Ok(Payload::binary(body)),
            Ok(serde_json::json!({})),
        ))
    }
}
