use base64::Engine;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedVec<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
}

pub fn encode<T: Serialize>(cursor: &T) -> color_eyre::Result<String> {
    let serialized = serde_json::to_vec(cursor)?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(serialized))
}

pub fn decode<T: DeserializeOwned>(s: &str) -> color_eyre::Result<T> {
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(s)?;
    Ok(serde_json::from_slice(&bytes)?)
}

const fn default_limit() -> usize {
    10
}
