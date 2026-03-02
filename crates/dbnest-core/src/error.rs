use thiserror::Error;

pub type Result<T> = std::result::Result<T, DbnestError>;

#[derive(Debug, Error)]
pub enum DbnestError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("instance not found: {0}")]
    InstanceNotFound(String),

    #[error("docker not available (required for postgres/mysql in v1)")]
    DockerNotAvailable,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}