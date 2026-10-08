use basaltic::{
    Client, Config, Credentials, Error, RequestOptions,
    models::compute::{ImageStatus, ListImagesQuery},
};
use futures_util::TryStreamExt;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path, query_param},
};

fn image(id: &str) -> Value {
    json!({"id":id,"crn":"crn:compute:test-1:account:image/one","name":"linux","architecture":"x86_64","version":"1","status":"active","faults":[],"created_at":"2026-01-01T00:00:00Z","updated_at":"2026-01-01T00:00:00Z"})
}
fn config(server: &MockServer) -> Config {
    Config {
        credentials: Credentials::bearer("fixture-token").unwrap(),
        region: "test-1".into(),
        account_id: "account".into(),
        endpoints: ["compute", "iam", "storage", "network"]
            .map(|s| (s.to_string(), server.uri()))
            .into(),
        base_delay: Duration::ZERO,
        max_attempts: 3,
        ..Config::default()
    }
}
fn client(server: &MockServer) -> Client {
    Client::new(config(server)).unwrap()
}

#[tokio::test]
async fn typed_envelope_and_scoped_headers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/images/x"))
        .and(header("authorization", "Bearer fixture-token"))
        .and(header("x-account-id", "other"))
        .and(header("x-custom", "value"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("x-request-id", "request-1")
                .set_body_json(json!({"image":image("x")})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let options = RequestOptions {
        account_id: Some("other".into()),
        headers: [("x-custom".parse().unwrap(), "value".parse().unwrap())]
            .into_iter()
            .collect(),
        ..Default::default()
    };
    let result = client(&server)
        .compute()
        .get_image("x")
        .options(options)
        .send()
        .await
        .unwrap();
    assert_eq!(result.data.image.id, "x");
    assert_eq!(result.data.image.status, ImageStatus::Active);
    assert_eq!(result.request_id(), "request-1");
    assert_eq!(result.status, 200);
}

#[tokio::test]
async fn path_encoding_and_endpoint_prefix_do_not_change_routing() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"image":image("x")})))
        .mount(&server)
        .await;
    let mut cfg = config(&server);
    cfg.endpoints
        .insert("compute".into(), server.uri() + "/api/");
    Client::new(cfg)
        .unwrap()
        .compute()
        .get_image("a/b ?%ü")
        .send()
        .await
        .unwrap();
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests[0].url.path(),
        "/api/v1/images/a%2Fb%20%3F%25%C3%BC"
    );
    assert!(requests[0].headers.get("cookie").is_none());
}

#[tokio::test]
async fn false_zero_and_empty_query_values_survive() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(query_param("limit", "0"))
        .and(query_param("all_versions", "false"))
        .and(query_param("name", ""))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"images":[]})))
        .expect(1)
        .mount(&server)
        .await;
    client(&server)
        .compute()
        .list_images(&ListImagesQuery {
            limit: Some(0),
            all_versions: Some(false),
            name: Some("".into()),
            ..Default::default()
        })
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn only_safe_or_idempotent_mutations_are_retried() {
    for (mutation, key, expected) in [
        (false, None, 3),
        (true, None, 1),
        (true, Some("same-key"), 3),
    ] {
        let server = MockServer::start().await;
        Mock::given(method(if mutation { "POST" } else { "GET" }))
            .respond_with(
                ResponseTemplate::new(503)
                    .set_body_json(json!({"error":{"code":"BUSY","message":"Try later"}})),
            )
            .expect(expected)
            .mount(&server)
            .await;
        let c = client(&server);
        let error = if mutation {
            c.compute()
                .start_instance("x")
                .options(RequestOptions {
                    idempotency_key: key.map(str::to_owned),
                    ..Default::default()
                })
                .send()
                .await
                .err()
                .unwrap()
        } else {
            c.compute().get_image("x").send().await.err().unwrap()
        };
        assert!(matches!(error,Error::Api(e) if e.status==503 && e.code=="BUSY"));
    }
}

#[tokio::test]
async fn streaming_upload_is_never_replayed() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let stream = futures_util::stream::iter([Ok::<_, std::io::Error>(bytes::Bytes::from_static(
        b"payload",
    ))]);
    let error = client(&server)
        .storage()
        .put_object("bucket", "key", basaltic::Body::wrap_stream(stream))
        .options(RequestOptions {
            idempotency_key: Some("stable".into()),
            ..Default::default()
        })
        .send()
        .await
        .err()
        .unwrap();
    assert!(matches!(error, Error::Api(_)));
}

#[tokio::test]
async fn excessive_retry_after_is_not_shortened() {
    for retry in ["30", "9999999999999999999999999999999999999999"] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(429).insert_header("retry-after", retry))
            .expect(1)
            .mount(&server)
            .await;
        assert!(
            client(&server)
                .compute()
                .get_image("x")
                .send()
                .await
                .is_err()
        );
    }
}

