//! Generated API methods; do not edit.
#![allow(unused_imports)]
use crate::models::catalog as m;
use crate::transport::{Core, Operation, Payload, QueryEncoding};
use crate::{BinaryRequest, Client, EmptyRequest, PagedRequest, Request, WebSocketRequest};
const ENDPOINT: &str = "https://catalog.basaltic.sh";
static OP_0: Operation = Operation {
    id: "getCountry",
    method: "GET",
    path: "/v1/countries/{country}",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_1: Operation = Operation {
    id: "getRegion",
    method: "GET",
    path: "/v1/regions/{code}",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_2: Operation = Operation {
    id: "listCities",
    method: "GET",
    path: "/v1/cities",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[
        QueryEncoding {
            name: "state",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "country",
            style: "form",
            explode: true,
        },
        QueryEncoding {
            name: "q",
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
static OP_3: Operation = Operation {
    id: "listCountries",
    method: "GET",
    path: "/v1/countries",
    authenticated: false,
    required_query: &[],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[],
};
static OP_4: Operation = Operation {
    id: "listRegions",
    method: "GET",
    path: "/v1/regions",
    authenticated: false,
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
static OP_5: Operation = Operation {
    id: "listStates",
    method: "GET",
    path: "/v1/states",
    authenticated: false,
    required_query: &["country"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "country",
        style: "form",
        explode: true,
    }],
};
static OP_6: Operation = Operation {
    id: "lookupPostalCode",
    method: "GET",
    path: "/v1/postal-codes/{code}",
    authenticated: false,
    required_query: &["country"],
    required_headers: &[],
    content_type: "",
    accept: "application/json",
    query_encoding: &[QueryEncoding {
        name: "country",
        style: "form",
        explode: true,
    }],
};
#[derive(Clone, Debug)]
pub struct CatalogService {
    pub(crate) client: Client,
}
impl CatalogService {
    /// Get a country and its address rules
    pub fn get_country(&self, country: &str) -> Request<m::GetCountryResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "catalog",
            ENDPOINT,
            &OP_0,
            &[("country", country)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Get a region
    pub fn get_region(&self, code: &str) -> Request<m::GetRegionResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "catalog",
            ENDPOINT,
            &OP_1,
            &[("code", code)],
            Ok(Payload::Empty),
            Ok(serde_json::json!({})),
        ))
    }
    /// Resolve one resource by UUID, CRN, or an unambiguous name.
    pub fn get_region_by_reference(
        &self,
        reference: &str,
        scope: &m::GetRegionScope,
    ) -> Request<m::GetRegionResource> {
        let (core, extract) = crate::request::reference_core(
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_1,
                &[("code", reference)],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_4,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(scope).map_err(crate::Error::from),
            ),
            reference,
            true,
            None,
            "regions",
        );
        Request::reference(core, extract)
    }
    /// List cities
    pub fn list_cities(
        &self,
        query: &m::ListCitiesQuery,
    ) -> PagedRequest<m::ListCitiesResponse, m::ListCitiesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_2,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "cities",
        )
    }
    /// List countries
    pub fn list_countries(&self) -> PagedRequest<m::ListCountriesResponse, m::ListCountriesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_3,
                &[],
                Ok(Payload::Empty),
                Ok(serde_json::json!({})),
            ),
            "countries",
        )
    }
    /// List regions
    pub fn list_regions(
        &self,
        query: &m::ListRegionsQuery,
    ) -> PagedRequest<m::ListRegionsResponse, m::ListRegionsItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_4,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "regions",
        )
    }
    /// List a country's states
    pub fn list_states(
        &self,
        query: &m::ListStatesQuery,
    ) -> PagedRequest<m::ListStatesResponse, m::ListStatesItem> {
        PagedRequest::new(
            Core::new(
                self.client.clone(),
                "catalog",
                ENDPOINT,
                &OP_5,
                &[],
                Ok(Payload::Empty),
                serde_json::to_value(query).map_err(crate::Error::from),
            ),
            "states",
        )
    }
    /// Look up a postal code
    pub fn lookup_postal_code(
        &self,
        code: &str,
        query: &m::LookupPostalCodeQuery,
    ) -> Request<m::LookupPostalCodeResponse> {
        Request::new(Core::new(
            self.client.clone(),
            "catalog",
            ENDPOINT,
            &OP_6,
            &[("code", code)],
            Ok(Payload::Empty),
            serde_json::to_value(query).map_err(crate::Error::from),
        ))
    }
}
