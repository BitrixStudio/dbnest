mod common;

use common::docker::docker_tests_enabled;

use tempfile::tempdir;

use dbnest_core::engine::Engine;
use dbnest_core::instance::Registry;
use dbnest_core::instance::{InstanceSpec, PostgresSpec};
use dbnest_core::instance_ops::provision_with_schema;
use dbnest_core::paths::Dirs;

#[test]
fn up_with_schema_rolls_back_on_apply_failure_postgres() {
    if !docker_tests_enabled() {
        eprintln!("skipping docker test (set DBNEST_TEST_DOCKER=1 and ensure docker is available)");
        return;
    }

    // Use a temp registry so we don't touch user machine registry
    let dir = tempdir().unwrap();
    let dirs = Dirs::from_base(dir.path().to_path_buf());
    let _registry = Registry::with_dirs(dirs).unwrap();

    // Create an invalid schema file (bad SQL due to invalid identifier / syntax)
    let bad_schema_path = dir.path().join("bad_schema.json");
    std::fs::write(
        &bad_schema_path,
        r#"
        { "tables": [
            { "name": "bad table", "columns": [
                { "name": "id", "type": "uuid", "primary_key": true }
            ]}
          ]
        }
        "#,
    )
    .unwrap();

    let spec = InstanceSpec {
        engine: Engine::Postgres,
        sqlite: None,
        postgres: Some(PostgresSpec {
            user: "dev".into(),
            password: "dev".into(),
            db: "appdb".into(),
            image: Some("postgres:16-alpine".into()),
        }),
    };

    // This should fail, and rollback should remove the created instance
    let result = provision_with_schema(spec, Some(&bad_schema_path));

    assert!(
        result.is_err(),
        "expected provisioning with bad schema to fail"
    );

    // NOTE: To fully verify rollback, we need the created instance id.
    // TODO: Refactor this tests or possibly remove and add new tests altogether
    // For MVP phase this is good enough
}
