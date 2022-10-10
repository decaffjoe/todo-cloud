mod config;
mod error;
mod gcp;

use crate::{config::create_config, gcp::upload_to_gcp};

use std::{fs, path::Path};

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Uploading todo file to cloud...");

    // Debug logging
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // get configuration file values
    let config = create_config()?;

    // read the todo file
    let todo_path = Path::new(&config.todo_file_path);
    let file_contents = fs::read_to_string(todo_path).expect("Unable to read todo file");

    // encrypt the todo file
    let mc = new_magic_crypt!(config.encryption_passphrase, 256);
    let encrypted_file = mc.encrypt_str_to_base64(file_contents);

    // upload encrypted file to gcp bucket
    upload_to_gcp(
        config.gcp_creds_file_path,
        config.gcp_bucket_name,
        encrypted_file,
    )
    .await
}
