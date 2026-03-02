use crate::schema::model::{Column, Index, Schema, Table};
use crate::{DbnestError, Result};
use std::path::{Path, PathBuf};

pub fn load_schema_dir(dir: impl AsRef<Path>) -> Result<Schema> {
    let dir = dir.as_ref();
    if !dir.is_dir() {
        return Err(DbnestError::InvalidArgument(format!(
            "schema directory does not exist: {}",
            dir.display()
        )));
    }

    let mut tables = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let table_dir = entry.path();
        if !table_dir.is_dir() {
            continue;
        }

        let table_name = entry.file_name().to_string_lossy().to_string();
        let columns_path = table_dir.join("columns.json");
        if !columns_path.exists() {
            return Err(DbnestError::InvalidArgument(format!(
                "missing columns.json for table '{}': {}",
                table_name,
                columns_path.display()
            )));
        }

        let columns: Vec<Column> = read_json(&columns_path)?;

        let indexes_path = table_dir.join("indexes.json");
        let indexes: Vec<Index> = if indexes_path.exists() {
            read_json(&indexes_path)?
        } else {
            vec![]
        };

        tables.push(Table {
            name: table_name,
            columns,
            indexes,
        });
    }

    Ok(Schema { tables })
}

fn read_json<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let bytes = std::fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}