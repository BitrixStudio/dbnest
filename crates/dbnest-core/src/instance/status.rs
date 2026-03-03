use crate::{
    engine::Engine,
    error::{DbnestError, Result},
    instance::{Instance, InstanceStatus, InstanceStatusReport, Registry},
};
use serde_json::json;
use std::process::Command;

pub fn status_one(id: &str) -> Result<InstanceStatusReport> {
    let registry = Registry::new()?;
    status_one_with_registry(&registry, id)
}

pub fn status_all() -> Result<Vec<InstanceStatusReport>> {
    let registry = Registry::new()?;
    status_all_with_registry(&registry)
}

pub fn status_one_with_registry(registry: &Registry, id: &str) -> Result<InstanceStatusReport> {
    let inst = registry
        .read(id)
        .map_err(|_| DbnestError::InstanceNotFound(id.to_string()))?;

    Ok(status_for_instance(&inst))
}

pub fn status_all_with_registry(registry: &Registry) -> Result<Vec<InstanceStatusReport>> {
    let instances = registry.list()?;
    Ok(instances.iter().map(status_for_instance).collect())
}

fn status_for_instance(inst: &Instance) -> InstanceStatusReport {
    match inst.engine {
        Engine::Sqlite => sqlite_status(inst),
        Engine::Postgres => postgres_status(inst),
        Engine::Mysql => InstanceStatusReport {
            id: inst.id.clone(),
            engine: inst.engine,
            status: InstanceStatus::Unknown,
            details: json!({"note": "mysql status not implemented yet"}),
        },
    }
}

fn sqlite_status(inst: &Instance) -> InstanceStatusReport {
    let Some(sqlite) = &inst.sqlite else {
        return InstanceStatusReport {
            id: inst.id.clone(),
            engine: inst.engine,
            status: InstanceStatus::Unknown,
            details: json!({"error": "missing sqlite info"}),
        };
    };

    let exists = sqlite.path.exists();
    InstanceStatusReport {
        id: inst.id.clone(),
        engine: inst.engine,
        status: if exists {
            InstanceStatus::Running
        } else {
            InstanceStatus::Missing
        },
        details: json!({
            "path": sqlite.path,
            "file_exists": exists,
            "managed": sqlite.managed
        }),
    }
}

fn postgres_status(inst: &Instance) -> InstanceStatusReport {
    let Some(container) = &inst.container else {
        return InstanceStatusReport {
            id: inst.id.clone(),
            engine: inst.engine,
            status: InstanceStatus::Unknown,
            details: json!({"error": "missing container info"}),
        };
    };

    // 1) container status from docker inspect
    let state = docker_container_state(&container.container_id);
    match state.as_deref() {
        None => InstanceStatusReport {
            id: inst.id.clone(),
            engine: inst.engine,
            status: InstanceStatus::Missing,
            details: json!({"container_id": container.container_id, "docker_state": null}),
        },
        Some("running") => {
            // 2) connectivity check
            let can_connect = postgres_can_connect(&inst.connection.database_url);
            InstanceStatusReport {
                id: inst.id.clone(),
                engine: inst.engine,
                status: if can_connect {
                    InstanceStatus::Running
                } else {
                    InstanceStatus::Unhealthy
                },
                details: json!({
                    "container_id": container.container_id,
                    "docker_state": "running",
                    "can_connect": can_connect
                }),
            }
        }
        Some(other) => InstanceStatusReport {
            id: inst.id.clone(),
            engine: inst.engine,
            status: InstanceStatus::Stopped,
            details: json!({
                "container_id": container.container_id,
                "docker_state": other
            }),
        },
    }
}

fn docker_container_state(container_id: &str) -> Option<String> {
    let out = Command::new("docker")
        .args(["inspect", "-f", "{{.State.Status}}", container_id])
        .output()
        .ok()?;

    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn postgres_can_connect(url: &str) -> bool {
    match postgres::Client::connect(url, postgres::NoTls) {
        Ok(mut c) => c.simple_query("SELECT 1;").is_ok(),
        Err(_) => false,
    }
}
