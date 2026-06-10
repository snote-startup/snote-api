use sqlx::PgPool;

use crate::{
    feature::{
        auth::service::{AuthService, PartialTokenService, TokenService},
        project::service::{ChatService, ProjectService},
    },
    infra::{storage::S3Client, transcript::AssemblyAIClient},
    shared::Config,
};

pub struct ApiState {
    pub db: PgPool,

    pub s3_client: S3Client,
    pub assembly_ai_client: AssemblyAIClient,

    pub token_service: TokenService,
    pub auth_service: AuthService,
    pub chat_service: ChatService,
    pub project_service: ProjectService,
}

impl ApiState {
    pub async fn new(config: Config) -> color_eyre::Result<ApiState> {
        Ok(ApiState {
            db: PgPool::connect(&config.database_url).await?,

            s3_client: S3Client::new(config.aws_endpoint_url.clone(), config.s3_bucket.clone())
                .await,

            assembly_ai_client: AssemblyAIClient::new(&config.assembly_ai_api_key.to_string()),

            token_service: TokenService {
                access: PartialTokenService::new(&config.jwt_secret, config.jwt_expired_in),
                refresh: PartialTokenService::new(
                    &config.jwt_refresh_secret,
                    config.jwt_refresh_expired_in,
                ),
            },

            auth_service: AuthService,

            chat_service: ChatService::new(
                &config.gemini_api_key,
                config.chat_context_transcript_size,
                config.chat_context_history_size,
            )?,

            project_service: ProjectService,
        })
    }
}
