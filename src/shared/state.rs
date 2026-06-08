use sqlx::PgPool;

use crate::{
    feature::auth::service::{AuthService, PartialTokenService, TokenService},
    infra::{storage::S3Client, transcript::AssemblyAIClient},
    shared::Config,
};

pub struct ApiState {
    pub db: PgPool,

    pub s3_client: S3Client,
    pub assembly_ai_client: AssemblyAIClient,

    pub token_service: TokenService,
    pub auth_service: AuthService,
}

impl ApiState {
    pub async fn new(config: Config) -> color_eyre::Result<ApiState> {
        Ok(ApiState {
            db: PgPool::connect(&config.database_url).await?,

            s3_client: S3Client::new(config.aws_endpoint_url.clone(), config.s3_bucket.clone())
                .await,

            assembly_ai_client: AssemblyAIClient {
                api_key: config.assembly_ai_api_key.clone(),
            },

            token_service: TokenService {
                access: PartialTokenService::new(&config.jwt_secret, config.jwt_expired_in),
                refresh: PartialTokenService::new(
                    &config.jwt_refresh_secret,
                    config.jwt_refresh_expired_in,
                ),
            },

            auth_service: AuthService,
        })
    }
}
