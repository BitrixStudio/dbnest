use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlPlan {
    pub statements: Vec<String>,
}
