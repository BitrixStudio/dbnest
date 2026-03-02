use crate::engine::Engine;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct InstanceSpec {
    pub engine: Engine,
    pub sqlite: Option<SqliteSpec>,
    pub postgres: Option<PostgresSpec>,
}

#[derive(Debug, Clone)]
pub struct SqliteSpec {
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct PostgresSpec {
    pub user: String,
    pub password: String,
    pub db: String,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub engine: Engine,
    pub backend: Backend,
    pub created_at: OffsetDateTime,
    pub connection: ConnectionInfo,

    #[serde(default)]
    pub sqlite: Option<SqliteInfo>,

    #[serde(default)]
    pub container: Option<ContainerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    Embedded,
    Container,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub database_url: String,

    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
    #[serde(default)]
    pub database: Option<String>,
    #[serde(default)]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    // Current implemented runtime "docker" (future: "podman", "kubernetes")
    pub runtime: String,
    pub container_id: String,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteInfo {
    /// Absolute path
    pub path: PathBuf,
    /// True if the file is managed by dbnest and can be deleted on rm
    pub managed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSummary {
    pub id: String,
    pub engine: Engine,
    pub backend: Backend,
    pub created_at: OffsetDateTime,
    pub database_url: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Running,
    Stopped,
    Unknown,
}

impl Instance {
    pub fn summary(&self) -> InstanceSummary {
        InstanceSummary {
            id: self.id.clone(),
            engine: self.engine,
            backend: self.backend.clone(),
            created_at: self.created_at,
            database_url: self.connection.database_url.clone(),
            status: Status::Unknown,
        }
    }
}
