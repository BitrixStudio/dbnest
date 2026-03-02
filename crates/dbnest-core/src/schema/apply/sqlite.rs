use crate::{DbnestError, Result};
use crate::instance::Instance;
use crate::schema::plan::SqlPlan;

pub fn apply_sqlite_plan(inst: &Instance, plan: &SqlPlan) -> Result<()> {
    let sqlite = inst
        .sqlite
        .as_ref()
        .ok_or_else(|| DbnestError::InvalidArgument("instance is not sqlite".into()))?;

    let mut conn = rusqlite::Connection::open(&sqlite.path)
        .map_err(|e| DbnestError::InvalidArgument(format!("failed to open sqlite db: {e}")))?;

    let tx = conn
        .transaction()
        .map_err(|e| DbnestError::InvalidArgument(format!("sqlite transaction failed: {e}")))?;

    for stmt in &plan.statements {
        tx.execute_batch(stmt)
            .map_err(|e| DbnestError::InvalidArgument(format!("sqlite apply failed: {e}\nSQL: {stmt}")))?;
    }

    tx.commit()
        .map_err(|e| DbnestError::InvalidArgument(format!("sqlite commit failed: {e}")))?;

    Ok(())
}