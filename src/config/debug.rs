use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Debug {
    #[serde(rename = "logQueries")]
    pub log_queries: bool,
    #[serde(rename = "logMigrations")]
    pub log_migrations: bool,
    #[serde(rename = "logSeedRecords")]
    pub log_seed_records: bool,
}
