use tempfile::tempdir;
use time::OffsetDateTime;

use dbnest_core::engine::Engine;
use dbnest_core::instance::Registry;
use dbnest_core::instance::{Backend, ConnectionInfo, Instance, SqliteInfo};
use dbnest_core::paths::Dirs;

fn make_sqlite_instance(
    id: &str,
    created_at: OffsetDateTime,
    db_path: std::path::PathBuf,
) -> Instance {
    Instance {
        id: id.to_string(),
        engine: Engine::Sqlite,
        backend: Backend::Embedded,
        created_at,
        connection: ConnectionInfo {
            database_url: format!("sqlite:///{}", db_path.to_string_lossy().replace('\\', "/")),
            host: None,
            port: None,
            database: None,
            user: None,
        },
        sqlite: Some(SqliteInfo {
            path: db_path,
            managed: true,
        }),
        container: None,
    }
}

#[test]
fn registry_write_and_read_roundtrip() {
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let registry = Registry::with_dirs(dirs).unwrap();

    let now = OffsetDateTime::now_utc();
    let db_path = dir.path().join("sqlite").join("a").join("database.sqlite");
    let inst = make_sqlite_instance("a1b2c3d4", now, db_path);

    registry.write(&inst).unwrap();

    let read = registry.read("a1b2c3d4").unwrap();
    assert_eq!(read.id, inst.id);
    assert_eq!(read.engine, inst.engine);
    assert_eq!(read.backend as u8, inst.backend as u8);
    assert_eq!(read.connection.database_url, inst.connection.database_url);
    assert!(read.sqlite.is_some());
}

#[test]
fn registry_list_returns_all_instances() {
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let registry = Registry::with_dirs(dirs).unwrap();

    let t1 = OffsetDateTime::now_utc() - time::Duration::seconds(10);
    let t2 = OffsetDateTime::now_utc();

    let inst1 = make_sqlite_instance("old11111", t1, dir.path().join("db1.sqlite"));
    let inst2 = make_sqlite_instance("new22222", t2, dir.path().join("db2.sqlite"));

    registry.write(&inst1).unwrap();
    registry.write(&inst2).unwrap();

    let list = registry.list().unwrap();
    assert_eq!(list.len(), 2);

    let ids = list.iter().map(|i| i.id.as_str()).collect::<Vec<_>>();
    assert!(ids.contains(&"old11111"));
    assert!(ids.contains(&"new22222"));
}

#[test]
fn registry_list_is_sorted_newest_first() {
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let registry = Registry::with_dirs(dirs).unwrap();

    let older = OffsetDateTime::now_utc() - time::Duration::seconds(60);
    let newer = OffsetDateTime::now_utc();

    let inst_old = make_sqlite_instance("old00001", older, dir.path().join("old.sqlite"));
    let inst_new = make_sqlite_instance("new00002", newer, dir.path().join("new.sqlite"));

    registry.write(&inst_old).unwrap();
    registry.write(&inst_new).unwrap();

    let list = registry.list().unwrap();
    assert_eq!(list.len(), 2);

    assert_eq!(list[0].id, "new00002");
    assert_eq!(list[1].id, "old00001");
}

#[test]
fn registry_remove_metadata_deletes_instance_file() {
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let registry = Registry::with_dirs(dirs.clone()).unwrap();

    let inst = make_sqlite_instance(
        "deadbeef",
        OffsetDateTime::now_utc(),
        dir.path().join("dev.sqlite"),
    );

    registry.write(&inst).unwrap();

    let path = dirs.instance_file("deadbeef");
    assert!(path.exists(), "expected instance file to exist");

    registry.remove_metadata("deadbeef").unwrap();
    assert!(!path.exists(), "expected instance file to be removed");
}

#[test]
fn registry_list_ignores_corrupted_json_files() {
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let registry = Registry::with_dirs(dirs.clone()).unwrap();

    // Write one valid instance
    let inst = make_sqlite_instance(
        "good1234",
        OffsetDateTime::now_utc(),
        dir.path().join("ok.sqlite"),
    );
    registry.write(&inst).unwrap();

    // Write a corrupted json file next to it
    std::fs::write(dirs.instances.join("corrupt.json"), b"{ not valid json").unwrap();

    let list = registry.list().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "good1234");
}
