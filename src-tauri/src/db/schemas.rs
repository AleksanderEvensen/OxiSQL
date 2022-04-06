use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct InformationSchemaTables {
    pub schema: String,
    pub name: String,
    pub table_type: String,
    pub engine: Option<String>,
}
