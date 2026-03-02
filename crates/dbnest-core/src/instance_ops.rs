use crate::{
    engine::Engine,
    error::{DbnestError, Result},
    instance::{Instance, InstanceSpec, InstanceSummary, Registry},
    sqlite::provision_sqlite,
};

use crate::schema::apply::apply_sqlite_plan;
use crate::schema::load::{load_schema_dir, load_schema_json};
use crate::schema::plan::generators::plan_sqlite;
use crate::schema::validate::validate_schema;
use std::path::Path;

use crate::runtime::docker::postgres::provision_postgres_docker;

pub fn provision(spec: InstanceSpec) -> Result<Instance> {
    match spec.engine {
        Engine::Sqlite => provision_sqlite(spec),
        Engine::Postgres => provision_postgres_docker(spec),
        Engine::Mysql => Err(DbnestError::InvalidArgument(
            "MySQL not implemented yet".into(),
        )),
    }
}

pub fn list_instances() -> Result<Vec<InstanceSummary>> {
    let registry = Registry::new()?;
    let mut out = Vec::new();
    for inst in registry.list()? {
        let mut s = inst.summary();
        if let Some(sqlite) = &inst.sqlite {
            s.status = if sqlite.path.exists() {
                crate::Status::Running
            } else {
                crate::Status::Unknown
            };
        }
        out.push(s);
    }
    Ok(out)
}

pub fn stop_instance(id: &str) -> Result<()> {
    let registry = Registry::new()?;
    let inst = registry
        .read(id)
        .map_err(|_| DbnestError::InstanceNotFound(id.into()))?;

    match inst.engine {
        Engine::Sqlite => Ok(()),
        Engine::Postgres => {
            let c = inst
                .container
                .as_ref()
                .ok_or_else(|| DbnestError::InvalidArgument("missing container info".into()))?;
            let out = std::process::Command::new("docker")
                .args(["stop", &c.container_id])
                .output()?;
            if !out.status.success() {
                let stderr = String::from_utf8_lossy(&out.stderr);
                return Err(DbnestError::InvalidArgument(format!(
                    "docker stop failed: {stderr}"
                )));
            }
            Ok(())
        }
        Engine::Mysql => Ok(()),
    }
}

pub fn remove_instance(id: &str) -> Result<()> {
    let registry = Registry::new()?;
    let inst = registry
        .read(id)
        .map_err(|_| DbnestError::InstanceNotFound(id.into()))?;

    match inst.engine {
        Engine::Sqlite => {
            if inst.engine == Engine::Sqlite {
                if let Some(sqlite) = &inst.sqlite {
                    if sqlite.managed && sqlite.path.exists() {
                        let _ = std::fs::remove_file(&sqlite.path);
                        if let Some(parent) = sqlite.path.parent() {
                            let _ = std::fs::remove_dir(parent);
                        }
                    }
                }
            }
        }
        Engine::Postgres => {
            if let Some(c) = &inst.container {
                // remove container (force to ensure running containers are removed too)
                let out = std::process::Command::new("docker")
                    .args(["rm", "-f", &c.container_id])
                    .output()?;
                if !out.status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    return Err(DbnestError::InvalidArgument(format!(
                        "docker rm failed: {stderr}"
                    )));
                }
            }
        }
        Engine::Mysql => {}
    }

    registry.remove_metadata(id)?;
    Ok(())
}

pub fn apply_schema_to_instance(instance_id: &str, schema_path: &Path) -> Result<()> {
    let registry = Registry::new()?;
    let inst = registry
        .read(instance_id)
        .map_err(|_| DbnestError::InstanceNotFound(instance_id.into()))?;

    let schema = load_schema_auto(schema_path)?;
    validate_schema(&schema)?;

    match inst.engine {
        Engine::Sqlite => {
            let plan = plan_sqlite(&schema);
            apply_sqlite_plan(&inst, &plan)?;
            Ok(())
        }
        Engine::Postgres => {
            let plan = crate::schema::plan::generators::plan_postgres(&schema);
            crate::schema::apply::apply_postgres_plan(&inst, &plan)?;
            Ok(())
        }
        Engine::Mysql => Err(DbnestError::InvalidArgument(
            "apply not implemented for mysql yet".into(),
        )),
    }
}

pub fn plan_schema(engine: Engine, schema_path: &Path) -> Result<crate::schema::plan::SqlPlan> {
    let schema = load_schema_auto(schema_path)?;
    validate_schema(&schema)?;

    Ok(match engine {
        Engine::Sqlite => crate::schema::plan::generators::plan_sqlite(&schema),
        Engine::Postgres => crate::schema::plan::generators::plan_postgres(&schema),
        Engine::Mysql => {
            return Err(DbnestError::InvalidArgument(
                "plan not implemented for mysql yet".into(),
            ));
        }
    })
}

fn load_schema_auto(path: &Path) -> Result<crate::schema::model::Schema> {
    if path.is_dir() {
        load_schema_dir(path)
    } else {
        load_schema_json(path)
    }
}
