use std::{fs, path::Path};

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use tracing::Level;

#[derive(Debug, Default, Serialize, Deserialize)]
struct TodoConfig {
    gcp_bucket_name: String,
    gcp_creds_file_path: String,
    todo_file_path: String,
    encryption_passphrase: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // read config
    let cfg: TodoConfig =
        confy::load("todo-cloud", "todo-cloud").expect("Unable to read config file");

    // read file
    let todo_path = Path::new(&cfg.todo_file_path);
    let file_contents = fs::read_to_string(todo_path).expect("Unable to read todo file");

    // encrypt file
    let mc = new_magic_crypt!(cfg.encryption_passphrase, 256);
    let encrypted_file = mc.encrypt_str_to_base64(file_contents);

    upload_to_gcp(cfg.gcp_bucket_name, encrypted_file).await
}

async fn upload_to_gcp(
    bucket_name: String,
    encrypted_file: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // config gcp client
    let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

    // TODO - use explicit gcloud authorization via credentials file
    let response = gcloud_sdk::google_rest_apis::storage_v1::objects_api::storage_objects_insert_ext_bytes(
            &google_rest_client.create_google_storage_v1_config().await?,
            gcloud_sdk::google_rest_apis::storage_v1::objects_api::StoragePeriodObjectsPeriodInsertParams {
                bucket: bucket_name,
                name: Some("todo".to_string()),
                ..Default::default()
            },
            None,
            encrypted_file.as_bytes().to_vec()
        ).await?;

    println!("{:?}", response);

    Ok(())
}
