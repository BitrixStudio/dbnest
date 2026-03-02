use crate::schema::model::{LogicalType, Schema};
use crate::schema::plan::SqlPlan;

pub fn plan_postgres(schema: &Schema) -> SqlPlan {
    let mut stmts = Vec::new();

    for table in &schema.tables {
        let mut cols_sql = Vec::new();

        for col in &table.columns {
            let mut s = String::new();
            s.push_str(&quote_ident(&col.name));
            s.push(' ');
            s.push_str(map_type(&col.col_type));

            if col.primary_key {
                s.push_str(" PRIMARY KEY");
            }

            if col.unique && !col.primary_key {
                s.push_str(" UNIQUE");
            }

            if !col.nullable && !col.primary_key {
                s.push_str(" NOT NULL");
            }

            if let Some(def) = &col.default {
                s.push_str(" DEFAULT ");
                s.push_str(&map_default(def));
            }

            cols_sql.push(s);
        }

        let create = format!(
            "CREATE TABLE IF NOT EXISTS {} (\n  {}\n);",
            quote_ident(&table.name),
            cols_sql.join(",\n  ")
        );
        stmts.push(create);

        for idx in &table.indexes {
            let cols = idx
                .columns
                .iter()
                .map(|c| quote_ident(c))
                .collect::<Vec<_>>()
                .join(", ");
            let unique = if idx.unique { "UNIQUE " } else { "" };
            stmts.push(format!(
                "CREATE {unique}INDEX IF NOT EXISTS {} ON {} ({});",
                quote_ident(&idx.name),
                quote_ident(&table.name),
                cols
            ));
        }
    }

    SqlPlan { statements: stmts }
}

fn map_type(t: &LogicalType) -> &'static str {
    match t {
        LogicalType::String => "TEXT",
        LogicalType::Int64 => "BIGINT",
        LogicalType::Bool => "BOOLEAN",
        LogicalType::Uuid => "UUID",
        LogicalType::Timestamp => "TIMESTAMPTZ",
    }
}

fn map_default(def: &str) -> String {
    match def {
        "now" => "NOW()".to_string(),
        other => other.to_string(),
    }
}

fn quote_ident(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}
