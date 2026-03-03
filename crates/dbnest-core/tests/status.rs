mod common;

use tempfile::tempdir;
use time::OffsetDateTime;

use dbnest_core::engine::Engine;
use dbnest_core::instance::InstanceStatus;
use dbnest_core::instance::Registry;
use dbnest_core::instance::{Backend, ConnectionInfo, Instance, SqliteInfo};
use dbnest_core::paths::Dirs;

fn sqlite_url(path: &std::path::Path) -> String {
    format!("sqlite:///{}", path.to_string_lossy().replace('\\', "/"))
}

fn write_instance_with_registry(registry: &Registry, inst: &Instance) {
    registry.write(inst).unwrap();
}

#[test]
fn status_sqlite_running_when_file_exists() {
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let registry = Registry::with_dirs(dirs.clone()).unwrap();

    let db_path = dir.path().join("db.sqlite");
    std::fs::write(&db_path, b"").unwrap();

    let inst = Instance {
        id: "sqlite1".into(),
        engine: Engine::Sqlite,
        backend: Backend::Embedded,
        created_at: OffsetDateTime::now_utc(),
        connection: ConnectionInfo {
            database_url: sqlite_url(&db_path),
            host: None,
            port: None,
            database: None,
            user: None,
        },
        sqlite: Some(SqliteInfo {
            path: db_path.clone(),
            managed: false,
        }),
        container: None,
    };

    write_instance_with_registry(&registry, &inst);

    let report = dbnest_core::instance::status_one_with_registry(&registry, "sqlite1").unwrap();
    assert_eq!(report.status, InstanceStatus::Running);
}

#[test]
fn status_sqlite_missing_when_file_deleted() {
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let registry = Registry::with_dirs(dirs.clone()).unwrap();

    let db_path = dir.path().join("db.sqlite");
    std::fs::write(&db_path, b"").unwrap();
    std::fs::remove_file(&db_path).unwrap();

    let inst = Instance {
        id: "sqlite2".into(),
        engine: Engine::Sqlite,
        backend: Backend::Embedded,
        created_at: OffsetDateTime::now_utc(),
        connection: ConnectionInfo {
            database_url: sqlite_url(&db_path),
            host: None,
            port: None,
            database: None,
            user: None,
        },
        sqlite: Some(SqliteInfo {
            path: db_path.clone(),
            managed: false,
        }),
        container: None,
    };

    write_instance_with_registry(&registry, &inst);

    let report = dbnest_core::instance::status_one_with_registry(&registry, "sqlite2").unwrap();
    assert_eq!(report.status, InstanceStatus::Missing);
}
