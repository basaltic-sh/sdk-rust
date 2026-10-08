use crate::{Error, Result};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A present nullable field. `Option<Nullable<T>>` distinguishes omission from null.
#[derive(Clone, Debug, PartialEq)]
pub enum Nullable<T> {
    Null,
    Value(T),
}
impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Value(v) => v.serialize(serializer),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Ok(Option::<T>::deserialize(deserializer)?
            .map(Self::Value)
            .unwrap_or(Self::Null))
    }
}
/// Preserve explicit null for optional nullable fields during deserialization.
pub(crate) fn present_nullable<'de, D, T>(
    deserializer: D,
) -> std::result::Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

#[derive(Clone, Debug)]
pub struct ApiResponse<T> {
    pub data: T,
    pub status: u16,
    pub headers: HeaderMap,
}
impl<T> ApiResponse<T> {
    pub fn request_id(&self) -> &str {
        self.headers
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
    }
    pub(crate) fn map<U>(self, data: U) -> ApiResponse<U> {
        ApiResponse {
            data,
            status: self.status,
            headers: self.headers,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Page<T, I> {
    pub response: ApiResponse<T>,
    pub items: Vec<I>,
    pub has_more: bool,
    pub marker: Option<String>,
}
impl<T, I> Page<T, I> {
    pub fn request_id(&self) -> &str {
        self.response.request_id()
    }
    pub(crate) fn next_marker(
        &self,
        seen: &mut std::collections::HashSet<String>,
    ) -> Result<Option<String>> {
        if !self.has_more {
            return Ok(None);
        }
        match &self.marker {
            Some(m) if !m.is_empty() && seen.insert(m.clone()) => Ok(Some(m.clone())),
            _ => Err(Error::Protocol("Pagination did not advance".into())),
        }
    }
}

/// Prepared WebSocket URL and authentication headers. Connect with your chosen
/// WebSocket library; keep the headers private and do not follow cross-host redirects.
pub struct WebSocketConnection {
    pub url: String,
    pub headers: HeaderMap,
}
impl std::fmt::Debug for WebSocketConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocketConnection")
            .finish_non_exhaustive()
    }
}
