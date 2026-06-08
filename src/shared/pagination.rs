use base64::Engine;
use http::StatusCode;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use utoipa::ToSchema;

use crate::error::{Result, ResultExt};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub cursor: Option<String>,
}

const fn default_limit() -> u32 {
    10
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedVec<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
}

pub fn encode<T: Serialize>(cursor: &T) -> Result<String> {
    let serialized = serde_json::to_vec(cursor)?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(serialized))
}

pub fn decode<T: DeserializeOwned>(s: &str) -> Result<T> {
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .with_context(StatusCode::BAD_REQUEST, "Invalid pagination cursor")?;
    serde_json::from_slice(&bytes)
        .with_context(StatusCode::BAD_REQUEST, "Invalid pagination cursor")
}
