use sqlx::PgPool;

use crate::{
    feature::{
        auth::service::{AuthService, PartialTokenService, TokenService},
        chat::service::ChatService,
        payment_test::service::PaymentTestService,
        project::service::ProjectService,
        task::service::TaskService,
    },
    infra::{payment::PayOSClient, storage::S3Client, transcript::AssemblyAIClient},
    shared::Config,
};

pub struct ApiState {
    pub db: PgPool,

    pub s3: S3Client,
    pub assembly_ai: AssemblyAIClient,
    pub payos: PayOSClient,

    pub token_svc: TokenService,
    pub auth_svc: AuthService,
    pub project_svc: ProjectService,
    pub chat_svc: ChatService,
    pub task_svc: TaskService,

    pub payment_test_svc: PaymentTestService,
}

impl ApiState {
    pub async fn new(config: Config) -> color_eyre::Result<ApiState> {
        Ok(ApiState {
            db: PgPool::connect(&config.database_url).await?,

            s3: S3Client::new(config.aws_endpoint_url, config.s3_bucket).await,

            assembly_ai: AssemblyAIClient::new(config.assembly_ai_api_key),

            payos: PayOSClient::new(
                config.payos_client_id,
                config.payos_api_key,
                config.payos_checksum_key,
            ),

            token_svc: TokenService {
                access: PartialTokenService::new(&config.jwt_secret, config.jwt_expired_in),
                refresh: PartialTokenService::new(
                    &config.jwt_refresh_secret,
                    config.jwt_refresh_expired_in,
                ),
            },

            auth_svc: AuthService,

            project_svc: ProjectService,

            chat_svc: ChatService::new(
                &config.gemini_api_key,
                config.chat_context_transcript_size,
                config.chat_context_history_size,
            )?,

            task_svc: TaskService::new(&config.gemini_api_key)?,

            payment_test_svc: PaymentTestService::new(config.base_url),
        })
    }
}
