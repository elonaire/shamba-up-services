use std::time::Duration;

use async_graphql::{Context, Object, Upload};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{
    presigning::PresigningConfig,
    Client,
};

use crate::graphql::schemas::file::TestResponse;

// const CHUNK_SIZE: u64 = 1024 * 1024 * 5; // 5MB

pub struct Mutation;

#[Object]
impl Mutation {
    // multipart upload to AWS S3
    async fn upload_file(
        &self,
        ctx: &Context<'_>,
        file: Upload,
    ) -> async_graphql::Result<TestResponse> {
        println!("file: {:?}", file.value(ctx).unwrap().size());
        // let file_size_in_mb = file.value(ctx).unwrap().size().unwrap() / 1024 / 1024;
        // let file_size_in_bytes = file.value(ctx).unwrap().size().unwrap();
        let bucket = "shamba-up-files";
        let key = "test_file";

        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&config);

        // let multipart_upload_res: CreateMultipartUploadOutput = client
        //     .create_multipart_upload()
        //     .bucket(bucket.clone())
        //     .key(key.clone())
        //     .send()
        //     .await
        //     .unwrap();

        let presigned_url_expiry = Duration::from_secs(60 * 60 * 24 * 7);
        // let mut presigned_urls = vec![];
        let mut response = TestResponse {
            message: "test".to_string(),
            // presigned_urls: None,
            // upload_id: None,
            presigned_url: None,
        };

        // if file_size_in_bytes > CHUNK_SIZE {
        //     let upload_id = multipart_upload_res.upload_id.unwrap();
        //     // upload in parts
        //     let iterations = if file_size_in_bytes % CHUNK_SIZE == 0 {
        //         file_size_in_bytes / CHUNK_SIZE
        //     } else {
        //         file_size_in_bytes / CHUNK_SIZE + 1
        //     };

        //     for i in 0..iterations {
        //         let part_number = i + 1;
        //         let presigned_request = client
        //             .upload_part()
        //             .bucket(bucket.clone())
        //             .key(key.clone())
        //             .part_number(part_number.try_into().unwrap())
        //             .upload_id(upload_id.clone())
        //             .presigned(PresigningConfig::expires_in(presigned_url_expiry)?)
        //             .await?;
        //         let presigned_url = presigned_request.uri().to_string();

        //         presigned_urls.push(presigned_url);

        //         // println!("presigned_request: {:?}", presigned_request.uri());
        //     }

        //     response.presigned_urls = Some(presigned_urls);
        //     response.upload_id = Some(upload_id);
        // } else {
        //     // upload in one go
        //     let presigned_request = client
        //         .put_object()
        //         .bucket(bucket.clone())
        //         .key(key.clone())
        //         .presigned(PresigningConfig::expires_in(presigned_url_expiry)?)
        //         .await?;

        //     let presigned_url = presigned_request.uri().to_string();
        //     response.presigned_url = Some(presigned_url);

        //     println!("presigned_request: {:?}", presigned_request.uri());
        // }

        // upload in one go
        let presigned_request = client
            .put_object()
            .bucket(bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(presigned_url_expiry)?)
            .await?;

        let presigned_url = presigned_request.uri().to_string();
        response.presigned_url = Some(presigned_url);

        println!("presigned_request: {:?}", presigned_request.uri());

        Ok(response)
    }
}