#[tokio::test]
async fn redirects_never_receive_credentials() {
    let server = MockServer::start().await;
    let target = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(302).insert_header("location", target.uri()))
        .expect(1)
        .mount(&server)
        .await;
    assert!(
        matches!(client(&server).compute().get_image("x").send().await,Err(Error::Api(e)) if e.status==302)
    );
    assert!(target.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn oauth_refresh_is_single_flight_and_cached() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/oauth/token"))
        .and(header("authorization", "Basic a2V5OnNlY3JldA=="))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_millis(30))
                .set_body_json(
                    json!({"access_token":"issued","token_type":"Bearer","expires_in":900}),
                ),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(header("authorization", "Bearer issued"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"image":image("x")})))
        .expect(16)
        .mount(&server)
        .await;
    let mut cfg = config(&server);
    cfg.credentials = Credentials::access_key("key", "secret").unwrap();
    let c = Client::new(cfg).unwrap();
    let tasks = (0..16).map(|_| {
        let c = c.clone();
        async move {
            c.compute().get_image("x").send().await.unwrap();
        }
    });
    futures_util::future::join_all(tasks).await;
}

#[tokio::test]
async fn rejected_credentials_refresh_once() {
    let server = MockServer::start().await;
    let tokens = Arc::new(AtomicUsize::new(0));
    let count = tokens.clone();
    Mock::given(method("POST"))
        .and(path("/v1/oauth/token"))
        .respond_with(move |_: &wiremock::Request| {
            let n = count.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(200)
                .set_body_json(json!({"access_token":format!("token-{n}"),"expires_in":900}))
        })
        .expect(2)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(header("authorization", "Bearer token-0"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(header("authorization", "Bearer token-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"image":image("x")})))
        .expect(1)
        .mount(&server)
        .await;
    let mut cfg = config(&server);
    cfg.credentials = Credentials::access_key("key", "secret").unwrap();
    Client::new(cfg)
        .unwrap()
        .compute()
        .get_image("x")
        .send()
        .await
        .unwrap();
    assert_eq!(tokens.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn cancelling_refresh_does_not_poison_credentials() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/oauth/token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(Duration::from_millis(100))
                .set_body_json(json!({"access_token":"issued"})),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"image":image("x")})))
        .mount(&server)
        .await;
    let mut cfg = config(&server);
    cfg.credentials = Credentials::access_key("key", "secret").unwrap();
    let c = Client::new(cfg).unwrap();
    let cancelled =
        tokio::time::timeout(Duration::from_millis(15), c.compute().get_image("x").send()).await;
    assert!(cancelled.is_err());
    tokio::time::timeout(Duration::from_secs(2), c.compute().get_image("x").send())
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn authentication_errors_and_debug_omit_secrets() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(401).set_body_json(json!({"error":"key secret echoed"})),
        )
        .mount(&server)
        .await;
    let mut cfg = config(&server);
    cfg.credentials = Credentials::access_key("private-key", "private-secret").unwrap();
    assert!(!format!("{cfg:?}").contains("private-secret"));
    let err = Client::new(cfg)
        .unwrap()
        .compute()
        .get_image("x")
        .send()
        .await
        .err()
        .unwrap();
    assert!(matches!(err, Error::Authentication { status: Some(401) }));
    assert!(!format!("{err:?} {err}").contains("echoed"));
}

#[tokio::test]
async fn lazy_pagination_uses_markers_and_preserves_scope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(query_param("name", "linux"))
        .respond_with(|request: &wiremock::Request| {
            let next = request
                .url
                .query_pairs()
                .any(|(k, v)| k == "marker" && v == "next");
            ResponseTemplate::new(200).set_body_json(if next {
                json!({"images":[image("two")],"meta":{"has_more":false}})
            } else {
                json!({"images":[image("one")],"meta":{"has_more":true,"marker":"next"}})
            })
        })
        .expect(2)
        .mount(&server)
        .await;
    let stream = client(&server)
        .compute()
        .list_images(&ListImagesQuery {
            name: Some("linux".into()),
            ..Default::default()
        })
        .items();
    assert!(server.received_requests().await.unwrap().is_empty());
    let ids = stream
        .map_ok(|i| i.id)
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(ids, ["one", "two"]);
}

#[tokio::test]
async fn pagination_missing_or_repeated_markers_fail() {
    for meta in [
        json!({"has_more":true}),
        json!({"has_more":true,"marker":"same"}),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!({"images":[],"meta":meta})),
            )
            .mount(&server)
            .await;
        let result = client(&server)
            .compute()
            .list_images(&Default::default())
            .items()
            .try_collect::<Vec<_>>()
            .await;
        assert!(matches!(result, Err(Error::Protocol(_))));
        assert!(server.received_requests().await.unwrap().len() <= 2);
    }
}

