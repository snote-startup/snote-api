use aws_sdk_s3::{Client, primitives::ByteStream, types::ObjectCannedAcl};

use crate::Result;

pub struct S3Service {
    pub client: Client,
    pub endpoint_url: String,
    pub bucket: String,
}

impl S3Service {
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
