use dbnest_core::schema::model::Schema;
use dbnest_core::schema::plan::generators::{plan_postgres, plan_sqlite};
use dbnest_core::schema::validate::validate_schema;

#[test]
fn sqlite_plan_contains_expected_statements() {
    let schema: Schema = serde_json::from_str(
        r#"
        {
          "tables": [
            {
              "name": "users",
              "columns": [
                { "name": "id", "type": "uuid", "primary_key": true },
                { "name": "email", "type": "string", "unique": true, "nullable": false },
                { "name": "created_at", "type": "timestamp", "default": "now" }
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

    let joined = plan.statements.join("\n");
    assert!(
        joined.contains("CREATE TABLE IF NOT EXISTS"),
        "missing CREATE TABLE"
    );
    assert!(
        joined.contains("\"users\""),
        "missing users table identifier"
    );
    assert!(
        joined.contains("\"email\" TEXT"),
        "missing email column mapping to TEXT"
    );
    assert!(
        joined.contains("DEFAULT CURRENT_TIMESTAMP"),
        "missing now->CURRENT_TIMESTAMP mapping"
    );
    assert!(
        joined.contains("CREATE UNIQUE INDEX IF NOT EXISTS"),
        "missing unique index creation"
    );
}

#[test]
fn postgres_plan_contains_expected_types_and_defaults() {
    let schema: Schema = serde_json::from_str(
        r#"
        {
          "tables": [
            {
              "name": "users",
              "columns": [
                { "name": "id", "type": "uuid", "primary_key": true },
                { "name": "created_at", "type": "timestamp", "default": "now" },
                { "name": "active", "type": "bool", "nullable": false }
              ]
            }
          ]
        }
        "#,
    )
    .unwrap();

    validate_schema(&schema).unwrap();
    let plan = plan_postgres(&schema);

    let joined = plan.statements.join("\n");
    assert!(
        joined.contains("UUID"),
        "uuid should map to UUID in postgres"
    );
    assert!(
        joined.contains("TIMESTAMPTZ"),
        "timestamp should map to TIMESTAMPTZ in postgres"
    );
    assert!(
        joined.contains("DEFAULT NOW()"),
        "now should map to NOW() in postgres"
    );
    assert!(
        joined.contains("\"active\" BOOLEAN"),
        "bool should map to BOOLEAN in postgres"
    );
}
