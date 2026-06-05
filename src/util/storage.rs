use aws_sdk_s3::{Client, primitives::ByteStream, types::ObjectCannedAcl};

use crate::config::CONFIG;

pub async fn upload(s3: &Client, key: String, content: ByteStream) -> color_eyre::Result<String> {
    let req = s3
        .put_object()
        .bucket(&CONFIG.s3_bucket)
        .acl(ObjectCannedAcl::PublicRead)
        .key(&key)
        .body(content);
    req.send().await?;

    Ok(format!(
        "{}/{}/{}",
        CONFIG.aws_endpoint_url, CONFIG.s3_bucket, key
    ))
}
