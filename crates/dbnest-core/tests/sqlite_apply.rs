use tempfile::tempdir;

use dbnest_core::engine::Engine;
use dbnest_core::instance::{Backend, ConnectionInfo, Instance, SqliteInfo};

use dbnest_core::schema::apply::apply_sqlite_plan;
use dbnest_core::schema::model::Schema;
use dbnest_core::schema::plan::generators::plan_sqlite;
use dbnest_core::schema::validate::validate_schema;

use time::OffsetDateTime;

fn sqlite_url(path: &std::path::Path) -> String {
    // Consistent URL string across platforms
    let p = path.to_string_lossy().replace('\\', "/");
    format!("sqlite:///{}", p)
}

#[test]
fn apply_schema_creates_tables_in_sqlite() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("dev.sqlite");

    let schema: Schema = serde_json::from_str(
        r#"
        {
          "tables": [
            {
              "name": "users",
              "columns": [
                { "name": "id", "type": "uuid", "primary_key": true },
                { "name": "email", "type": "string", "unique": true, "nullable": false }
              ],
              "indexes": [
                { "name": "idx_users_email", "columns": ["email"], "unique": true }
              ]
            }
          ]
        }
        "#,
    )
    .unwrap();

    validate_schema(&schema).unwrap();
    let plan = plan_sqlite(&schema);

    // Create an Instance object pointing at our temp sqlite file
    // This runs the same executor used in production
    let inst = Instance {
        id: "test-sqlite".into(),
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

    apply_sqlite_plan(&inst, &plan).unwrap();

    // Verify tables exist using rusqlite directly
    let conn = rusqlite::Connection::open(&db_path).unwrap();

    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='users';")
        .unwrap();

    let mut rows = stmt.query([]).unwrap();
    let row = rows.next().unwrap();
    assert!(row.is_some(), "expected table 'users' to exist");

    // Verify index exists
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name='idx_users_email';")
        .unwrap();
    let mut rows = stmt.query([]).unwrap();
    let row = rows.next().unwrap();
    assert!(row.is_some(), "expected index 'idx_users_email' to exist");
}
