use crate::schema::model::{Schema, Table};
use crate::{DbnestError, Result};
use std::collections::HashSet;

pub fn validate_schema(schema: &Schema) -> Result<()> {
    if schema.tables.is_empty() {
        return Err(DbnestError::InvalidArgument("schema has no tables".into()));
    }

    let mut table_names = HashSet::new();
    for t in &schema.tables {
        validate_table(t)?;
        if !table_names.insert(t.name.clone()) {
            return Err(DbnestError::InvalidArgument(format!(
                "duplicate table name '{}'",
                t.name
            )));
        }
    }

    Ok(())
}

fn validate_table(t: &Table) -> Result<()> {
    if t.columns.is_empty() {
        return Err(DbnestError::InvalidArgument(format!(
            "table '{}' has no columns",
            t.name
        )));
    }

    let mut col_names = HashSet::new();
    let mut pk_count = 0usize;

    for c in &t.columns {
        if !col_names.insert(c.name.clone()) {
            return Err(DbnestError::InvalidArgument(format!(
                "duplicate column '{}' in table '{}'",
                c.name, t.name
            )));
        }
        if c.primary_key {
            pk_count += 1;
        }
    }

    if pk_count > 1 {
        return Err(DbnestError::InvalidArgument(format!(
            "table '{}' has multiple primary keys (v1 supports only single-column PK)",
            t.name
        )));
    }

    for idx in &t.indexes {
        if idx.columns.is_empty() {
            return Err(DbnestError::InvalidArgument(format!(
                "index '{}' on table '{}' has no columns",
                idx.name, t.name
            )));
        }
        for col in &idx.columns {
            if !col_names.contains(col) {
                return Err(DbnestError::InvalidArgument(format!(
                    "index '{}' on table '{}' references unknown column '{}'",
                    idx.name, t.name, col
                )));
            }
        }
    }

    Ok(())
}