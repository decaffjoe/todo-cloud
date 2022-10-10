#[derive(Debug)]
pub enum TodoCloudError {
    InvalidGcpCredentialsPath(String),
    InvalidTodoFilePath(String),
}
impl std::fmt::Display for TodoCloudError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoCloudError::InvalidGcpCredentialsPath(path) => write!(
                f,
                "Invalid gcp credentials file {} specified in configuration file",
                path
            ),
            TodoCloudError::InvalidTodoFilePath(path) => write!(
                f,
                "Invalid todo file {} specified in configuration file",
                path
            ),
        }
    }
}
impl std::error::Error for TodoCloudError {}
