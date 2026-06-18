use sqlx::PgPool;

use crate::{
    feature::{
        auth::service::{AuthService, PartialTokenService, TokenService},
        chat::service::ChatService,
        project::service::ProjectService,
    },
    infra::{storage::S3Client, transcript::AssemblyAIClient},
    shared::Config,
};

pub struct ApiState {
    pub db: PgPool,

    pub s3: S3Client,
    pub assembly_ai: AssemblyAIClient,

    pub token_svc: TokenService,
    pub auth_svc: AuthService,
    pub chat_svc: ChatService,
    pub project_svc: ProjectService,
}

impl ApiState {
    pub async fn new(config: Config) -> color_eyre::Result<ApiState> {
        Ok(ApiState {
            db: PgPool::connect(&config.database_url).await?,

            s3: S3Client::new(config.aws_endpoint_url.clone(), config.s3_bucket.clone()).await,

            assembly_ai: AssemblyAIClient::new(&config.assembly_ai_api_key.to_string()),

            token_svc: TokenService {
                access: PartialTokenService::new(&config.jwt_secret, config.jwt_expired_in),
                refresh: PartialTokenService::new(
                    &config.jwt_refresh_secret,
                    config.jwt_refresh_expired_in,
                ),
            },

            auth_svc: AuthService,

            chat_svc: ChatService::new(
                &config.gemini_api_key,
                config.chat_context_transcript_size,
                config.chat_context_history_size,
            )?,

            project_svc: ProjectService,
        })
    }
}
