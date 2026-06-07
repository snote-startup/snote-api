use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Token not found")]
    TokenNotFound,

    #[error("Invalid token")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),

    #[error("Invalid pagination metadata")]
    InvalidPaginationMetadata,

    #[error("Internal server error")]
    Internal(#[from] color_eyre::eyre::Error),
}
