use aws_sdk_s3::{Client, primitives::ByteStream, types::ObjectCannedAcl};

pub async fn upload(
    s3: &Client,
    endpoint_url: &str,
    bucket: &str,
    key: String,
    content: ByteStream,
) -> color_eyre::Result<String> {
    let req = s3
        .put_object()
        .bucket(bucket)
        .acl(ObjectCannedAcl::PublicRead)
        .key(&key)
        .body(content);
    req.send().await?;

    Ok(format!("{}/{}/{}", endpoint_url, bucket, key))
}
