use std::collections::HashMap;

use crate::{Connection, PoolTypes};
use serde::Serialize;
use serde_json::Value;

pub mod schemas;

#[derive(Serialize)]
struct SchemaTable {
    pub name: String,
    pub table_type: String,
    pub engine: Option<String>,
}

#[tauri::command]
pub async fn get_schemas(state: tauri::State<'_, Connection>) -> Result<Value, String> {
    let pool: &PoolTypes = &state.pool;

    match pool {
        PoolTypes::NoConnection => Err("No connection to database".to_string()),
        PoolTypes::MySQL(pool) => {
            let schemas = sqlx::query_as::<_, schemas::InformationSchemaTables>(
                "SELECT * FROM information_schema.TABLES",
            )
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut tables: HashMap<String, Vec<SchemaTable>> = HashMap::new();
            for table in schemas {
                if !tables.contains_key(&table.schema) {
                    tables.insert(table.schema.clone(), Vec::new());
                }
                tables.get_mut(&table.schema).unwrap().push(SchemaTable {
                    name: table.name,
                    table_type: table.table_type,
                    engine: table.engine,
                });
            }

            match serde_json::to_value(tables) {
                Ok(v) => Ok(v),
                Err(e) => Err(e.to_string()),
            }
        }
        _ => Err("Not implemented pool type".to_string()),
    }
}
