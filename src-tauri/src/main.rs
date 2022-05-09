use serde_json::{json, Value};
use sqlx::{mssql, MySql, Pool, Postgres, Sqlite};

mod db;

enum PoolTypes {
    NoConnection,
    MySQL(Pool<MySql>),
    Postgres(Pool<Postgres>),
    MsSQL(Pool<mssql::Mssql>),
    Sqlite(Pool<Sqlite>),
}

impl Default for PoolTypes {
    fn default() -> Self {
        PoolTypes::NoConnection
    }
}

#[derive(Default)]
pub struct Connection {
    pool: PoolTypes,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        // .plugin(tauri_plugin_stronghold::TauriStronghold::default())
        .manage(Connection::default())
        .invoke_handler(tauri::generate_handler![db::get_schemas, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_config() -> Result<Value, String> {
    Ok(json!([
        {
            "type": "mysql",
            "id": "mysql-123123",
            "name": "MySQL Connection",
            "ip": "localhost",
            "port": 3306,
            "user": "root",
            "password": "",
            "database": ""
        },
        {
            "type": "postgres",
            "id": "postgres-123123",
            "name": "PostgreSQL Connection",
            "ip": "localhost",
            "port": 5432,
            "user": "postgres",
            "password": "",
            "database": ""
        }
    ]))
}
