use serde::Serialize;
use crate::database::database::Database;


#[derive(Debug, Serialize)]
pub struct Connector {
    pub provider: Database,
    pub url: String,
}
