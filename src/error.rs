use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, ToSchema)]
pub struct Context {
    #[serde(skip)]
    pub status: StatusCode,

    pub message: String,

    pub detail: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Something went wrong".to_string(),
            detail: None,
        }
    }
}

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct Error {
    #[serde(flatten)]
    pub context: Context,

    #[serde(skip)]
    #[allow(dead_code)]
    pub inner: Option<color_eyre::eyre::Error>,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.context.status, Json(self.context)).into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<color_eyre::eyre::Error>,
{
    fn from(error: E) -> Self {
        Error {
            context: Default::default(),
            inner: Some(error.into()),
        }
    }
}

pub trait ResultExt<T>
where
    Self: Sized,
{
    fn with_context(self, status: StatusCode, message: &str) -> Result<T> {
        self.with_full_context(status, message, &None)
    }

    fn with_full_context(
        self,
        status: StatusCode,
        message: &str,
        detail: &Option<String>,
    ) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: Into<color_eyre::eyre::Error>,
{
    fn with_full_context(
        self,
        status: StatusCode,
        message: &str,
        detail: &Option<String>,
    ) -> Result<T> {
        self.map_err(|error| Error {
            context: Context {
                status,
                message: message.to_string(),
                detail: detail.clone(),
            },
            inner: Some(error.into()),
        })
    }
}

pub trait OptionExt<T>
where
    Self: Sized,
{
    fn with_context(self, status: StatusCode, message: &str) -> Result<T> {
        self.with_full_context(status, message, &None)
    }

    fn with_full_context(
        self,
        status: StatusCode,
        message: &str,
        detail: &Option<String>,
    ) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn with_full_context(
        self,
        status: StatusCode,
        message: &str,
        detail: &Option<String>,
    ) -> Result<T> {
        self.ok_or_else(|| Error {
            context: Context {
                status,
                message: message.to_string(),
                detail: detail.clone(),
            },
            inner: None,
        })
    }
}
