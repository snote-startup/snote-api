use base64::Engine;
use serde::{Serialize, de::DeserializeOwned};

use crate::{Error, Result};

pub fn encode<T: Serialize>(cursor: &T) -> Result<String> {
    let serialized = serde_json::to_vec(cursor).map_err(color_eyre::eyre::Error::from)?;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(serialized))
}

pub fn decode<T: DeserializeOwned>(s: &str) -> Result<T> {
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(s)
        .map_err(|_| Error::InvalidPaginationMetadata)?;
    serde_json::from_slice(&bytes).map_err(|_| Error::InvalidPaginationMetadata)
}
