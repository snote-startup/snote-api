pub mod extractor;
pub mod handler;
pub mod model;
mod repository;
pub mod routes;
pub mod service;

pub use routes::*;

const AUTH_ENDPOINT: &str = "/auth";
const REFRESH_COOKIE: &str = "refresh";
