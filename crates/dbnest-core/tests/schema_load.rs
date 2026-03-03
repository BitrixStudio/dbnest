use std::fs;
use std::path::Path;

use tempfile::tempdir;

use dbnest_core::schema::load::{load_schema_dir, load_schema_json};
use dbnest_core::schema::validate::validate_schema;

fn write_file(path: &Path, contents: &str) {
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).unwrap();
    }
    fs::write(path, contents).unwrap();
}

#[test]
fn load_schema_from_json_file() {
    let dir = tempdir().unwrap();
    let schema_path = dir.path().join("schema.json");

    let schema_json = r#"
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
    "#;

    write_file(&schema_path, schema_json);

    let schema = load_schema_json(&schema_path).unwrap();
    validate_schema(&schema).unwrap();

    assert_eq!(schema.tables.len(), 1);
    assert_eq!(schema.tables[0].name, "users");
    assert_eq!(schema.tables[0].columns.len(), 3);
    assert_eq!(schema.tables[0].indexes.len(), 1);
}

#[test]
fn load_schema_from_directory_layout() {
    let dir = tempdir().unwrap();
    let schema_dir = dir.path().join("schema");

    let users_cols = r#"
    [
      { "name": "id", "type": "uuid", "primary_key": true },
      { "name": "email", "type": "string", "unique": true, "nullable": false }
    ]
    "#;

    let users_indexes = r#"
    [
      { "name": "idx_users_email", "columns": ["email"], "unique": true }
    ]
    "#;

    write_file(&schema_dir.join("users").join("columns.json"), users_cols);
    write_file(
        &schema_dir.join("users").join("indexes.json"),
        users_indexes,
    );

    // Add another table to ensure multi-table works
    let projects_cols = r#"
    [
      { "name": "id", "type": "uuid", "primary_key": true },
      { "name": "name", "type": "string", "nullable": false }
    ]
    "#;
    write_file(
        &schema_dir.join("projects").join("columns.json"),
        projects_cols,
    );

    let schema = load_schema_dir(&schema_dir).unwrap();
    validate_schema(&schema).unwrap();

    assert_eq!(schema.tables.len(), 2);

    let mut names = schema
        .tables
        .iter()
        .map(|t| t.name.as_str())
        .collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, vec!["projects", "users"]);
}
