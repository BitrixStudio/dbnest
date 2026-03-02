use crate::instance::Instance;
use crate::schema::plan::SqlPlan;
use crate::{DbnestError, Result};

pub fn apply_postgres_plan(inst: &Instance, plan: &SqlPlan) -> Result<()> {
    if inst.engine != crate::Engine::Postgres {
        return Err(DbnestError::InvalidArgument(
            "instance is not postgres".into(),
        ));
    }

    let url = inst.connection.database_url.clone();
    let mut client = postgres::Client::connect(&url, postgres::NoTls)
        .map_err(|e| DbnestError::InvalidArgument(format!("postgres connect failed: {e}")))?;

    let mut tx = client
        .transaction()
        .map_err(|e| DbnestError::InvalidArgument(format!("postgres transaction failed: {e}")))?;

    for stmt in &plan.statements {
        tx.batch_execute(stmt).map_err(|e| {
            DbnestError::InvalidArgument(format!("postgres apply failed: {e}\nSQL: {stmt}"))
        })?;
    }

    tx.commit()
        .map_err(|e| DbnestError::InvalidArgument(format!("postgres commit failed: {e}")))?;

    Ok(())
}
