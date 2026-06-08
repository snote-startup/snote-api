pub mod extractor;
pub mod handler;
pub mod model;
mod repository;
pub mod route;
pub mod service;

const AUTH_ENDPOINT: &str = "/auth";
const REFRESH_COOKIE: &str = "refresh";
