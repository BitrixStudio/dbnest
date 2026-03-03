use serde::{Deserialize, Serialize};

use crate::engine::Engine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceStatusReport {
    pub id: String,
    pub engine: Engine,
    pub status: InstanceStatus,
    #[serde(default)]
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InstanceStatus {
    Running,
    Stopped,
    Unhealthy,
    Missing,
    Unknown,
}
