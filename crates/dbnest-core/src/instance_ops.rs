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

pub fn provision(spec: InstanceSpec) -> Result<Instance> {
    match spec.engine {
        Engine::Sqlite => provision_sqlite(spec),
        Engine::Postgres | Engine::Mysql => Err(DbnestError::DockerNotAvailable),
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

pub fn stop_instance(_id: &str) -> Result<()> {
    Ok(())
}

pub fn remove_instance(id: &str) -> Result<()> {
    let registry = Registry::new()?;
    let inst = registry
        .read(id)
        .map_err(|_| DbnestError::InstanceNotFound(id.into()))?;

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
        Engine::Postgres | Engine::Mysql => Err(DbnestError::InvalidArgument(
            "apply schema not implemented for this engine yet".into(),
        )),
    }
}

pub fn plan_schema(engine: Engine, schema_path: &Path) -> Result<crate::schema::plan::SqlPlan> {
    let schema = load_schema_auto(schema_path)?;
    validate_schema(&schema)?;

    Ok(match engine {
        Engine::Sqlite => crate::schema::plan::generators::plan_sqlite(&schema),
        Engine::Postgres | Engine::Mysql => {
            return Err(DbnestError::InvalidArgument(
                "plan not implemented for this engine yet".into(),
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
