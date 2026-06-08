use aws_sdk_s3::{Client, primitives::ByteStream, types::ObjectCannedAcl};

pub struct S3Client {
    inner: Client,
    pub bucket: String,
    pub endpoint_url: String,
}

impl S3Client {
    pub async fn new(endpoint_url: String, bucket: String) -> Self {
        let config = aws_config::load_from_env().await;
        let inner = aws_sdk_s3::Client::new(&config);

        Self {
            inner,
            endpoint_url,
            bucket,
        }
    }

    #[tracing::instrument(err(Debug), skip(self, content))]
    pub async fn upload(&self, key: String, content: ByteStream) -> color_eyre::Result<String> {
        let req = self
            .inner
            .put_object()
            .bucket(&self.bucket)
            .acl(ObjectCannedAcl::PublicRead)
            .key(&key)
            .body(content);
        req.send().await?;

        Ok(format!("{}/{}/{}", self.endpoint_url, self.bucket, key))
    }
}
