use aws_sdk_s3::{Client, primitives::ByteStream, types::ObjectCannedAcl};

pub struct CloudStorageUtil {
    client: Client,
}

impl CloudStorageUtil {
    pub async fn new() -> CloudStorageUtil {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        CloudStorageUtil { client }
    }

    pub async fn upload(
        &self,
        key: &str,
        body: impl Into<ByteStream>,
    ) -> color_eyre::Result<String> {
        let req = self
            .client
            .put_object()
            .bucket(&CONFIG.bucket)
            .acl(ObjectCannedAcl::PublicRead)
            .key(key)
            .body(body.into());
        req.send().await?;

        todo!()
    }
}
