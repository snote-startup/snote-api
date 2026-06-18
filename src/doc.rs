use std::sync::Arc;

use axum::Router;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::feature::{
    auth::handler as auth, chat::handler as chat, project::handler as project,
    task::handler as task,
};
use crate::{error::Error, shared::ApiState, shared::health};

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
        health::health,

        auth::register,
        auth::login,
        auth::refresh,
        auth::me,

        project::create,
        project::get_by_account,
        project::get,
        project::update,
        project::upload_audio,
        project::create_transcript,
        project::get_transcript,

        chat::chat,
        chat::get_history,

        task::create,
        task::get_by_project,
        task::update,
        task::delete
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
