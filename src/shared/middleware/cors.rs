use axum::http::{
    HeaderName, HeaderValue, Method,
    header::{
        ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
    },
};
use tower_http::cors::CorsLayer;

pub const ALLOW_METHODS: [Method; 6] = [
    Method::GET,
    Method::POST,
    Method::DELETE,
    Method::PATCH,
    Method::PUT,
    Method::OPTIONS,
];

pub const ALLOW_HEADERS: [HeaderName; 7] = [
    ORIGIN,
    AUTHORIZATION,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
    ACCEPT,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_HEADERS,
];

pub fn cors(origins: &str) -> CorsLayer {
    let origins: Vec<_> = origins
        .split(',')
        .map(|origin| origin.parse::<HeaderValue>().unwrap())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_headers(ALLOW_HEADERS)
        .expose_headers(ALLOW_HEADERS)
        .allow_credentials(true)
        .allow_methods(ALLOW_METHODS)
}