#[tokio::test]
async fn references_support_names_uuids_crns_and_ambiguity() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/images"))
        .respond_with(|r: &wiremock::Request| {
            let name = r
                .url
                .query_pairs()
                .find(|(k, _)| k == "name")
                .map(|(_, v)| v.into_owned())
                .unwrap_or_default();
            let items = match name.as_str() {
                "absent" => vec![],
                "ambiguous" => vec![image("one"), image("two")],
                _ => vec![image("one")],
            };
            ResponseTemplate::new(200).set_body_json(json!({"images":items}))
        })
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/images/00000000-0000-0000-0000-000000000001"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"image":image("uuid")})))
        .mount(&server)
        .await;
    let c = client(&server);
    for name in ["linux", "crn:compute:region:account:image/one"] {
        assert_eq!(
            c.compute()
                .get_image_by_reference(name, &Default::default())
                .send()
                .await
                .unwrap()
                .data
                .id,
            "one"
        );
    }
    assert_eq!(
        c.compute()
            .get_image_by_reference("00000000-0000-0000-0000-000000000001", &Default::default())
            .send()
            .await
            .unwrap()
            .data
            .id,
        "uuid"
    );
    assert!(
        matches!(c.compute().get_image_by_reference("absent",&Default::default()).send().await,Err(Error::Api(e)) if e.status==404)
    );
    assert!(matches!(
        c.compute()
            .get_image_by_reference("ambiguous", &Default::default())
            .send()
            .await,
        Err(Error::AmbiguousReference)
    ));
    assert!(matches!(
        c.compute()
            .get_image_by_reference("crn:invalid", &Default::default())
            .send()
            .await,
        Err(Error::Configuration(_))
    ));
}

#[tokio::test]
async fn binary_and_head_responses_remain_streams() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(206)
                .set_body_bytes(b"binary\x00payload")
                .insert_header("content-type", "application/octet-stream"),
        )
        .mount(&server)
        .await;
    Mock::given(method("HEAD"))
        .respond_with(ResponseTemplate::new(200).insert_header("etag", "object-etag"))
        .mount(&server)
        .await;
    let c = client(&server);
    let response = c
        .storage()
        .get_object("bucket", "key")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 206);
    assert_eq!(response.bytes().await.unwrap(), b"binary\x00payload"[..]);
    assert_eq!(
        c.storage()
            .head_object("bucket", "key")
            .send()
            .await
            .unwrap()
            .headers()["etag"],
        "object-etag"
    );
}

#[tokio::test]
async fn websocket_descriptor_does_not_connect() {
    let server = MockServer::start().await;
    let result = client(&server)
        .compute()
        .start_serial_console("x", &Default::default())
        .prepare()
        .await
        .unwrap();
    assert!(result.url.starts_with("ws://"));
    assert_eq!(result.headers["authorization"], "Bearer fixture-token");
    assert!(!format!("{result:?}").contains("fixture-token"));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn rejects_managed_headers_dot_paths_and_invalid_configuration() {
    assert!(Client::new(Config::default()).is_err());
    for url in [
        "https://user:pass@example.test",
        "https://example.test?x=1",
        "https://example.test#fragment",
        "https://example.test:0",
        "not a url",
    ] {
        assert!(
            Client::new(Config {
                credentials: Credentials::Anonymous,
                endpoints: BTreeMap::from([("compute".into(), url.into())]),
                ..Default::default()
            })
            .is_err()
        );
    }
    for token in ["", "bad\ntoken", "bad token", "badü"] {
        assert!(Credentials::bearer(token).is_err());
    }
    let server = MockServer::start().await;
    let c = client(&server);
    for value in ["", ".", ".."] {
        assert!(matches!(
            c.compute().get_image(value).send().await,
            Err(Error::Configuration(_))
        ));
    }
    let options = RequestOptions {
        headers: [(
            "authorization".parse().unwrap(),
            "Bearer override".parse().unwrap(),
        )]
        .into_iter()
        .collect(),
        ..Default::default()
    };
    assert!(matches!(
        c.compute().get_image("x").options(options).send().await,
        Err(Error::Configuration(_))
    ));
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn timeout_is_distinct_and_not_retried_for_mutations() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(204).set_delay(Duration::from_secs(1)))
        .expect(1)
        .mount(&server)
        .await;
    let result = client(&server)
        .compute()
        .start_instance("x")
        .options(RequestOptions {
            timeout: Some(Duration::from_millis(20)),
            ..Default::default()
        })
        .send()
        .await;
    assert!(matches!(result, Err(Error::Timeout)));
}

#[tokio::test]
async fn invalid_json_or_schema_is_a_protocol_error() {
    for body in ["not json", "42", r#"{"image":{}}"#, r#"{"wrong":[]}"#] {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;
        assert!(matches!(
            client(&server).compute().get_image("x").send().await,
            Err(Error::Protocol(_))
        ));
    }
}

#[test]
fn enums_accept_future_values_and_idempotency_keys_are_unique() {
    let status: ImageStatus = serde_json::from_value(json!("future-status")).unwrap();
    assert_eq!(status, ImageStatus::Unknown("future-status".into()));
    assert_eq!(
        serde_json::to_value(status).unwrap(),
        json!("future-status")
    );
    let key = basaltic::new_idempotency_key();
    assert!(uuid::Uuid::parse_str(&key).is_ok());
    assert_ne!(key, basaltic::new_idempotency_key());
}
