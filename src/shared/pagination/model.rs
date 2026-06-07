use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
    pub cursor: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedVec<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
}

const fn default_limit() -> u32 {
    10
}
