use std::collections::HashMap;

use crate::Connection;
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
    let schemas =
        sqlx::query_as::<_, schemas::InformationSchemaTables>("SELECT TABLE_SCHEMA AS 'schema', TABLE_NAME AS 'name', TABLE_TYPE AS 'table_type', ENGINE AS 'engine' FROM information_schema.TABLES")
            .fetch_all(&state.pool)
            .await
            .map_err(|e| e.to_string())?;

    let mut data: HashMap<String, Vec<SchemaTable>> = HashMap::new();
    for table in schemas {
        if !data.contains_key(&table.schema) {
            data.insert(table.schema.clone(), vec![]);
        }
        data.get_mut(&table.schema).unwrap().push(SchemaTable {
            name: table.name,
            table_type: table.table_type,
            engine: table.engine,
        });
    }

    match serde_json::to_value(data) {
        Ok(v) => Ok(v),
        Err(e) => Err(e.to_string()),
    }
}
