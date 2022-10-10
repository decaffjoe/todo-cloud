use gcloud_sdk::{
    google_rest_apis::storage_v1::objects_api::{
        storage_objects_insert_ext_bytes, StoragePeriodObjectsPeriodInsertParams,
    },
    GoogleRestApi,
};

pub async fn upload_to_gcp(
    gcp_creds_file_path: String,
    bucket_name: String,
    encrypted_file: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // set env for gcp authentication
    std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", gcp_creds_file_path);

    // config gcp client; authenticates via GOOGLE_APPLICATION_CREDENTIALS env var
    let google_rest_client = GoogleRestApi::new().await?;

    // upload object
    match storage_objects_insert_ext_bytes(
        &google_rest_client.create_google_storage_v1_config().await?,
        StoragePeriodObjectsPeriodInsertParams {
            bucket: bucket_name,
            name: Some("todo".to_string()),
            ..Default::default()
        },
        None,
        encrypted_file.as_bytes().to_vec(),
    )
    .await
    {
        Ok(_) => {
            println!("Successfully updated cloud");
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
