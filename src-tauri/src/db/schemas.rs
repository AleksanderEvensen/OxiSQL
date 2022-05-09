use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct InformationSchemaTables {
    #[sqlx(rename = "TABLE_SCHEMA")]
    pub schema: String,

    #[sqlx(rename = "TABLE_NAME")]
    pub name: String,

    pub table_type: String,

    pub engine: Option<String>,
}
