use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Database {
    MongoDB,
    MySQL,
    PostgreSQL,
    SQLite,
}

#[derive(Debug, Serialize)]
pub struct Connector {
    pub provider: Database,
    pub url: String,
}
