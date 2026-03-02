use crate::Result;
use crate::schema::model::Schema;
use std::path::Path;

pub fn load_schema_json(path: impl AsRef<Path>) -> Result<Schema> {
    let bytes = std::fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}
