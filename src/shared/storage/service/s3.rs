use aws_sdk_s3::{Client, primitives::ByteStream, types::ObjectCannedAcl};

use crate::{config::Config, error::Result};

pub struct S3Service {
    pub client: Client,
    pub endpoint_url: String,
    pub bucket: String,
}

impl S3Service {
    pub async fn new(config: &Config) -> color_eyre::Result<Self> {
        let s3_config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&s3_config);

        Ok(Self {
            client,
            endpoint_url: config.aws_endpoint_url.to_string(),
            bucket: config.s3_bucket.to_string(),
        })
    }

    pub async fn upload(&self, key: String, content: ByteStream) -> Result<String> {
        let req = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .acl(ObjectCannedAcl::PublicRead)
            .key(&key)
            .body(content);
        req.send().await.map_err(color_eyre::eyre::Error::from)?;

        Ok(format!("{}/{}/{}", self.endpoint_url, self.bucket, key))
    }
}
