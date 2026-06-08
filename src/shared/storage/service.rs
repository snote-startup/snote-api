use aws_sdk_s3::{Client, primitives::ByteStream, types::ObjectCannedAcl};

use crate::{config::Config, error::Result};

pub struct StorageService {
    pub inner: Client,
    pub aws_endpoint_url: String,
    pub s3_bucket: String,
}

impl StorageService {
    pub async fn new(config: &Config) -> color_eyre::Result<Self> {
        let s3_config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&s3_config);

        Ok(Self {
            inner: client,
            aws_endpoint_url: config.aws_endpoint_url.to_string(),
            s3_bucket: config.s3_bucket.to_string(),
        })
    }

    pub async fn upload(&self, key: String, content: ByteStream) -> Result<String> {
        let req = self
            .inner
            .put_object()
            .bucket(&self.s3_bucket)
            .acl(ObjectCannedAcl::PublicRead)
            .key(&key)
            .body(content);
        req.send().await.map_err(color_eyre::eyre::Error::from)?;

        Ok(format!(
            "{}/{}/{}",
            self.aws_endpoint_url, self.s3_bucket, key
        ))
    }
}
