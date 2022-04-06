use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

mod db;

pub struct Connection {
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
        .invoke_handler(tauri::generate_handler![db::get_schemas])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
