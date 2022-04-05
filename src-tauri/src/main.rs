use sqlx::{mysql::MySqlPoolOptions, MySql, Pool, Row};

struct Connection {
    pool: Pool<MySql>,
}

#[tokio::main]
async fn main() {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root@localhost:3306")
        .await
        .unwrap();

    tauri::Builder::default()
        .manage(Connection { pool })
        .invoke_handler(tauri::generate_handler![get_schemas])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(sqlx::FromRow)]
struct SchemaQuery {
    Database: String,
}

#[tauri::command]
async fn get_schemas(state: tauri::State<'_, Connection>) -> Result<Vec<String>, String> {
    let schemas = sqlx::query_as::<_, SchemaQuery>("SHOW SCHEMAS")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

    let result: Vec<String> = schemas
        .iter()
        .map(|schema| schema.Database.clone())
        .collect();

    Ok(result)
}
