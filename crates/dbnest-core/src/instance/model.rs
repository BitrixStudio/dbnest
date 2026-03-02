use crate::engine::Engine;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct InstanceSpec {
    pub engine: Engine,
    pub sqlite: Option<SqliteSpec>,
    // postgres/mysql spec will come later (docker)
}

#[derive(Debug, Clone)]
pub struct SqliteSpec {
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub id: String,
    pub engine: Engine,
    pub backend: Backend,
    pub created_at: OffsetDateTime,
    pub connection: ConnectionInfo,
    pub sqlite: Option<SqliteInfo>,
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
