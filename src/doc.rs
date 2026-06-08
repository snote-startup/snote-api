use std::sync::Arc;

use axum::Router;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::feature::auth::handler as auth;
use crate::{error::Error, shared::ApiState};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register,
        auth::login,
        auth::refresh,
        auth::me,

        // project::handler::create,
        // project::handler::get_by_account,
        // project::handler::get,
        // project::handler::update,
        // project::handler::upload_audio,
        // project::handler::get_transcript,
        // project::handler::get_chat_messages,
        // project::handler::chat
    ),
    components(schemas(Error,)),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

pub fn build() -> Router<Arc<ApiState>> {
    SwaggerUi::new("/swagger")
        .url("/api-docs/openapi.json", ApiDoc::openapi())
        .into()
}
