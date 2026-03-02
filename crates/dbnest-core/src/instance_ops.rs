use crate::{
    engine::Engine,
    error::{DbnestError, Result},
    instance::{Instance, InstanceSpec, InstanceSummary, Registry},
    sqlite::provision_sqlite,
};

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
    let inst = registry.read(id).map_err(|_| DbnestError::InstanceNotFound(id.into()))?;

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