use crate::{
    engine::Engine,
    error::{DbnestError, Result},
    ids::new_instance_id,
    instance::Registry,
    instance::{Backend, Instance, InstanceSpec, SqliteInfo},
};
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

pub fn provision_sqlite(spec: InstanceSpec) -> Result<Instance> {
    if spec.engine != Engine::Sqlite {
        return Err(DbnestError::InvalidArgument(
            "provision_sqlite called with non-sqlite engine".into(),
        ));
    }

    let registry = Registry::new()?;
    let id = new_instance_id();
    let sqlite_spec = spec
        .sqlite
        .unwrap_or(crate::instance::SqliteSpec { path: None });

    let (path, managed) =
        resolve_sqlite_path(registry.dirs().managed_sqlite_file(&id), sqlite_spec.path)?;
    ensure_parent_dir(&path)?;

    if !path.exists() {
        std::fs::File::create(&path)?;
    }

    let db_url = to_sqlite_url(&path);

    let inst = Instance {
        id: id.clone(),
        engine: Engine::Sqlite,
        backend: Backend::Embedded,
        created_at: OffsetDateTime::now_utc(),
        connection: crate::ConnectionInfo {
            database_url: db_url,
            host: None,
            port: None,
            database: None,
            user: None,
        },
        sqlite: Some(SqliteInfo { path, managed }),
        container: None,
    };

    registry.write(&inst)?;
    Ok(inst)
}

fn resolve_sqlite_path(
    managed_default: PathBuf,
    user_path: Option<PathBuf>,
) -> Result<(PathBuf, bool)> {
    match user_path {
        None => Ok((managed_default, true)),
        Some(p) => {
            let abs = if p.is_absolute() {
                p
            } else {
                std::env::current_dir()?.join(p)
            };
            Ok((abs, false))
        }
    }
}

fn ensure_parent_dir(p: &Path) -> Result<()> {
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn to_sqlite_url(path: &Path) -> String {
    let abs = path;
    format!("sqlite:///{}", abs.to_string_lossy().replace('\\', "/"))
}
