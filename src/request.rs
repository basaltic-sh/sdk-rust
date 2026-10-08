use crate::{
    ApiError, ApiResponse, Error, Page, RequestOptions, Result, WebSocketConnection,
    error::invalid,
    transport::{Core, json_response},
};
use futures_util::{
    StreamExt, TryStreamExt,
    stream::{self, BoxStream},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{collections::HashSet, marker::PhantomData};

pub(crate) enum Extract {
    Normal,
    Envelope(&'static str),
    ReferenceList(&'static str),
}

/// An owned, typed JSON request. Constructing a request performs no network IO.
#[must_use = "Requests are executed by calling send().await"]
pub struct Request<T> {
    core: Result<Core>,
    extract: Extract,
    marker: PhantomData<T>,
}
impl<T: DeserializeOwned> Request<T> {
    pub(crate) fn new(core: Result<Core>) -> Self {
        Self {
            core,
            extract: Extract::Normal,
            marker: PhantomData,
        }
    }
    pub(crate) fn reference(core: Result<Core>, extract: Extract) -> Self {
        Self {
            core,
            extract,
            marker: PhantomData,
        }
    }
    pub fn options(mut self, options: RequestOptions) -> Self {
        if let Ok(core) = &mut self.core {
            core.options = options;
        }
        self
    }
    pub async fn send(self) -> Result<ApiResponse<T>> {
        let response = json_response(self.core?.send().await?).await?;
        let value = match self.extract {
            Extract::Normal => response.data.clone(),
            Extract::Envelope(key) => response
                .data
                .get(key)
                .cloned()
                .ok_or_else(|| Error::Protocol("Missing resource envelope".into()))?,
            Extract::ReferenceList(key) => {
                let items = response
                    .data
                    .get(key)
                    .and_then(Value::as_array)
                    .ok_or_else(|| Error::Protocol("Missing resource collection".into()))?;
                if items.len() > 1
                    || response
                        .data
                        .pointer("/meta/has_more")
                        .and_then(Value::as_bool)
                        == Some(true)
                {
                    return Err(Error::AmbiguousReference);
                }
                items.first().cloned().ok_or_else(|| {
                    Error::Api(Box::new(ApiError {
                        status: 404,
                        code: "REFERENCE_NOT_FOUND".into(),
                        message: "No resource matches the reference".into(),
                        request_id: response.request_id().into(),
                        operation: "resolve_reference".into(),
                        headers: response.headers.clone(),
                    }))
                })?
            }
        };
        let data = serde_json::from_value(value)?;
        Ok(response.map(data))
    }
}

#[must_use = "Requests are executed by calling send().await or consuming their stream"]
pub struct PagedRequest<T, I> {
    core: Result<Core>,
    items_key: &'static str,
    marker: PhantomData<(T, I)>,
}
impl<T: DeserializeOwned + Send + 'static, I: DeserializeOwned + Send + 'static>
    PagedRequest<T, I>
{
    pub(crate) fn new(core: Result<Core>, items_key: &'static str) -> Self {
        Self {
            core,
            items_key,
            marker: PhantomData,
        }
    }
    pub fn options(mut self, options: RequestOptions) -> Self {
        if let Ok(core) = &mut self.core {
            core.options = options;
        }
        self
    }
    pub async fn send(self) -> Result<Page<T, I>> {
        let response = json_response(self.core?.send().await?).await?;
        let values = response
            .data
            .get(self.items_key)
            .filter(|v| v.is_array())
            .ok_or_else(|| Error::Protocol("List response is missing its items array".into()))?;
        let items = serde_json::from_value(values.clone())?;
        let has_more = response
            .data
            .pointer("/meta/has_more")
            .and_then(Value::as_bool)
            == Some(true);
        let marker = response
            .data
            .pointer("/meta/marker")
            .and_then(Value::as_str)
            .map(str::to_owned);
        let data = serde_json::from_value(response.data.clone())?;
        Ok(Page {
            response: response.map(data),
            items,
            has_more,
            marker,
        })
    }
    /// Lazily request subsequent pages and fail if a marker does not advance.
    pub fn pages(self) -> BoxStream<'static, Result<Page<T, I>>> {
        let items_key = self.items_key;
        let initial = self
            .core
            .as_ref()
            .ok()
            .and_then(|c| c.query.get("marker"))
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        stream::try_unfold(
            (self.core, HashSet::from([initial]), false),
            move |(core, mut seen, done)| async move {
                if done {
                    return Ok(None);
                }
                let mut core = core?;
                let page = PagedRequest::<T, I>::new(core.try_clone(), items_key)
                    .send()
                    .await?;
                let next = page.next_marker(&mut seen)?;
                let done = next.is_none();
                if let Some(marker) = next {
                    core.query
                        .as_object_mut()
                        .ok_or_else(|| invalid("Pagination requires an object query"))?
                        .insert("marker".into(), Value::String(marker));
                }
                Ok(Some((page, (Ok(core), seen, done))))
            },
        )
        .boxed()
    }
    pub fn items(self) -> BoxStream<'static, Result<I>> {
        self.pages()
            .map_ok(|p| stream::iter(p.items.into_iter().map(Ok)))
            .try_flatten()
            .boxed()
    }
}

/// Streaming binary/HEAD response. Consume bytes/chunks or drop it to close.
#[must_use = "Requests are executed by calling send().await"]
pub struct BinaryRequest {
    core: Result<Core>,
}
impl BinaryRequest {
    pub(crate) fn new(core: Result<Core>) -> Self {
        Self { core }
    }
    pub fn options(mut self, options: RequestOptions) -> Self {
        if let Ok(core) = &mut self.core {
            core.options = options;
        }
        self
    }
    pub async fn send(self) -> Result<reqwest::Response> {
        self.core?.send().await
    }
}
#[must_use = "Requests are executed by calling send().await"]
pub struct EmptyRequest {
    core: Result<Core>,
}
impl EmptyRequest {
    pub(crate) fn new(core: Result<Core>) -> Self {
        Self { core }
    }
    pub fn options(mut self, options: RequestOptions) -> Self {
        if let Ok(core) = &mut self.core {
            core.options = options;
        }
        self
    }
    pub async fn send(self) -> Result<ApiResponse<()>> {
        let response = self.core?.send().await?;
        Ok(ApiResponse {
            data: (),
            status: response.status().as_u16(),
            headers: response.headers().clone(),
        })
    }
}
#[must_use = "Call prepare().await to obtain connection parameters"]
pub struct WebSocketRequest {
    core: Result<Core>,
}
impl WebSocketRequest {
    pub(crate) fn new(core: Result<Core>) -> Self {
        Self { core }
    }
    pub fn options(mut self, options: RequestOptions) -> Self {
        if let Ok(core) = &mut self.core {
            core.options = options;
        }
        self
    }
    pub async fn prepare(self) -> Result<WebSocketConnection> {
        self.core?.websocket().await
    }
}

pub(crate) fn reference_core(
    get: Result<Core>,
    list: Result<Core>,
    reference: &str,
    has_name: bool,
    envelope: Option<&'static str>,
    items_key: &'static str,
) -> (Result<Core>, Extract) {
    if reference.is_empty() {
        return (
            Err(invalid("Resource reference cannot be empty")),
            Extract::Normal,
        );
    }
    if reference.len() == 36 && uuid::Uuid::parse_str(reference).is_ok() {
        return (
            get,
            envelope.map(Extract::Envelope).unwrap_or(Extract::Normal),
        );
    }
    let kind = if reference.starts_with("crn:") {
        if reference.split(':').count() != 5 {
            return (
                Err(invalid("A CRN requires five colon-separated segments")),
                Extract::Normal,
            );
        }
        "crn"
    } else {
        if !has_name {
            return (
                Err(invalid("This resource requires a UUID or CRN")),
                Extract::Normal,
            );
        }
        "name"
    };
    let core = list.and_then(|mut core| {
        let query = core
            .query
            .as_object_mut()
            .ok_or_else(|| invalid("Reference scope must be an object"))?;
        for name in ["name", "crn", "marker"] {
            query.remove(name);
        }
        query.insert(kind.into(), Value::String(reference.into()));
        if core.op.query_encoding.iter().any(|e| e.name == "limit") {
            query.insert("limit".into(), Value::Number(2.into()));
        }
        Ok(core)
    });
    (core, Extract::ReferenceList(items_key))
}
