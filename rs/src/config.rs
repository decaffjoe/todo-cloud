use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

use crate::error::TodoCloudError;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TodoConfig {
    pub gcp_bucket_name: String,
    pub gcp_creds_file_path: String,
    pub todo_file_path: String,
    pub encryption_passphrase: String,
}

pub fn create_config() -> Result<TodoConfig, Box<dyn Error>> {
    // read config from ~/.config/todo-cloud/todo-cloud.toml
    let config = confy::load("todo-cloud", "todo-cloud")
        .expect("Unable to read config file from $HOME/.config/todo-cloud/todo-cloud.toml");

    // validate configuration values
    validate_config(&config)?;

    Ok(config)
}

fn validate_config(config: &TodoConfig) -> Result<(), Box<dyn Error>> {
    // check that gcp credentials filepath is a valid file
    let creds_path = Path::new(&config.gcp_creds_file_path);
    if !creds_path.is_file() {
        return Err(Box::new(TodoCloudError::InvalidGcpCredentialsPath(
            config.gcp_creds_file_path.clone(),
        )) as Box<dyn std::error::Error>);
    }

    // check that todo filepath is a valid file
    let todo_path = Path::new(&config.todo_file_path);
    if !todo_path.is_file() {
        return Err(Box::new(TodoCloudError::InvalidTodoFilePath(
            config.todo_file_path.clone(),
        )) as Box<dyn std::error::Error>);
    }

    Ok(())
}
